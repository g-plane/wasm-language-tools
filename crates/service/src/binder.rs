use crate::{
    idx::{IdentsCtx, Idx, IdxGen},
    syntax_tree::SyntaxTreeCtx,
    uri::InternUri,
};
use rowan::{
    ast::{support, AstNode},
    GreenNode, TextRange,
};
use std::{hash::Hash, sync::Arc};
use wat_syntax::{
    ast::{CompType, ModuleFieldFunc, PlainInstr, SubType},
    SyntaxKind, SyntaxNode, SyntaxNodePtr,
};

#[salsa::query_group(SymbolTables)]
pub(crate) trait SymbolTablesCtx: SyntaxTreeCtx + IdentsCtx {
    #[salsa::memoized]
    #[salsa::invoke(create_symbol_table)]
    fn symbol_table(&self, uri: InternUri) -> Arc<SymbolTable>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SymbolTable {
    pub symbols: Vec<Symbol>,
    pub blocks: Vec<BlockItem>,
    pub exports: Vec<ExportItem>,
}
fn create_symbol_table(db: &dyn SymbolTablesCtx, uri: InternUri) -> Arc<SymbolTable> {
    fn create_module_level_symbol(
        db: &dyn SymbolTablesCtx,
        node: SyntaxNode,
        id: u32,
        kind: SymbolKind,
        module: &SyntaxNode,
    ) -> Symbol {
        Symbol {
            key: SymbolKey::new(&node),
            green: node.green().into(),
            region: SymbolKey::new(module),
            kind,
            idx: Idx {
                num: Some(id),
                name: support::token(&node, SyntaxKind::IDENT)
                    .map(|token| db.ident(token.text().into())),
            },
        }
    }
    fn create_ref_symbol(
        db: &dyn SymbolTablesCtx,
        node: &SyntaxNode,
        region: SymbolKey,
        kind: SymbolKind,
    ) -> Option<Symbol> {
        support::token(node, SyntaxKind::IDENT)
            .map(|ident| Symbol {
                key: SymbolKey::new(node),
                green: node.green().into(),
                region,
                kind,
                idx: Idx {
                    num: None,
                    name: Some(db.ident(ident.text().into())),
                },
            })
            .or_else(|| {
                node.children_with_tokens()
                    .filter_map(|it| it.into_token())
                    .find(|it| matches!(it.kind(), SyntaxKind::UNSIGNED_INT | SyntaxKind::INT))
                    .and_then(|token| token.text().parse().ok())
                    .map(|num| Symbol {
                        green: node.green().into(),
                        key: SymbolKey::new(node),
                        region,
                        kind,
                        idx: Idx {
                            num: Some(num),
                            name: None,
                        },
                    })
            })
    }
    fn create_first_optional_ref_symbol(
        db: &dyn SymbolTablesCtx,
        instr: PlainInstr,
        kind: SymbolKind,
    ) -> Option<Symbol> {
        let node = instr.syntax();
        node.ancestors()
            .find(|node| node.kind() == SyntaxKind::MODULE)
            .map(|region| {
                let region = SymbolKey::new(&region);
                instr
                    .immediates()
                    .next()
                    .and_then(|immediate| create_ref_symbol(db, immediate.syntax(), region, kind))
                    .unwrap_or_else(|| Symbol {
                        green: node.green().into(),
                        key: SymbolKey::new(node),
                        region,
                        kind,
                        idx: Idx {
                            num: Some(0),
                            name: None,
                        },
                    })
            })
    }
    fn create_import_desc_symbol(
        db: &dyn SymbolTablesCtx,
        node: &SyntaxNode,
        id: u32,
        kind: SymbolKind,
        module: &SyntaxNode,
    ) -> Symbol {
        Symbol {
            key: SymbolKey::new(node),
            green: node.green().into(),
            region: SymbolKey::new(module),
            kind,
            idx: Idx {
                num: Some(id),
                name: support::token(node, SyntaxKind::IDENT)
                    .map(|token| db.ident(token.text().into())),
            },
        }
    }

    let root = SyntaxNode::new_root(db.root(uri));
    let mut symbols = Vec::with_capacity(2);
    let mut blocks = vec![];
    let mut exports = vec![];
    root.children().enumerate().for_each(|(module_id, module)| {
        symbols.push(Symbol {
            green: module.green().into(),
            key: SymbolKey::new(&module),
            region: SymbolKey::new(&root),
            kind: SymbolKind::Module,
            idx: Idx {
                num: Some(module_id as u32),
                name: None,
            },
        });
        let mut func_idx_gen = IdxGen::default();
        let mut type_idx_gen = IdxGen::default();
        let mut global_idx_gen = IdxGen::default();
        let mut mem_idx_gen = IdxGen::default();
        let mut table_idx_gen = IdxGen::default();
        module.descendants().for_each(|node| match node.kind() {
            SyntaxKind::MODULE_FIELD_FUNC => {
                symbols.push(create_module_level_symbol(
                    db,
                    node.clone(),
                    func_idx_gen.pull(),
                    SymbolKind::Func,
                    &module,
                ));
                let Some(func) = ModuleFieldFunc::cast(node.clone()) else {
                    return;
                };
                let mut local_idx_gen = IdxGen::default();
                func.type_use()
                    .iter()
                    .flat_map(|type_use| type_use.params())
                    .for_each(|param| {
                        if let Some(ident) = param.ident_token() {
                            let param = param.syntax();
                            symbols.push(Symbol {
                                key: SymbolKey::new(param),
                                green: param.green().into(),
                                region: SymbolKey::new(&node),
                                kind: SymbolKind::Param,
                                idx: Idx {
                                    num: Some(local_idx_gen.pull()),
                                    name: Some(db.ident(ident.text().into())),
                                },
                            });
                        } else {
                            symbols.extend(param.val_types().map(|val_type| {
                                let val_type = val_type.syntax();
                                Symbol {
                                    key: SymbolKey::new(val_type),
                                    green: val_type.green().into(),
                                    region: SymbolKey::new(&node),
                                    kind: SymbolKind::Param,
                                    idx: Idx {
                                        num: Some(local_idx_gen.pull()),
                                        name: None,
                                    },
                                }
                            }));
                        }
                    });
                func.locals().for_each(|local| {
                    if let Some(ident) = local.ident_token() {
                        let local = local.syntax();
                        symbols.push(Symbol {
                            key: SymbolKey::new(local),
                            green: local.green().into(),
                            region: SymbolKey::new(&node),
                            kind: SymbolKind::Local,
                            idx: Idx {
                                num: Some(local_idx_gen.pull()),
                                name: Some(db.ident(ident.text().into())),
                            },
                        });
                    } else {
                        symbols.extend(local.val_types().map(|val_type| {
                            let val_type = val_type.syntax();
                            Symbol {
                                key: SymbolKey::new(val_type),
                                green: val_type.green().into(),
                                region: SymbolKey::new(&node),
                                kind: SymbolKind::Local,
                                idx: Idx {
                                    num: Some(local_idx_gen.pull()),
                                    name: None,
                                },
                            }
                        }));
                    }
                });
            }
            SyntaxKind::TYPE_DEF => {
                symbols.push(create_module_level_symbol(
                    db,
                    node.clone(),
                    type_idx_gen.pull(),
                    SymbolKind::Type,
                    &module,
                ));
                if let Some(CompType::Struct(s)) =
                    support::child::<SubType>(&node).and_then(|sub_type| sub_type.comp_type())
                {
                    let mut field_idx_gen = IdxGen::default();
                    s.fields().for_each(|field| {
                        if let Some(ident) = field.ident_token() {
                            let field = field.syntax();
                            symbols.push(Symbol {
                                key: SymbolKey::new(field),
                                green: field.green().into(),
                                region: SymbolKey::new(&node),
                                kind: SymbolKind::FieldDef,
                                idx: Idx {
                                    num: Some(field_idx_gen.pull()),
                                    name: Some(db.ident(ident.text().into())),
                                },
                            });
                        } else {
                            symbols.extend(field.field_types().map(|field_type| {
                                let field_type = field_type.syntax();
                                Symbol {
                                    key: SymbolKey::new(field_type),
                                    green: field_type.green().into(),
                                    region: SymbolKey::new(&node),
                                    kind: SymbolKind::FieldDef,
                                    idx: Idx {
                                        num: Some(field_idx_gen.pull()),
                                        name: None,
                                    },
                                }
                            }));
                        }
                    });
                }
            }
            SyntaxKind::MODULE_FIELD_GLOBAL => {
                symbols.push(create_module_level_symbol(
                    db,
                    node.clone(),
                    global_idx_gen.pull(),
                    SymbolKind::GlobalDef,
                    &module,
                ));
            }
            SyntaxKind::PLAIN_INSTR => {
                let Some(instr) = PlainInstr::cast(node) else {
                    return;
                };
                let node = instr.syntax();
                match instr.instr_name().as_ref().map(|token| token.text()) {
                    Some("call" | "ref.func" | "return_call") => {
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, &node, SymbolKey::new(&module), SymbolKind::Call)
                        }));
                    }
                    Some("local.get" | "local.set" | "local.tee") => {
                        let Some(region) = node
                            .ancestors()
                            .find(|node| node.kind() == SyntaxKind::MODULE_FIELD_FUNC)
                            .map(|node| SymbolKey::new(&node))
                        else {
                            return;
                        };
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, &node, region, SymbolKind::LocalRef)
                        }));
                    }
                    Some("global.get" | "global.set") => {
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(
                                db,
                                &node,
                                SymbolKey::new(&module),
                                SymbolKind::GlobalRef,
                            )
                        }));
                    }
                    Some("br" | "br_if" | "br_table" | "br_on_null" | "br_on_non_null") => {
                        let Some(region) = node
                            .ancestors()
                            .find(|node| {
                                matches!(
                                    node.kind(),
                                    SyntaxKind::BLOCK_BLOCK
                                        | SyntaxKind::BLOCK_LOOP
                                        | SyntaxKind::BLOCK_IF
                                )
                            })
                            .map(|node| SymbolKey::new(&node))
                        else {
                            return;
                        };
                        node.children()
                            .filter_map(|node| {
                                create_ref_symbol(db, &node, region, SymbolKind::BlockRef)
                            })
                            .for_each(|symbol| {
                                let mut current = &symbol;
                                let mut levels = 0;
                                while let Some(
                                    parent @ Symbol {
                                        key,
                                        kind: SymbolKind::BlockDef,
                                        ..
                                    },
                                ) = symbols.iter().find(|sym| {
                                    sym.kind == SymbolKind::BlockDef && sym.key == current.region
                                }) {
                                    let mut idx = parent.idx;
                                    idx.num = Some(levels);
                                    if symbol.idx.is_defined_by(&idx) {
                                        blocks.push(BlockItem {
                                            ref_key: symbol.key,
                                            def_key: *key,
                                            def_idx: idx,
                                        });
                                    }
                                    current = parent;
                                    levels += 1;
                                }
                                symbols.push(symbol);
                            });
                    }
                    Some("call_indirect" | "return_call_indirect") => {
                        if let Some(symbol) =
                            create_first_optional_ref_symbol(db, instr, SymbolKind::TableRef)
                        {
                            symbols.push(symbol);
                        }
                    }
                    Some(
                        "table.get" | "table.set" | "table.size" | "table.grow" | "table.fill"
                        | "table.copy",
                    ) => {
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(
                                db,
                                &node,
                                SymbolKey::new(&module),
                                SymbolKind::TableRef,
                            )
                        }));
                    }
                    Some("table.init") => {
                        if let Some(symbol) = node.children().next().and_then(|node| {
                            create_ref_symbol(
                                db,
                                &node,
                                SymbolKey::new(&module),
                                SymbolKind::TableRef,
                            )
                        }) {
                            symbols.push(symbol);
                        }
                    }
                    Some(
                        "memory.size" | "memory.grow" | "memory.fill" | "memory.init" | "i32.load"
                        | "i64.load" | "f32.load" | "f64.load" | "i32.load8_s" | "i32.load8_u"
                        | "i32.load16_s" | "i32.load16_u" | "i64.load8_s" | "i64.load8_u"
                        | "i64.load16_s" | "i64.load16_u" | "i64.load32_s" | "i64.load32_u"
                        | "i32.store" | "i64.store" | "f32.store" | "f64.store" | "i32.store8"
                        | "i32.store16" | "i64.store8" | "i64.store16" | "i64.store32"
                        | "v128.load" | "v128.load8x8_s" | "v128.load8x8_u" | "v128.load16x4_s"
                        | "v128.load16x4_u" | "v128.load32x2_s" | "v128.load32x2_u"
                        | "v128.load8_splat" | "v128.load16_splat" | "v128.load32_splat"
                        | "v128.load64_splat" | "v128.load32_zero" | "v128.load64_zero"
                        | "v128.store" | "v128.load8_lane" | "v128.load16_lane"
                        | "v128.load32_lane" | "v128.load64_lane" | "v128.store8_lane"
                        | "v128.store16_lane" | "v128.store32_lane" | "v128.store64_lane",
                    ) => {
                        if let Some(symbol) =
                            create_first_optional_ref_symbol(db, instr, SymbolKind::MemoryRef)
                        {
                            symbols.push(symbol);
                        }
                    }
                    Some("memory.copy") => {
                        if let Some(symbol) = node.children().next().and_then(|node| {
                            create_ref_symbol(
                                db,
                                &node,
                                SymbolKey::new(&module),
                                SymbolKind::MemoryRef,
                            )
                        }) {
                            symbols.push(symbol);
                        }
                    }
                    Some(
                        "struct.new" | "struct.new_default" | "array.new" | "array.new_default"
                        | "array.new_fixed" | "array.new_data" | "array.new_elem" | "array.get"
                        | "array.get_u" | "array.get_s" | "array.set" | "array.fill" | "call_ref"
                        | "return_call_ref",
                    ) => {
                        if let Some(symbol) = node.children().next().and_then(|node| {
                            create_ref_symbol(
                                db,
                                &node,
                                SymbolKey::new(&module),
                                SymbolKind::TypeUse,
                            )
                        }) {
                            symbols.push(symbol);
                        }
                    }
                    Some("array.copy") => {
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(
                                db,
                                &node,
                                SymbolKey::new(&module),
                                SymbolKind::TypeUse,
                            )
                        }));
                    }
                    Some("struct.get" | "struct.get_s" | "struct.get_u" | "struct.set") => {
                        let mut children = node.children();
                        if let Some(symbol) = children.next().and_then(|node| {
                            create_ref_symbol(
                                db,
                                &node,
                                SymbolKey::new(&module),
                                SymbolKind::TypeUse,
                            )
                        }) {
                            let key = symbol.key;
                            symbols.push(symbol);
                            if let Some(symbol) = children.next().and_then(|node| {
                                // The region here is temporary.
                                // It's used for tracking which struct it belongs to,
                                // and it will be replaced with the actual region later.
                                // If the struct it belongs to isn't defined, nothing will happen.
                                create_ref_symbol(db, &node, key, SymbolKind::FieldRef)
                            }) {
                                symbols.push(symbol);
                            }
                        }
                    }
                    _ => {}
                }
            }
            SyntaxKind::BLOCK_BLOCK | SyntaxKind::BLOCK_IF | SyntaxKind::BLOCK_LOOP => {
                if let Some(symbol) = node.parent().map(|parent| Symbol {
                    key: SymbolKey::new(&node),
                    green: node.green().into(),
                    region: SymbolKey::new(&parent),
                    kind: SymbolKind::BlockDef,
                    idx: Idx {
                        num: Some(0), // fake ID
                        name: support::token(&node, SyntaxKind::IDENT)
                            .map(|token| db.ident(token.text().into())),
                    },
                }) {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::MODULE_FIELD_START | SyntaxKind::EXPORT_DESC_FUNC => {
                if let Some(symbol) = node
                    .children()
                    .find(|child| child.kind() == SyntaxKind::INDEX)
                    .and_then(|index| {
                        create_ref_symbol(db, &index, SymbolKey::new(&module), SymbolKind::Call)
                    })
                {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::TYPE_USE | SyntaxKind::HEAP_TYPE | SyntaxKind::SUB_TYPE => {
                if let Some(symbol) = node
                    .children()
                    .find(|child| child.kind() == SyntaxKind::INDEX)
                    .and_then(|index| {
                        create_ref_symbol(db, &index, SymbolKey::new(&module), SymbolKind::TypeUse)
                    })
                {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::MODULE_FIELD_MEMORY => {
                symbols.push(create_module_level_symbol(
                    db,
                    node.clone(),
                    mem_idx_gen.pull(),
                    SymbolKind::MemoryDef,
                    &module,
                ));
            }
            SyntaxKind::MODULE_FIELD_TABLE => {
                symbols.push(create_module_level_symbol(
                    db,
                    node.clone(),
                    table_idx_gen.pull(),
                    SymbolKind::TableDef,
                    &module,
                ));
            }
            SyntaxKind::EXPORT_DESC_GLOBAL => {
                if let Some(symbol) = node
                    .children()
                    .find(|child| child.kind() == SyntaxKind::INDEX)
                    .and_then(|index| {
                        create_ref_symbol(
                            db,
                            &index,
                            SymbolKey::new(&module),
                            SymbolKind::GlobalRef,
                        )
                    })
                {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::EXPORT_DESC_MEMORY => {
                if let Some(symbol) = node
                    .children()
                    .find(|child| child.kind() == SyntaxKind::INDEX)
                    .and_then(|index| {
                        create_ref_symbol(
                            db,
                            &index,
                            SymbolKey::new(&module),
                            SymbolKind::MemoryRef,
                        )
                    })
                {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::EXPORT_DESC_TABLE => {
                if let Some(symbol) = node
                    .children()
                    .find(|child| child.kind() == SyntaxKind::INDEX)
                    .and_then(|index| {
                        create_ref_symbol(db, &index, SymbolKey::new(&module), SymbolKind::TableRef)
                    })
                {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::IMPORT_DESC_TYPE_USE => {
                symbols.push(create_import_desc_symbol(
                    db,
                    &node,
                    func_idx_gen.pull(),
                    SymbolKind::Func,
                    &module,
                ));
            }
            SyntaxKind::IMPORT_DESC_TABLE_TYPE => {
                symbols.push(create_import_desc_symbol(
                    db,
                    &node,
                    table_idx_gen.pull(),
                    SymbolKind::TableDef,
                    &module,
                ));
            }
            SyntaxKind::IMPORT_DESC_MEMORY_TYPE => {
                symbols.push(create_import_desc_symbol(
                    db,
                    &node,
                    mem_idx_gen.pull(),
                    SymbolKind::MemoryDef,
                    &module,
                ));
            }
            SyntaxKind::IMPORT_DESC_GLOBAL_TYPE => {
                symbols.push(create_import_desc_symbol(
                    db,
                    &node,
                    global_idx_gen.pull(),
                    SymbolKind::GlobalDef,
                    &module,
                ));
            }
            SyntaxKind::MEM_USE => {
                if let Some(symbol) = node
                    .children()
                    .find(|child| child.kind() == SyntaxKind::INDEX)
                    .and_then(|index| {
                        create_ref_symbol(
                            db,
                            &index,
                            SymbolKey::new(&module),
                            SymbolKind::MemoryRef,
                        )
                    })
                {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::MODULE_FIELD_EXPORT | SyntaxKind::EXPORT => {
                if let Some(name) = node.children().find(|node| node.kind() == SyntaxKind::NAME) {
                    exports.push(ExportItem {
                        name: name.to_string(),
                        range: name.text_range(),
                        module: SymbolKey::new(&module),
                    });
                }
            }
            _ => {}
        });
    });

    // replace struct fields' region with their actual region
    let type_symbols = symbols
        .iter()
        .filter(|symbol| matches!(symbol.kind, SymbolKind::Type | SymbolKind::TypeUse))
        .cloned()
        .collect::<Vec<_>>();
    symbols
        .iter_mut()
        .filter(|symbol| symbol.kind == SymbolKind::FieldRef)
        .for_each(|ref_symbol| {
            if let Some(struct_def_symbol) = find_def(&type_symbols, ref_symbol.region) {
                ref_symbol.region = struct_def_symbol.key;
            }
        });

    Arc::new(SymbolTable {
        symbols,
        blocks,
        exports,
    })
}

impl SymbolTable {
    pub fn find_def(&self, key: SymbolKey) -> Option<&Symbol> {
        find_def(&self.symbols, key)
    }

    pub fn find_def_by_symbol(&self, ref_symbol: &Symbol) -> Option<&Symbol> {
        find_def_by_symbol(&self.symbols, ref_symbol)
    }

    pub fn find_param_def(&self, key: SymbolKey) -> Option<&Symbol> {
        self.find_local_ref(key).and_then(|local| {
            self.symbols.iter().find(|symbol| {
                symbol.region == local.region
                    && symbol.kind == SymbolKind::Param
                    && local.idx.is_defined_by(&symbol.idx)
            })
        })
    }

    pub fn find_param_or_local_def(&self, key: SymbolKey) -> Option<&Symbol> {
        self.find_local_ref(key).and_then(|local| {
            self.symbols.iter().find(|symbol| {
                symbol.region == local.region
                    && matches!(symbol.kind, SymbolKind::Param | SymbolKind::Local)
                    && local.idx.is_defined_by(&symbol.idx)
            })
        })
    }

    fn find_local_ref(&self, key: SymbolKey) -> Option<&Symbol> {
        self.symbols
            .iter()
            .find(|symbol| symbol.kind == SymbolKind::LocalRef && symbol.key == key)
    }

    pub fn find_block_def(&self, key: SymbolKey) -> Option<SymbolKey> {
        if self.find_block_ref(key).is_some() {
            self.blocks
                .iter()
                .find(|block| key == block.ref_key)
                .map(|block| block.def_key)
        } else {
            None
        }
    }

    pub fn find_block_ref(&self, key: SymbolKey) -> Option<&Symbol> {
        self.symbols
            .iter()
            .find(|symbol| symbol.kind == SymbolKind::BlockRef && symbol.key == key)
    }

    pub fn get_declared(
        &self,
        node: SyntaxNode,
        kind: SymbolKind,
    ) -> impl Iterator<Item = &Symbol> {
        let key = SymbolKey::new(&node);
        self.symbols
            .iter()
            .filter(move |symbol| symbol.kind == kind && symbol.region == key)
    }

    pub fn find_block_references(
        &self,
        def_key: SymbolKey,
        with_decl: bool,
    ) -> impl Iterator<Item = &Symbol> {
        if with_decl {
            self.symbols.iter().find(|symbol| symbol.key == def_key)
        } else {
            None
        }
        .into_iter()
        .chain(
            self.blocks
                .iter()
                .filter(move |block| block.def_key == def_key)
                .filter_map(|block| {
                    self.symbols
                        .iter()
                        .find(|symbol| symbol.key == block.ref_key)
                }),
        )
    }

    pub fn find_module(&self, module_id: u32) -> Option<&Symbol> {
        self.symbols
            .iter()
            .find(|symbol| symbol.kind == SymbolKind::Module && symbol.idx.num == Some(module_id))
    }
}

fn find_def(symbols: &[Symbol], key: SymbolKey) -> Option<&Symbol> {
    symbols
        .iter()
        .find(|symbol| symbol.key == key)
        .and_then(|ref_symbol| find_def_by_symbol(symbols, ref_symbol))
}
fn find_def_by_symbol<'a>(symbols: &'a [Symbol], ref_symbol: &Symbol) -> Option<&'a Symbol> {
    let kind = match ref_symbol.kind {
        SymbolKind::Call => SymbolKind::Func,
        SymbolKind::TypeUse => SymbolKind::Type,
        SymbolKind::GlobalRef => SymbolKind::GlobalDef,
        SymbolKind::MemoryRef => SymbolKind::MemoryDef,
        SymbolKind::TableRef => SymbolKind::TableDef,
        SymbolKind::FieldRef => SymbolKind::FieldDef,
        _ => return None,
    };
    symbols.iter().find(move |symbol| {
        symbol.region == ref_symbol.region
            && symbol.kind == kind
            && ref_symbol.idx.is_defined_by(&symbol.idx)
    })
}

pub type SymbolKey = SyntaxNodePtr;

#[derive(Clone, Debug)]
pub struct Symbol {
    pub key: SymbolKey,
    pub green: GreenNode,
    pub region: SymbolKey,
    pub kind: SymbolKind,
    pub idx: Idx,
}
impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.kind == other.kind
    }
}
impl Eq for Symbol {}
impl Hash for Symbol {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.kind.hash(state);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    Module,
    Func,
    Param,
    Local,
    Call,
    LocalRef,
    Type,
    TypeUse,
    GlobalDef,
    GlobalRef,
    MemoryDef,
    MemoryRef,
    TableDef,
    TableRef,
    BlockDef,
    BlockRef,
    FieldDef,
    FieldRef,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BlockItem {
    pub ref_key: SymbolKey,
    pub def_key: SymbolKey,
    pub def_idx: Idx,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ExportItem {
    pub name: String, // with double quotes
    pub range: TextRange,
    pub module: SyntaxNodePtr,
}
