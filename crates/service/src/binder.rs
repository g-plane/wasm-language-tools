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
    pub symbols: Vec<SymbolItem>,
    pub blocks: Vec<BlockItem>,
    pub exports: Vec<ExportItem>,
}
fn create_symbol_table(db: &dyn SymbolTablesCtx, uri: InternUri) -> Rc<SymbolTable> {
    fn create_parent_based_symbol(
        db: &dyn SymbolTablesCtx,
        node: SyntaxNode,
        id: u32,
        kind: SymbolItemKind,
    ) -> Option<SymbolItem> {
        node.parent().map(|parent| SymbolItem {
            key: SymbolItemKey::new(&node),
            green: node.green().into(),
            region: SymbolItemKey::new(&parent),
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
        node: SyntaxNode,
        region: SymbolItemKey,
        kind: SymbolItemKind,
    ) -> Option<SymbolItem> {
        support::token(&node, SyntaxKind::IDENT)
            .map(|ident| SymbolItem {
                key: SymbolItemKey::new(&node),
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
                    .map(|num| SymbolItem {
                        green: node.green().into(),
                        key: SymbolItemKey::new(&node),
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
        kind: SymbolItemKind,
    ) -> Option<SymbolItem> {
        node.parent()
            .and_then(|parent| parent.parent())
            .map(|module| SymbolItem {
                key: SymbolItemKey::new(node),
                green: node.green().into(),
                region: SymbolItemKey::new(&module),
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
                    SymbolItemKey::new(&parent)
                } else {
                    continue;
                };
                symbols.push(SymbolItem {
                    green: node.green().into(),
                    key: SymbolItemKey::new(&node),
                    region,
                    kind: SymbolItemKind::Module,
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
                    SymbolItemKind::Func,
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
                            symbols.push(SymbolItem {
                                key: SymbolItemKey::new(param),
                                green: param.green().into(),
                                region: SymbolItemKey::new(&node),
                                kind: SymbolItemKind::Param,
                                idx: Idx {
                                    num: Some(local_idx_gen.pull()),
                                    name: Some(db.ident(ident.text().into())),
                                },
                            });
                        } else {
                            symbols.extend(param.val_types().map(|val_type| {
                                let val_type = val_type.syntax();
                                SymbolItem {
                                    key: SymbolItemKey::new(val_type),
                                    green: val_type.green().into(),
                                    region: SymbolItemKey::new(&node),
                                    kind: SymbolItemKind::Param,
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
                        symbols.push(SymbolItem {
                            key: SymbolItemKey::new(local),
                            green: local.green().into(),
                            region: SymbolItemKey::new(&node),
                            kind: SymbolItemKind::Local,
                            idx: Idx {
                                num: Some(local_idx_gen.pull()),
                                name: Some(db.ident(ident.text().into())),
                            },
                        });
                    } else {
                        symbols.extend(local.val_types().map(|val_type| {
                            let val_type = val_type.syntax();
                            SymbolItem {
                                key: SymbolItemKey::new(val_type),
                                green: val_type.green().into(),
                                region: SymbolItemKey::new(&node),
                                kind: SymbolItemKind::Local,
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
                    SymbolItemKind::Type,
                ) {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::MODULE_FIELD_GLOBAL => {
                if let Some(symbol) = create_parent_based_symbol(
                    db,
                    node.clone(),
                    global_idx_gen.pull(),
                    SymbolItemKind::GlobalDef,
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
                            .map(|node| SymbolItemKey::new(&node))
                        else {
                            continue;
                        };
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, node, region, SymbolItemKind::Call)
                        }));
                    }
                    Some("local.get" | "local.set" | "local.tee") => {
                        let Some(region) = node
                            .ancestors()
                            .find(|node| node.kind() == SyntaxKind::MODULE_FIELD_FUNC)
                            .map(|node| SymbolItemKey::new(&node))
                        else {
                            continue;
                        };
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, node, region, SymbolItemKind::LocalRef)
                        }));
                    }
                    Some("global.get" | "global.set") => {
                        let Some(region) = node
                            .ancestors()
                            .find(|node| node.kind() == SyntaxKind::MODULE)
                            .map(|node| SymbolItemKey::new(&node))
                        else {
                            continue;
                        };
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, node, region, SymbolItemKind::GlobalRef)
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
                            .map(|node| SymbolItemKey::new(&node))
                        else {
                            continue;
                        };
                        node.children()
                            .filter_map(|node| {
                                create_ref_symbol(db, node, region, SymbolItemKind::BlockRef)
                            })
                            .for_each(|symbol| {
                                let mut current = &symbol;
                                let mut levels = 0;
                                while let Some(
                                    parent @ SymbolItem {
                                        key,
                                        kind: SymbolItemKind::BlockDef,
                                        ..
                                    },
                                ) = symbols.iter().find(|sym| {
                                    sym.kind == SymbolItemKind::BlockDef
                                        && sym.key == current.region
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
                    Some(
                        "table.get" | "table.set" | "table.size" | "table.grow" | "table.fill"
                        | "table.copy",
                    ) => {
                        let Some(region) = node
                            .ancestors()
                            .find(|node| node.kind() == SyntaxKind::MODULE)
                            .map(|node| SymbolItemKey::new(&node))
                        else {
                            continue;
                        };
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, node, region, SymbolItemKind::TableRef)
                        }));
                    }
                    Some("table.init") => {
                        let Some(region) = node
                            .ancestors()
                            .find(|node| node.kind() == SyntaxKind::MODULE)
                            .map(|node| SymbolItemKey::new(&node))
                        else {
                            continue;
                        };
                        let mut children = node.children();
                        if let Some(symbol) = children.next().and_then(|node| {
                            create_ref_symbol(db, node, region, SymbolItemKind::TableRef)
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
                    SymbolItemKind::BlockDef,
                ) {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::MODULE_FIELD_START | SyntaxKind::EXPORT_DESC_FUNC => {
                if let Some(symbol) = node
                    .ancestors()
                    .find(|node| node.kind() == SyntaxKind::MODULE)
                    .map(|node| SymbolItemKey::new(&node))
                    .zip(
                        node.children()
                            .find(|child| child.kind() == SyntaxKind::INDEX),
                    )
                    .and_then(|(region, index)| {
                        create_ref_symbol(db, index, region, SymbolItemKind::Call)
                    })
                {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::TYPE_USE => {
                if let Some(symbol) = node
                    .ancestors()
                    .find(|node| node.kind() == SyntaxKind::MODULE)
                    .map(|node| SymbolItemKey::new(&node))
                    .zip(
                        node.children()
                            .find(|child| child.kind() == SyntaxKind::INDEX),
                    )
                    .and_then(|(region, index)| {
                        create_ref_symbol(db, index, region, SymbolItemKind::TypeUse)
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
                    SymbolItemKind::MemoryDef,
                ) {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::MODULE_FIELD_TABLE => {
                if let Some(symbol) = create_parent_based_symbol(
                    db,
                    node.clone(),
                    table_idx_gen.pull(),
                    SymbolItemKind::TableDef,
                ) {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::EXPORT_DESC_GLOBAL => {
                if let Some(symbol) = node
                    .ancestors()
                    .find(|node| node.kind() == SyntaxKind::MODULE)
                    .map(|node| SymbolItemKey::new(&node))
                    .zip(
                        node.children()
                            .find(|child| child.kind() == SyntaxKind::INDEX),
                    )
                    .and_then(|(region, index)| {
                        create_ref_symbol(db, index, region, SymbolItemKind::GlobalRef)
                    })
                {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::EXPORT_DESC_MEMORY => {
                if let Some(symbol) = node
                    .ancestors()
                    .find(|node| node.kind() == SyntaxKind::MODULE)
                    .map(|node| SymbolItemKey::new(&node))
                    .zip(
                        node.children()
                            .find(|child| child.kind() == SyntaxKind::INDEX),
                    )
                    .and_then(|(region, index)| {
                        create_ref_symbol(db, index, region, SymbolItemKind::MemoryRef)
                    })
                {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::EXPORT_DESC_TABLE => {
                if let Some(symbol) = node
                    .ancestors()
                    .find(|node| node.kind() == SyntaxKind::MODULE)
                    .map(|node| SymbolItemKey::new(&node))
                    .zip(
                        node.children()
                            .find(|child| child.kind() == SyntaxKind::INDEX),
                    )
                    .and_then(|(region, index)| {
                        create_ref_symbol(db, index, region, SymbolItemKind::TableRef)
                    })
                {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::IMPORT_DESC_TYPE_USE => {
                if let Some(symbol) =
                    create_import_desc_symbol(db, &node, func_idx_gen.pull(), SymbolItemKind::Func)
                {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::IMPORT_DESC_TABLE_TYPE => {
                if let Some(symbol) = create_import_desc_symbol(
                    db,
                    &node,
                    table_idx_gen.pull(),
                    SymbolItemKind::TableDef,
                ) {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::IMPORT_DESC_MEMORY_TYPE => {
                if let Some(symbol) = create_import_desc_symbol(
                    db,
                    &node,
                    mem_idx_gen.pull(),
                    SymbolItemKind::MemoryDef,
                ) {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::IMPORT_DESC_GLOBAL_TYPE => {
                if let Some(symbol) = create_import_desc_symbol(
                    db,
                    &node,
                    global_idx_gen.pull(),
                    SymbolItemKind::GlobalDef,
                ) {
                    symbols.push(symbol);
                }
            }
            SyntaxKind::MEM_USE => {
                if let Some(symbol) = node
                    .ancestors()
                    .find(|node| node.kind() == SyntaxKind::MODULE)
                    .map(|node| SymbolItemKey::new(&node))
                    .zip(
                        node.children()
                            .find(|child| child.kind() == SyntaxKind::INDEX),
                    )
                    .and_then(|(region, index)| {
                        create_ref_symbol(db, index, region, SymbolItemKind::MemoryRef)
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
                        module: SymbolItemKey::new(&module),
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
    pub fn find_defs(&self, key: SymbolItemKey) -> Option<impl Iterator<Item = &SymbolItem>> {
        self.symbols
            .iter()
            .find(|symbol| symbol.key == key)
            .and_then(|ref_symbol| {
                let kind = match ref_symbol.kind {
                    SymbolItemKind::Call => SymbolItemKind::Func,
                    SymbolItemKind::TypeUse => SymbolItemKind::Type,
                    SymbolItemKind::GlobalRef => SymbolItemKind::GlobalDef,
                    SymbolItemKind::MemoryRef => SymbolItemKind::MemoryDef,
                    SymbolItemKind::TableRef => SymbolItemKind::TableDef,
                    _ => return None,
                };
                Some(self.symbols.iter().filter(move |symbol| {
                    symbol.region == ref_symbol.region
                        && symbol.kind == kind
                        && ref_symbol.idx.is_defined_by(&symbol.idx)
                }))
            })
    }

    pub fn find_param_def(&self, key: SymbolItemKey) -> Option<&SymbolItem> {
        self.find_local_ref(key).and_then(|local| {
            self.symbols.iter().find(|symbol| {
                symbol.region == local.region
                    && symbol.kind == SymbolItemKind::Param
                    && local.idx.is_defined_by(&symbol.idx)
            })
        })
    }

    pub fn find_param_or_local_def(&self, key: SymbolItemKey) -> Option<&SymbolItem> {
        self.find_local_ref(key).and_then(|local| {
            self.symbols.iter().find(|symbol| {
                symbol.region == local.region
                    && matches!(symbol.kind, SymbolItemKind::Param | SymbolItemKind::Local)
                    && local.idx.is_defined_by(&symbol.idx)
            })
        })
    }

    fn find_local_ref(&self, key: SymbolItemKey) -> Option<&SymbolItem> {
        self.symbols
            .iter()
            .find(|symbol| symbol.kind == SymbolItemKind::LocalRef && symbol.key == key)
    }

    pub fn find_block_def(&self, key: SymbolItemKey) -> Option<SymbolItemKey> {
        if self.find_block_ref(key).is_some() {
            self.blocks
                .iter()
                .find(|block| key == block.ref_key)
                .map(|block| block.def_key)
        } else {
            None
        }
    }

    pub fn find_block_ref(&self, key: SymbolItemKey) -> Option<&SymbolItem> {
        self.symbols
            .iter()
            .find(|symbol| symbol.kind == SymbolItemKind::BlockRef && symbol.key == key)
    }

    pub fn get_declared(
        &self,
        node: SyntaxNode,
        kind: SymbolItemKind,
    ) -> impl Iterator<Item = &SymbolItem> {
        let key = SymbolItemKey::new(&node);
        self.symbols
            .iter()
            .filter(move |symbol| symbol.kind == kind && symbol.region == key)
    }

    pub fn find_block_references(
        &self,
        def_key: SymbolItemKey,
        with_decl: bool,
    ) -> impl Iterator<Item = &SymbolItem> {
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

pub type SymbolItemKey = SyntaxNodePtr;

#[derive(Clone, Debug)]
pub struct SymbolItem {
    pub key: SymbolItemKey,
    pub green: GreenNode,
    pub region: SymbolItemKey,
    pub kind: SymbolItemKind,
    pub idx: Idx,
}
impl PartialEq for SymbolItem {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.kind == other.kind
    }
}
impl Eq for SymbolItem {}
impl Hash for SymbolItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.kind.hash(state);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SymbolItemKind {
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
    pub ref_key: SymbolItemKey,
    pub def_key: SymbolItemKey,
    pub def_idx: Idx,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportItem {
    pub name: String, // with double quotes
    pub range: TextRange,
    pub module: SyntaxNodePtr,
}
