use crate::{
    document::Document,
    idx::{Idx, IdxGen, InternIdent},
};
use indexmap::IndexMap;
use rowan::{
    GreenNode, TextRange,
    ast::{AstNode, support},
};
use rustc_hash::FxBuildHasher;
use std::{hash::Hash, ops::Deref};
use wat_syntax::{
    SyntaxKind, SyntaxNode, SyntaxNodePtr,
    ast::{CompType, ModuleFieldFunc, PlainInstr, SubType},
};

#[derive(Clone, Debug, PartialEq, Eq, salsa::Update)]
pub(crate) struct SymbolTable<'db> {
    pub symbols: Symbols<'db>,
    pub blocks: Vec<BlockItem<'db>>,
    pub exports: Vec<ExportItem>,
}
fn create_symbol_table<'db>(db: &'db dyn salsa::Database, document: Document) -> SymbolTable<'db> {
    fn create_module_level_symbol<'db>(
        db: &'db dyn salsa::Database,
        node: SyntaxNode,
        id: u32,
        kind: SymbolKind,
        module: &SyntaxNode,
    ) -> Symbol<'db> {
        Symbol {
            key: SymbolKey::new(&node),
            green: node.green().into(),
            region: SymbolKey::new(module),
            kind,
            idx: Idx {
                num: Some(id),
                name: support::token(&node, SyntaxKind::IDENT)
                    .map(|token| InternIdent::new(db, token.text())),
            },
            idx_kind: kind.into(),
        }
    }
    fn create_ref_symbol<'db>(
        db: &'db dyn salsa::Database,
        node: &SyntaxNode,
        region: SymbolKey,
        kind: SymbolKind,
    ) -> Option<Symbol<'db>> {
        support::token(node, SyntaxKind::IDENT)
            .map(|ident| Idx {
                num: None,
                name: Some(InternIdent::new(db, ident.text())),
            })
            .or_else(|| {
                node.children_with_tokens()
                    .filter_map(|it| it.into_token())
                    .find(|it| matches!(it.kind(), SyntaxKind::UNSIGNED_INT | SyntaxKind::INT))
                    .and_then(|token| token.text().parse().ok())
                    .map(|num| Idx {
                        num: Some(num),
                        name: None,
                    })
            })
            .map(|idx| Symbol {
                key: SymbolKey::new(node),
                green: node.green().into(),
                region,
                kind,
                idx,
                idx_kind: kind.into(),
            })
    }
    fn create_first_optional_ref_symbol<'db>(
        db: &'db dyn salsa::Database,
        instr: PlainInstr,
        kind: SymbolKind,
    ) -> Option<Symbol<'db>> {
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
                        idx_kind: kind.into(),
                    })
            })
    }
    fn create_import_desc_symbol<'db>(
        db: &'db dyn salsa::Database,
        node: &SyntaxNode,
        id: u32,
        kind: SymbolKind,
        module: &SyntaxNode,
    ) -> Symbol<'db> {
        Symbol {
            key: SymbolKey::new(node),
            green: node.green().into(),
            region: SymbolKey::new(module),
            kind,
            idx: Idx {
                num: Some(id),
                name: support::token(node, SyntaxKind::IDENT)
                    .map(|token| InternIdent::new(db, token.text())),
            },
            idx_kind: kind.into(),
        }
    }

    let root = document.root_tree(db);
    let mut symbols = Symbols::with_capacity_and_hasher(8, FxBuildHasher);
    let mut blocks = vec![];
    let mut exports = vec![];
    root.children().enumerate().for_each(|(module_id, module)| {
        let module_key = SymbolKey::new(&module);
        symbols.insert(
            module_key,
            Symbol {
                green: module.green().into(),
                key: module_key,
                region: SymbolKey::new(&root),
                kind: SymbolKind::Module,
                idx: Idx {
                    num: Some(module_id as u32),
                    name: None,
                },
                idx_kind: IdxKind::Module,
            },
        );
        let mut func_idx_gen = IdxGen::default();
        let mut type_idx_gen = IdxGen::default();
        let mut global_idx_gen = IdxGen::default();
        let mut mem_idx_gen = IdxGen::default();
        let mut table_idx_gen = IdxGen::default();
        module.descendants().for_each(|node| match node.kind() {
            SyntaxKind::MODULE_FIELD_FUNC => {
                let symbol = create_module_level_symbol(
                    db,
                    node.clone(),
                    func_idx_gen.pull(),
                    SymbolKind::Func,
                    &module,
                );
                let func_key = symbol.key;
                symbols.insert(func_key, symbol);
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
                            let key = SymbolKey::new(param);
                            symbols.insert(
                                key,
                                Symbol {
                                    key,
                                    green: param.green().into(),
                                    region: func_key,
                                    kind: SymbolKind::Param,
                                    idx: Idx {
                                        num: Some(local_idx_gen.pull()),
                                        name: Some(InternIdent::new(db, ident.text())),
                                    },
                                    idx_kind: IdxKind::Local,
                                },
                            );
                        } else {
                            symbols.extend(param.val_types().map(|val_type| {
                                let val_type = val_type.syntax();
                                let key = SymbolKey::new(val_type);
                                (
                                    key,
                                    Symbol {
                                        key,
                                        green: val_type.green().into(),
                                        region: func_key,
                                        kind: SymbolKind::Param,
                                        idx: Idx {
                                            num: Some(local_idx_gen.pull()),
                                            name: None,
                                        },
                                        idx_kind: IdxKind::Local,
                                    },
                                )
                            }));
                        }
                    });
                func.locals().for_each(|local| {
                    if let Some(ident) = local.ident_token() {
                        let local = local.syntax();
                        let key = SymbolKey::new(local);
                        symbols.insert(
                            key,
                            Symbol {
                                key,
                                green: local.green().into(),
                                region: func_key,
                                kind: SymbolKind::Local,
                                idx: Idx {
                                    num: Some(local_idx_gen.pull()),
                                    name: Some(InternIdent::new(db, ident.text())),
                                },
                                idx_kind: IdxKind::Local,
                            },
                        );
                    } else {
                        symbols.extend(local.val_types().map(|val_type| {
                            let val_type = val_type.syntax();
                            let key = SymbolKey::new(val_type);
                            (
                                key,
                                Symbol {
                                    key,
                                    green: val_type.green().into(),
                                    region: func_key,
                                    kind: SymbolKind::Local,
                                    idx: Idx {
                                        num: Some(local_idx_gen.pull()),
                                        name: None,
                                    },
                                    idx_kind: IdxKind::Local,
                                },
                            )
                        }));
                    }
                });
            }
            SyntaxKind::TYPE_DEF => {
                let symbol = create_module_level_symbol(
                    db,
                    node.clone(),
                    type_idx_gen.pull(),
                    SymbolKind::Type,
                    &module,
                );
                let type_def_key = symbol.key;
                symbols.insert(type_def_key, symbol);
                if let Some(CompType::Struct(s)) =
                    support::child::<SubType>(&node).and_then(|sub_type| sub_type.comp_type())
                {
                    let mut field_idx_gen = IdxGen::default();
                    s.fields().for_each(|field| {
                        if let Some(ident) = field.ident_token() {
                            let field = field.syntax();
                            let key = SymbolKey::new(field);
                            symbols.insert(
                                key,
                                Symbol {
                                    key,
                                    green: field.green().into(),
                                    region: type_def_key,
                                    kind: SymbolKind::FieldDef,
                                    idx: Idx {
                                        num: Some(field_idx_gen.pull()),
                                        name: Some(InternIdent::new(db, ident.text())),
                                    },
                                    idx_kind: IdxKind::Field,
                                },
                            );
                        } else {
                            symbols.extend(field.field_types().map(|field_type| {
                                let field_type = field_type.syntax();
                                let key = SymbolKey::new(field_type);
                                (
                                    key,
                                    Symbol {
                                        key,
                                        green: field_type.green().into(),
                                        region: type_def_key,
                                        kind: SymbolKind::FieldDef,
                                        idx: Idx {
                                            num: Some(field_idx_gen.pull()),
                                            name: None,
                                        },
                                        idx_kind: IdxKind::Field,
                                    },
                                )
                            }));
                        }
                    });
                }
            }
            SyntaxKind::MODULE_FIELD_GLOBAL => {
                let symbol = create_module_level_symbol(
                    db,
                    node.clone(),
                    global_idx_gen.pull(),
                    SymbolKind::GlobalDef,
                    &module,
                );
                symbols.insert(symbol.key, symbol);
            }
            SyntaxKind::PLAIN_INSTR => {
                let Some(instr) = PlainInstr::cast(node) else {
                    return;
                };
                let node = instr.syntax();
                match instr.instr_name().as_ref().map(|token| token.text()) {
                    Some("call" | "ref.func" | "return_call") => {
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, &node, module_key, SymbolKind::Call)
                                .map(|symbol| (symbol.key, symbol))
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
                                .map(|symbol| (symbol.key, symbol))
                        }));
                    }
                    Some("global.get" | "global.set") => {
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, &node, module_key, SymbolKind::GlobalRef)
                                .map(|symbol| (symbol.key, symbol))
                        }));
                    }
                    Some(
                        "br" | "br_if" | "br_table" | "br_on_null" | "br_on_non_null"
                        | "br_on_cast" | "br_on_cast_fail",
                    ) => {
                        let Some(region) = node
                            .ancestors()
                            .find(|node| {
                                matches!(
                                    node.kind(),
                                    SyntaxKind::BLOCK_BLOCK
                                        | SyntaxKind::BLOCK_LOOP
                                        | SyntaxKind::BLOCK_IF
                                        | SyntaxKind::MODULE_FIELD_FUNC
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
                                while let Some(parent) = symbols.values().find(|sym| {
                                    sym.kind == SymbolKind::BlockDef && sym.key == current.region
                                }) {
                                    let mut idx = parent.idx;
                                    idx.num = Some(levels);
                                    if symbol.idx.is_defined_by(&idx) {
                                        blocks.push(BlockItem {
                                            ref_key: symbol.key,
                                            def_key: parent.key,
                                            def_idx: idx,
                                        });
                                    }
                                    current = parent;
                                    levels += 1;
                                }
                                let func_block_idx = Idx {
                                    num: Some(levels),
                                    name: None,
                                };
                                if symbol.idx.is_defined_by(&func_block_idx)
                                    && let Some(func_symbol) = symbols.values().find(|symbol| {
                                        symbol.kind == SymbolKind::Func
                                            && symbol.key == current.region
                                    })
                                {
                                    blocks.push(BlockItem {
                                        ref_key: symbol.key,
                                        def_key: func_symbol.key,
                                        def_idx: func_block_idx,
                                    });
                                }
                                symbols.insert(symbol.key, symbol);
                            });
                    }
                    Some("call_indirect" | "return_call_indirect") => {
                        if let Some(symbol) =
                            create_first_optional_ref_symbol(db, instr, SymbolKind::TableRef)
                        {
                            symbols.insert(symbol.key, symbol);
                        }
                    }
                    Some(
                        "table.get" | "table.set" | "table.size" | "table.grow" | "table.fill"
                        | "table.copy",
                    ) => {
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, &node, module_key, SymbolKind::TableRef)
                                .map(|symbol| (symbol.key, symbol))
                        }));
                    }
                    Some("table.init") => {
                        if let Some(symbol) = node.children().next().and_then(|node| {
                            create_ref_symbol(db, &node, module_key, SymbolKind::TableRef)
                        }) {
                            symbols.insert(symbol.key, symbol);
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
                            symbols.insert(symbol.key, symbol);
                        }
                    }
                    Some("memory.copy") => {
                        if let Some(symbol) = node.children().next().and_then(|node| {
                            create_ref_symbol(db, &node, module_key, SymbolKind::MemoryRef)
                        }) {
                            symbols.insert(symbol.key, symbol);
                        }
                    }
                    Some(
                        "struct.new" | "struct.new_default" | "array.new" | "array.new_default"
                        | "array.new_fixed" | "array.new_data" | "array.new_elem" | "array.get"
                        | "array.get_u" | "array.get_s" | "array.set" | "array.fill"
                        | "array.init_data" | "array.init_elem" | "call_ref" | "return_call_ref"
                        | "ref.null",
                    ) => {
                        if let Some(symbol) = node.children().next().and_then(|node| {
                            create_ref_symbol(db, &node, module_key, SymbolKind::TypeUse)
                        }) {
                            symbols.insert(symbol.key, symbol);
                        }
                    }
                    Some("array.copy") => {
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, &node, module_key, SymbolKind::TypeUse)
                                .map(|symbol| (symbol.key, symbol))
                        }));
                    }
                    Some("struct.get" | "struct.get_s" | "struct.get_u" | "struct.set") => {
                        let mut children = node.children();
                        if let Some(symbol) = children.next().and_then(|node| {
                            create_ref_symbol(db, &node, module_key, SymbolKind::TypeUse)
                        }) {
                            let key = symbol.key;
                            symbols.insert(key, symbol);
                            if let Some(symbol) = children.next().and_then(|node| {
                                // The region here is temporary.
                                // It's used for tracking which struct it belongs to,
                                // and it will be replaced with the actual region later.
                                // If the struct it belongs to isn't defined, nothing will happen.
                                create_ref_symbol(db, &node, key, SymbolKind::FieldRef)
                            }) {
                                symbols.insert(symbol.key, symbol);
                            }
                        }
                    }
                    _ => {}
                }
            }
            SyntaxKind::BLOCK_BLOCK | SyntaxKind::BLOCK_IF | SyntaxKind::BLOCK_LOOP => {
                if let Some(symbol) = node
                    .ancestors()
                    .skip(1)
                    .find(|node| {
                        matches!(
                            node.kind(),
                            SyntaxKind::BLOCK_BLOCK
                                | SyntaxKind::BLOCK_LOOP
                                | SyntaxKind::BLOCK_IF
                                | SyntaxKind::MODULE_FIELD_FUNC
                        )
                    })
                    .map(|region| Symbol {
                        key: SymbolKey::new(&node),
                        green: node.green().into(),
                        region: SymbolKey::new(&region),
                        kind: SymbolKind::BlockDef,
                        idx: Idx {
                            num: Some(0), // fake ID
                            name: support::token(&node, SyntaxKind::IDENT)
                                .map(|token| InternIdent::new(db, token.text())),
                        },
                        idx_kind: IdxKind::Block,
                    })
                {
                    symbols.insert(symbol.key, symbol);
                }
            }
            SyntaxKind::MODULE_FIELD_START | SyntaxKind::EXPORT_DESC_FUNC => {
                if let Some(symbol) = node
                    .children()
                    .find(|child| child.kind() == SyntaxKind::INDEX)
                    .and_then(|index| create_ref_symbol(db, &index, module_key, SymbolKind::Call))
                {
                    symbols.insert(symbol.key, symbol);
                }
            }
            SyntaxKind::TYPE_USE | SyntaxKind::HEAP_TYPE | SyntaxKind::SUB_TYPE => {
                if let Some(symbol) = node
                    .children()
                    .find(|child| child.kind() == SyntaxKind::INDEX)
                    .and_then(|index| {
                        create_ref_symbol(db, &index, module_key, SymbolKind::TypeUse)
                    })
                {
                    symbols.insert(symbol.key, symbol);
                }
            }
            SyntaxKind::MODULE_FIELD_MEMORY => {
                let symbol = create_module_level_symbol(
                    db,
                    node.clone(),
                    mem_idx_gen.pull(),
                    SymbolKind::MemoryDef,
                    &module,
                );
                symbols.insert(symbol.key, symbol);
            }
            SyntaxKind::MODULE_FIELD_TABLE => {
                let symbol = create_module_level_symbol(
                    db,
                    node.clone(),
                    table_idx_gen.pull(),
                    SymbolKind::TableDef,
                    &module,
                );
                symbols.insert(symbol.key, symbol);
            }
            SyntaxKind::EXPORT_DESC_GLOBAL => {
                if let Some(symbol) = node
                    .children()
                    .find(|child| child.kind() == SyntaxKind::INDEX)
                    .and_then(|index| {
                        create_ref_symbol(db, &index, module_key, SymbolKind::GlobalRef)
                    })
                {
                    symbols.insert(symbol.key, symbol);
                }
            }
            SyntaxKind::EXPORT_DESC_MEMORY => {
                if let Some(symbol) = node
                    .children()
                    .find(|child| child.kind() == SyntaxKind::INDEX)
                    .and_then(|index| {
                        create_ref_symbol(db, &index, module_key, SymbolKind::MemoryRef)
                    })
                {
                    symbols.insert(symbol.key, symbol);
                }
            }
            SyntaxKind::EXPORT_DESC_TABLE | SyntaxKind::TABLE_USE => {
                if let Some(symbol) = node
                    .children()
                    .find(|child| child.kind() == SyntaxKind::INDEX)
                    .and_then(|index| {
                        create_ref_symbol(db, &index, module_key, SymbolKind::TableRef)
                    })
                {
                    symbols.insert(symbol.key, symbol);
                }
            }
            SyntaxKind::IMPORT_DESC_TYPE_USE => {
                let symbol = create_import_desc_symbol(
                    db,
                    &node,
                    func_idx_gen.pull(),
                    SymbolKind::Func,
                    &module,
                );
                symbols.insert(symbol.key, symbol);
            }
            SyntaxKind::IMPORT_DESC_TABLE_TYPE => {
                let symbol = create_import_desc_symbol(
                    db,
                    &node,
                    table_idx_gen.pull(),
                    SymbolKind::TableDef,
                    &module,
                );
                symbols.insert(symbol.key, symbol);
            }
            SyntaxKind::IMPORT_DESC_MEMORY_TYPE => {
                let symbol = create_import_desc_symbol(
                    db,
                    &node,
                    mem_idx_gen.pull(),
                    SymbolKind::MemoryDef,
                    &module,
                );
                symbols.insert(symbol.key, symbol);
            }
            SyntaxKind::IMPORT_DESC_GLOBAL_TYPE => {
                let symbol = create_import_desc_symbol(
                    db,
                    &node,
                    global_idx_gen.pull(),
                    SymbolKind::GlobalDef,
                    &module,
                );
                symbols.insert(symbol.key, symbol);
            }
            SyntaxKind::MEM_USE => {
                if let Some(symbol) = node
                    .children()
                    .find(|child| child.kind() == SyntaxKind::INDEX)
                    .and_then(|index| {
                        create_ref_symbol(db, &index, module_key, SymbolKind::MemoryRef)
                    })
                {
                    symbols.insert(symbol.key, symbol);
                }
            }
            SyntaxKind::MODULE_FIELD_EXPORT | SyntaxKind::EXPORT => {
                if let Some(name) = node.children().find(|node| node.kind() == SyntaxKind::NAME) {
                    exports.push(ExportItem {
                        name: name.to_string(),
                        range: name.text_range(),
                        module: module_key,
                    });
                }
            }
            SyntaxKind::ELEM_LIST => {
                symbols.extend(
                    node.children()
                        .filter(|child| child.kind() == SyntaxKind::INDEX)
                        .filter_map(|index| {
                            create_ref_symbol(db, &index, module_key, SymbolKind::Call)
                                .map(|symbol| (symbol.key, symbol))
                        }),
                );
            }
            _ => {}
        });
    });

    // replace struct fields' region with their actual region
    let type_symbols = symbols
        .iter()
        .filter(|(_, symbol)| matches!(symbol.kind, SymbolKind::Type | SymbolKind::TypeUse))
        .map(|(key, symbol)| (*key, symbol.clone()))
        .collect();
    symbols
        .values_mut()
        .filter(|symbol| symbol.kind == SymbolKind::FieldRef)
        .for_each(|ref_symbol| {
            if let Some(struct_def_symbol) = find_def(&type_symbols, ref_symbol.region) {
                ref_symbol.region = struct_def_symbol.key;
            }
        });

    SymbolTable {
        symbols,
        blocks,
        exports,
    }
}

impl<'db> SymbolTable<'db> {
    pub fn find_def(&'db self, key: SymbolKey) -> Option<&'db Symbol<'db>> {
        find_def(&self.symbols, key)
    }

    pub fn find_def_by_symbol(&'db self, ref_symbol: &'db Symbol<'db>) -> Option<&'db Symbol<'db>> {
        find_def_by_symbol(&self.symbols, ref_symbol)
    }

    pub fn find_block_def(&self, key: SymbolKey) -> Option<SymbolKey> {
        if self.symbols.get(&key).is_some() {
            self.blocks
                .iter()
                .find(|block| key == block.ref_key)
                .map(|block| block.def_key)
        } else {
            None
        }
    }

    pub fn get_declared(
        &self,
        node: SyntaxNode,
        kind: SymbolKind,
    ) -> impl Iterator<Item = &Symbol<'db>> {
        let key = SymbolKey::new(&node);
        self.symbols
            .values()
            .filter(move |symbol| symbol.kind == kind && symbol.region == key)
    }

    pub fn find_block_references(
        &self,
        def_key: SymbolKey,
        with_decl: bool,
    ) -> impl Iterator<Item = &Symbol<'db>> {
        if with_decl {
            self.symbols.get(&def_key)
        } else {
            None
        }
        .into_iter()
        .chain(
            self.blocks
                .iter()
                .filter(move |block| block.def_key == def_key)
                .filter_map(|block| self.symbols.get(&block.ref_key)),
        )
    }

    pub fn find_module(&self, module_id: u32) -> Option<&Symbol<'db>> {
        self.symbols
            .values()
            .find(|symbol| symbol.kind == SymbolKind::Module && symbol.idx.num == Some(module_id))
    }
}
#[salsa::tracked]
impl<'db> SymbolTable<'db> {
    #[salsa::tracked(returns(ref))]
    pub(crate) fn of(db: &'db dyn salsa::Database, document: Document) -> Self {
        create_symbol_table(db, document)
    }
}

fn find_def<'db>(symbols: &'db Symbols<'db>, key: SymbolKey) -> Option<&'db Symbol<'db>> {
    symbols
        .get(&key)
        .and_then(|ref_symbol| find_def_by_symbol(symbols, ref_symbol))
}
fn find_def_by_symbol<'db>(
    symbols: &'db Symbols<'db>,
    ref_symbol: &'db Symbol<'db>,
) -> Option<&'db Symbol<'db>> {
    symbols.values().find(move |symbol| {
        symbol.kind.is_def()
            && symbol.region == ref_symbol.region
            && symbol.idx_kind == ref_symbol.idx_kind
            && ref_symbol.idx.is_defined_by(&symbol.idx)
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, salsa::Update)]
/// Wrapper type for allowing `SyntaxNodePtr` to be stored in Salsa database.
pub struct SymbolKey(SyntaxNodePtr);
impl SymbolKey {
    pub fn new(node: &SyntaxNode) -> Self {
        SymbolKey(SyntaxNodePtr::new(node))
    }
}
impl Deref for SymbolKey {
    type Target = SyntaxNodePtr;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug, salsa::Update)]
pub struct Symbol<'db> {
    pub key: SymbolKey,
    pub green: GreenNode,
    pub region: SymbolKey,
    pub kind: SymbolKind,
    pub idx: Idx<'db>,
    pub idx_kind: IdxKind,
}
impl PartialEq for Symbol<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl Eq for Symbol<'_> {}
impl Hash for Symbol<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
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
impl SymbolKind {
    fn is_def(&self) -> bool {
        matches!(
            self,
            SymbolKind::Func
                | SymbolKind::Param
                | SymbolKind::Local
                | SymbolKind::Type
                | SymbolKind::GlobalDef
                | SymbolKind::MemoryDef
                | SymbolKind::TableDef
                | SymbolKind::BlockDef
                | SymbolKind::FieldDef
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IdxKind {
    Module,
    Func,
    Local,
    Type,
    Global,
    Memory,
    Table,
    Block,
    Field,
}
impl From<SymbolKind> for IdxKind {
    fn from(value: SymbolKind) -> Self {
        match value {
            SymbolKind::Module => IdxKind::Module,
            SymbolKind::Func | SymbolKind::Call => IdxKind::Func,
            SymbolKind::Param | SymbolKind::Local | SymbolKind::LocalRef => IdxKind::Local,
            SymbolKind::Type | SymbolKind::TypeUse => IdxKind::Type,
            SymbolKind::GlobalDef | SymbolKind::GlobalRef => IdxKind::Global,
            SymbolKind::MemoryDef | SymbolKind::MemoryRef => IdxKind::Memory,
            SymbolKind::TableDef | SymbolKind::TableRef => IdxKind::Table,
            SymbolKind::BlockDef | SymbolKind::BlockRef => IdxKind::Block,
            SymbolKind::FieldDef | SymbolKind::FieldRef => IdxKind::Field,
        }
    }
}

type Symbols<'db> = IndexMap<SymbolKey, Symbol<'db>, FxBuildHasher>;

#[derive(Clone, Debug, PartialEq, Eq, salsa::Update)]
pub(crate) struct BlockItem<'db> {
    pub ref_key: SymbolKey,
    pub def_key: SymbolKey,
    pub def_idx: Idx<'db>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ExportItem {
    pub name: String, // with double quotes
    pub range: TextRange,
    pub module: SymbolKey,
}
