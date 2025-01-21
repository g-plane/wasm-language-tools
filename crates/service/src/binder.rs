use crate::{
    idx::{IdentsCtx, Idx, IdxGen},
    syntax_tree::SyntaxTreeCtx,
    uri::InternUri,
};
use rowan::{
    ast::{support, AstNode},
    GreenNode, TextRange,
};
use std::{hash::Hash, rc::Rc};
use wat_syntax::{
    ast::{ModuleFieldFunc, PlainInstr},
    SyntaxKind, SyntaxNode, SyntaxNodePtr,
};

#[salsa::query_group(SymbolTables)]
pub(crate) trait SymbolTablesCtx: SyntaxTreeCtx + IdentsCtx {
    #[salsa::memoized]
    #[salsa::invoke(create_symbol_table)]
    fn symbol_table(&self, uri: InternUri) -> Rc<SymbolTable>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SymbolTable {
    pub symbols: Vec<Symbol>,
    pub blocks: Vec<BlockItem>,
    pub exports: Vec<ExportItem>,
}
fn create_symbol_table(db: &dyn SymbolTablesCtx, uri: InternUri) -> Rc<SymbolTable> {
    fn create_parent_based_symbol(
        db: &dyn SymbolTablesCtx,
        node: SyntaxNode,
        id: u32,
        kind: SymbolKind,
    ) -> Option<Symbol> {
        node.parent().map(|parent| Symbol {
            key: SymbolKey::new(&node),
            green: node.green().into(),
            region: SymbolKey::new(&parent),
            kind,
            idx: Idx {
                num: Some(id),
                name: support::token(&node, SyntaxKind::IDENT)
                    .map(|token| db.ident(token.text().into())),
            },
        })
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
                kind: kind.clone(),
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
    fn create_import_desc_symbol(
        db: &dyn SymbolTablesCtx,
        node: &SyntaxNode,
        id: u32,
        kind: SymbolKind,
    ) -> Option<Symbol> {
        node.parent()
            .and_then(|parent| parent.parent())
            .map(|module| Symbol {
                key: SymbolKey::new(node),
                green: node.green().into(),
                region: SymbolKey::new(&module),
                kind,
                idx: Idx {
                    num: Some(id),
                    name: support::token(node, SyntaxKind::IDENT)
                        .map(|token| db.ident(token.text().into())),
                },
            })
    }

    let root = SyntaxNode::new_root(db.root(uri));
    let mut func_idx_gen = IdxGen::default();
    let mut type_idx_gen = IdxGen::default();
    let mut global_idx_gen = IdxGen::default();
    let mut mem_idx_gen = IdxGen::default();
    let mut table_idx_gen = IdxGen::default();
    let mut symbols = Vec::with_capacity(2);
    let mut blocks = vec![];
    let mut exports = vec![];
    for node in root.descendants() {
        match node.kind() {
            SyntaxKind::MODULE => {
                func_idx_gen = IdxGen::default();
                type_idx_gen = IdxGen::default();
                global_idx_gen = IdxGen::default();
                mem_idx_gen = IdxGen::default();
                table_idx_gen = IdxGen::default();
                let region = if let Some(parent) = node.parent() {
                    SymbolKey::new(&parent)
                } else {
                    continue;
                };
                symbols.push(Symbol {
                    green: node.green().into(),
                    key: SymbolKey::new(&node),
                    region,
                    kind: SymbolKind::Module,
                    idx: Idx {
                        num: None,
                        name: None,
                    },
                });
            }
            SyntaxKind::MODULE_FIELD_FUNC => {
                if let Some(symbol) = create_parent_based_symbol(
                    db,
                    node.clone(),
                    func_idx_gen.pull(),
                    SymbolKind::Func,
                ) {
                    symbols.push(symbol);
                }
                let Some(func) = ModuleFieldFunc::cast(node.clone()) else {
                    continue;
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
            SyntaxKind::MODULE_FIELD_TYPE => {
                if let Some(symbol) = create_parent_based_symbol(
                    db,
                    node.clone(),
                    type_idx_gen.pull(),
                    SymbolKind::Type,
                ) {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::MODULE_FIELD_GLOBAL => {
                if let Some(symbol) = create_parent_based_symbol(
                    db,
                    node.clone(),
                    global_idx_gen.pull(),
                    SymbolKind::GlobalDef,
                ) {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::PLAIN_INSTR => {
                let Some(instr) = PlainInstr::cast(node.clone()) else {
                    continue;
                };
                match instr.instr_name().as_ref().map(|token| token.text()) {
                    Some("call" | "ref.func" | "return_call") => {
                        let Some(region) = node
                            .ancestors()
                            .find(|node| node.kind() == SyntaxKind::MODULE)
                            .map(|node| SymbolKey::new(&node))
                        else {
                            continue;
                        };
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, &node, region, SymbolKind::Call)
                        }));
                    }
                    Some("local.get" | "local.set" | "local.tee") => {
                        let Some(region) = node
                            .ancestors()
                            .find(|node| node.kind() == SyntaxKind::MODULE_FIELD_FUNC)
                            .map(|node| SymbolKey::new(&node))
                        else {
                            continue;
                        };
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, &node, region, SymbolKind::LocalRef)
                        }));
                    }
                    Some("global.get" | "global.set") => {
                        let Some(region) = node
                            .ancestors()
                            .find(|node| node.kind() == SyntaxKind::MODULE)
                            .map(|node| SymbolKey::new(&node))
                        else {
                            continue;
                        };
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, &node, region, SymbolKind::GlobalRef)
                        }));
                    }
                    Some("br" | "br_if" | "br_table") => {
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
                            continue;
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
                                    let mut idx = parent.idx.clone();
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
                    Some("call_indirect") => {
                        let Some(region) = node
                            .ancestors()
                            .find(|node| node.kind() == SyntaxKind::MODULE)
                            .map(|node| SymbolKey::new(&node))
                        else {
                            continue;
                        };
                        symbols.push(
                            instr
                                .immediates()
                                .next()
                                .and_then(|immediate| {
                                    create_ref_symbol(
                                        db,
                                        immediate.syntax(),
                                        region,
                                        SymbolKind::TableRef,
                                    )
                                })
                                .unwrap_or_else(|| Symbol {
                                    green: node.green().into(),
                                    key: SymbolKey::new(&node),
                                    region,
                                    kind: SymbolKind::TableRef,
                                    idx: Idx {
                                        num: Some(0),
                                        name: None,
                                    },
                                }),
                        );
                    }
                    Some(
                        "table.get" | "table.set" | "table.size" | "table.grow" | "table.fill"
                        | "table.copy",
                    ) => {
                        let Some(region) = node
                            .ancestors()
                            .find(|node| node.kind() == SyntaxKind::MODULE)
                            .map(|node| SymbolKey::new(&node))
                        else {
                            continue;
                        };
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, &node, region, SymbolKind::TableRef)
                        }));
                    }
                    Some("table.init") => {
                        let Some(region) = node
                            .ancestors()
                            .find(|node| node.kind() == SyntaxKind::MODULE)
                            .map(|node| SymbolKey::new(&node))
                        else {
                            continue;
                        };
                        let mut children = node.children();
                        if let Some(symbol) = children.next().and_then(|node| {
                            create_ref_symbol(db, &node, region, SymbolKind::TableRef)
                        }) {
                            symbols.push(symbol);
                        }
                    }
                    _ => {}
                }
            }
            SyntaxKind::BLOCK_BLOCK | SyntaxKind::BLOCK_IF | SyntaxKind::BLOCK_LOOP => {
                if let Some(symbol) = create_parent_based_symbol(
                    db,
                    node.clone(),
                    0, // fake ID
                    SymbolKind::BlockDef,
                ) {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::MODULE_FIELD_START | SyntaxKind::EXPORT_DESC_FUNC => {
                if let Some(symbol) = node
                    .ancestors()
                    .find(|node| node.kind() == SyntaxKind::MODULE)
                    .map(|node| SymbolKey::new(&node))
                    .zip(
                        node.children()
                            .find(|child| child.kind() == SyntaxKind::INDEX),
                    )
                    .and_then(|(region, index)| {
                        create_ref_symbol(db, &index, region, SymbolKind::Call)
                    })
                {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::TYPE_USE => {
                if let Some(symbol) = node
                    .ancestors()
                    .find(|node| node.kind() == SyntaxKind::MODULE)
                    .map(|node| SymbolKey::new(&node))
                    .zip(
                        node.children()
                            .find(|child| child.kind() == SyntaxKind::INDEX),
                    )
                    .and_then(|(region, index)| {
                        create_ref_symbol(db, &index, region, SymbolKind::TypeUse)
                    })
                {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::MODULE_FIELD_MEMORY => {
                if let Some(symbol) = create_parent_based_symbol(
                    db,
                    node.clone(),
                    mem_idx_gen.pull(),
                    SymbolKind::MemoryDef,
                ) {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::MODULE_FIELD_TABLE => {
                if let Some(symbol) = create_parent_based_symbol(
                    db,
                    node.clone(),
                    table_idx_gen.pull(),
                    SymbolKind::TableDef,
                ) {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::EXPORT_DESC_GLOBAL => {
                if let Some(symbol) = node
                    .ancestors()
                    .find(|node| node.kind() == SyntaxKind::MODULE)
                    .map(|node| SymbolKey::new(&node))
                    .zip(
                        node.children()
                            .find(|child| child.kind() == SyntaxKind::INDEX),
                    )
                    .and_then(|(region, index)| {
                        create_ref_symbol(db, &index, region, SymbolKind::GlobalRef)
                    })
                {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::EXPORT_DESC_MEMORY => {
                if let Some(symbol) = node
                    .ancestors()
                    .find(|node| node.kind() == SyntaxKind::MODULE)
                    .map(|node| SymbolKey::new(&node))
                    .zip(
                        node.children()
                            .find(|child| child.kind() == SyntaxKind::INDEX),
                    )
                    .and_then(|(region, index)| {
                        create_ref_symbol(db, &index, region, SymbolKind::MemoryRef)
                    })
                {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::EXPORT_DESC_TABLE => {
                if let Some(symbol) = node
                    .ancestors()
                    .find(|node| node.kind() == SyntaxKind::MODULE)
                    .map(|node| SymbolKey::new(&node))
                    .zip(
                        node.children()
                            .find(|child| child.kind() == SyntaxKind::INDEX),
                    )
                    .and_then(|(region, index)| {
                        create_ref_symbol(db, &index, region, SymbolKind::TableRef)
                    })
                {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::IMPORT_DESC_TYPE_USE => {
                if let Some(symbol) =
                    create_import_desc_symbol(db, &node, func_idx_gen.pull(), SymbolKind::Func)
                {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::IMPORT_DESC_TABLE_TYPE => {
                if let Some(symbol) =
                    create_import_desc_symbol(db, &node, table_idx_gen.pull(), SymbolKind::TableDef)
                {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::IMPORT_DESC_MEMORY_TYPE => {
                if let Some(symbol) =
                    create_import_desc_symbol(db, &node, mem_idx_gen.pull(), SymbolKind::MemoryDef)
                {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::IMPORT_DESC_GLOBAL_TYPE => {
                if let Some(symbol) = create_import_desc_symbol(
                    db,
                    &node,
                    global_idx_gen.pull(),
                    SymbolKind::GlobalDef,
                ) {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::MEM_USE => {
                if let Some(symbol) = node
                    .ancestors()
                    .find(|node| node.kind() == SyntaxKind::MODULE)
                    .map(|node| SymbolKey::new(&node))
                    .zip(
                        node.children()
                            .find(|child| child.kind() == SyntaxKind::INDEX),
                    )
                    .and_then(|(region, index)| {
                        create_ref_symbol(db, &index, region, SymbolKind::MemoryRef)
                    })
                {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::MODULE_FIELD_EXPORT | SyntaxKind::EXPORT => {
                if let Some((name, module)) = node
                    .children()
                    .find(|node| node.kind() == SyntaxKind::NAME)
                    .zip(
                        node.ancestors()
                            .find(|node| node.kind() == SyntaxKind::MODULE),
                    )
                {
                    exports.push(ExportItem {
                        name: name.to_string(),
                        range: name.text_range(),
                        module: SymbolKey::new(&module),
                    });
                }
            }
            _ => {}
        }
    }
    Rc::new(SymbolTable {
        symbols,
        blocks,
        exports,
    })
}

impl SymbolTable {
    pub fn find_defs(&self, key: SymbolKey) -> Option<impl Iterator<Item = &Symbol>> {
        self.symbols
            .iter()
            .find(|symbol| symbol.key == key)
            .and_then(|ref_symbol| {
                let kind = match ref_symbol.kind {
                    SymbolKind::Call => SymbolKind::Func,
                    SymbolKind::TypeUse => SymbolKind::Type,
                    SymbolKind::GlobalRef => SymbolKind::GlobalDef,
                    SymbolKind::MemoryRef => SymbolKind::MemoryDef,
                    SymbolKind::TableRef => SymbolKind::TableDef,
                    _ => return None,
                };
                Some(self.symbols.iter().filter(move |symbol| {
                    symbol.region == ref_symbol.region
                        && symbol.kind == kind
                        && ref_symbol.idx.is_defined_by(&symbol.idx)
                }))
            })
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockItem {
    pub ref_key: SymbolKey,
    pub def_key: SymbolKey,
    pub def_idx: Idx,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportItem {
    pub name: String, // with double quotes
    pub range: TextRange,
    pub module: SyntaxNodePtr,
}
