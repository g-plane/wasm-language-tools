use crate::{
    files::FilesCtx,
    idx::{IdentsCtx, Idx},
    InternUri,
};
use rowan::{
    ast::{support, AstNode, SyntaxNodePtr},
    GreenNode, TextRange,
};
use std::{hash::Hash, rc::Rc};
use wat_syntax::{
    ast::{ModuleFieldFunc, PlainInstr},
    SyntaxKind, SyntaxNode, WatLanguage,
};

#[salsa::query_group(SymbolTables)]
pub(crate) trait SymbolTablesCtx: FilesCtx + IdentsCtx {
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
            key: node.clone().into(),
            green: node.green().into(),
            region: parent.into(),
            kind,
            idx: Idx {
                num: Some(id),
                name: support::token(&node, SyntaxKind::IDENT)
                    .map(|token| db.ident(token.text().to_string())),
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
                key: node.clone().into(),
                green: node.green().into(),
                region: region.clone(),
                kind: kind.clone(),
                idx: Idx {
                    num: None,
                    name: Some(db.ident(ident.text().to_string())),
                },
            })
            .or_else(|| {
                node.children_with_tokens()
                    .filter_map(|it| it.into_token())
                    .find(|it| matches!(it.kind(), SyntaxKind::UNSIGNED_INT | SyntaxKind::INT))
                    .and_then(|token| token.text().parse().ok())
                    .map(|num| SymbolItem {
                        green: node.green().into(),
                        key: node.into(),
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
                key: node.clone().into(),
                green: node.green().into(),
                region: module.into(),
                kind,
                idx: Idx {
                    num: Some(id),
                    name: support::token(node, SyntaxKind::IDENT)
                        .map(|token| db.ident(token.text().to_string())),
                },
            })
    }

    let root = SyntaxNode::new_root(db.root(uri));
    let mut module_field_id = 0;
    let mut symbols = Vec::with_capacity(2);
    let mut blocks = vec![];
    let mut exports = vec![];
    for node in root.descendants() {
        match node.kind() {
            SyntaxKind::MODULE => {
                module_field_id = 0;
                let region = if let Some(parent) = node.parent() {
                    parent.into()
                } else {
                    continue;
                };
                symbols.push(SymbolItem {
                    green: node.green().into(),
                    key: node.into(),
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
                    module_field_id,
                    SymbolItemKind::Func,
                ) {
                    symbols.push(symbol);
                }
                module_field_id += 1;
                let Some(func) = ModuleFieldFunc::cast(node.clone()) else {
                    continue;
                };
                let local_index = func
                    .type_use()
                    .iter()
                    .flat_map(|type_use| type_use.params())
                    .fold(0, |i, param| {
                        if let Some(ident) = param.ident_token() {
                            let param = param.syntax();
                            symbols.push(SymbolItem {
                                key: param.to_owned().into(),
                                green: param.green().into(),
                                region: node.clone().into(),
                                kind: SymbolItemKind::Param,
                                idx: Idx {
                                    num: Some(i),
                                    name: Some(db.ident(ident.text().to_string())),
                                },
                            });
                            i + 1
                        } else {
                            param.val_types().fold(i, |i, val_type| {
                                let val_type = val_type.syntax();
                                symbols.push(SymbolItem {
                                    key: val_type.to_owned().into(),
                                    green: val_type.green().into(),
                                    region: node.clone().into(),
                                    kind: SymbolItemKind::Param,
                                    idx: Idx {
                                        num: Some(i),
                                        name: None,
                                    },
                                });
                                i + 1
                            })
                        }
                    });
                func.locals().fold(local_index, |i, local| {
                    if let Some(ident) = local.ident_token() {
                        let local = local.syntax();
                        symbols.push(SymbolItem {
                            key: local.to_owned().into(),
                            green: local.green().into(),
                            region: node.clone().into(),
                            kind: SymbolItemKind::Local,
                            idx: Idx {
                                num: Some(i),
                                name: Some(db.ident(ident.text().to_string())),
                            },
                        });
                        i + 1
                    } else {
                        local.val_types().fold(i, |i, val_type| {
                            let val_type = val_type.syntax();
                            symbols.push(SymbolItem {
                                key: val_type.to_owned().into(),
                                green: val_type.green().into(),
                                region: node.clone().into(),
                                kind: SymbolItemKind::Local,
                                idx: Idx {
                                    num: Some(i),
                                    name: None,
                                },
                            });
                            i + 1
                        })
                    }
                });
            }
            SyntaxKind::MODULE_FIELD_TYPE => {
                if let Some(symbol) = create_parent_based_symbol(
                    db,
                    node.clone(),
                    module_field_id,
                    SymbolItemKind::Type,
                ) {
                    symbols.push(symbol);
                }
                module_field_id += 1;
            }
            SyntaxKind::MODULE_FIELD_GLOBAL => {
                if let Some(symbol) = create_parent_based_symbol(
                    db,
                    node.clone(),
                    module_field_id,
                    SymbolItemKind::GlobalDef,
                ) {
                    symbols.push(symbol);
                }
                module_field_id += 1;
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
                            .map(SymbolItemKey::from)
                        else {
                            continue;
                        };
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, node, region.clone(), SymbolItemKind::Call)
                        }));
                    }
                    Some("local.get" | "local.set" | "local.tee") => {
                        let Some(region) = node
                            .ancestors()
                            .find(|node| node.kind() == SyntaxKind::MODULE_FIELD_FUNC)
                            .map(SymbolItemKey::from)
                        else {
                            continue;
                        };
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, node, region.clone(), SymbolItemKind::LocalRef)
                        }));
                    }
                    Some("global.get" | "global.set") => {
                        let Some(region) = node
                            .ancestors()
                            .find(|node| node.kind() == SyntaxKind::MODULE)
                            .map(SymbolItemKey::from)
                        else {
                            continue;
                        };
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, node, region.clone(), SymbolItemKind::GlobalRef)
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
                            .map(SymbolItemKey::from)
                        else {
                            continue;
                        };
                        node.children()
                            .filter_map(|node| {
                                create_ref_symbol(
                                    db,
                                    node,
                                    region.clone(),
                                    SymbolItemKind::BlockRef,
                                )
                            })
                            .for_each(|symbol| {
                                symbols.push(symbol.clone());
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
                                    blocks.push(BlockItem {
                                        ref_key: symbol.key.clone(),
                                        ref_idx: symbol.idx.clone(),
                                        def_key: key.clone(),
                                        def_idx: idx,
                                    });
                                    current = parent;
                                    levels += 1;
                                }
                            });
                    }
                    Some(
                        "table.get" | "table.set" | "table.size" | "table.grow" | "table.fill"
                        | "table.copy",
                    ) => {
                        let Some(region) = node
                            .ancestors()
                            .find(|node| node.kind() == SyntaxKind::MODULE)
                            .map(SymbolItemKey::from)
                        else {
                            continue;
                        };
                        symbols.extend(node.children().filter_map(|node| {
                            create_ref_symbol(db, node, region.clone(), SymbolItemKind::TableRef)
                        }));
                    }
                    Some("table.init") => {
                        let Some(region) = node
                            .ancestors()
                            .find(|node| node.kind() == SyntaxKind::MODULE)
                            .map(SymbolItemKey::from)
                        else {
                            continue;
                        };
                        let mut children = node.children();
                        if let Some(symbol) = children.next().and_then(|node| {
                            create_ref_symbol(db, node, region.clone(), SymbolItemKind::TableRef)
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
                    .map(SymbolItemKey::from)
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
                    .map(SymbolItemKey::from)
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
                    module_field_id,
                    SymbolItemKind::MemoryDef,
                ) {
                    symbols.push(symbol);
                }
                module_field_id += 1;
            }
            SyntaxKind::MODULE_FIELD_TABLE => {
                if let Some(symbol) = create_parent_based_symbol(
                    db,
                    node.clone(),
                    module_field_id,
                    SymbolItemKind::TableDef,
                ) {
                    symbols.push(symbol);
                }
                module_field_id += 1;
            }
            SyntaxKind::EXPORT_DESC_MEMORY => {
                if let Some(symbol) = node
                    .ancestors()
                    .find(|node| node.kind() == SyntaxKind::MODULE)
                    .map(SymbolItemKey::from)
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
                    .map(SymbolItemKey::from)
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
                    create_import_desc_symbol(db, &node, module_field_id, SymbolItemKind::Func)
                {
                    symbols.push(symbol);
                }
                module_field_id += 1;
            }
            SyntaxKind::IMPORT_DESC_TABLE_TYPE => {
                if let Some(symbol) =
                    create_import_desc_symbol(db, &node, module_field_id, SymbolItemKind::TableDef)
                {
                    symbols.push(symbol);
                }
                module_field_id += 1;
            }
            SyntaxKind::IMPORT_DESC_MEMORY_TYPE => {
                if let Some(symbol) =
                    create_import_desc_symbol(db, &node, module_field_id, SymbolItemKind::MemoryDef)
                {
                    symbols.push(symbol);
                }
                module_field_id += 1;
            }
            SyntaxKind::IMPORT_DESC_GLOBAL_TYPE => {
                if let Some(symbol) =
                    create_import_desc_symbol(db, &node, module_field_id, SymbolItemKind::GlobalDef)
                {
                    symbols.push(symbol);
                }
                module_field_id += 1;
            }
            SyntaxKind::MEM_USE => {
                if let Some(symbol) = node
                    .ancestors()
                    .find(|node| node.kind() == SyntaxKind::MODULE)
                    .map(SymbolItemKey::from)
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
                        module: SyntaxNodePtr::new(&module),
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
    pub fn find_defs(&self, key: &SymbolItemKey) -> Option<impl Iterator<Item = &SymbolItem>> {
        self.symbols
            .iter()
            .find(|symbol| &symbol.key == key)
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

    pub fn find_param_def(&self, key: &SymbolItemKey) -> Option<&SymbolItem> {
        self.find_local_ref(key).and_then(|local| {
            self.symbols.iter().find(|symbol| {
                symbol.region == local.region
                    && symbol.kind == SymbolItemKind::Param
                    && local.idx.is_defined_by(&symbol.idx)
            })
        })
    }

    pub fn find_param_or_local_def(&self, key: &SymbolItemKey) -> Option<&SymbolItem> {
        self.find_local_ref(key).and_then(|local| {
            self.symbols.iter().find(|symbol| {
                symbol.region == local.region
                    && matches!(symbol.kind, SymbolItemKind::Param | SymbolItemKind::Local)
                    && local.idx.is_defined_by(&symbol.idx)
            })
        })
    }

    fn find_local_ref(&self, key: &SymbolItemKey) -> Option<&SymbolItem> {
        self.symbols
            .iter()
            .find(|symbol| symbol.kind == SymbolItemKind::LocalRef && &symbol.key == key)
    }

    pub fn find_block_def(&self, key: &SymbolItemKey) -> Option<&SymbolItemKey> {
        self.find_block_ref(key).and_then(|block_ref| {
            self.blocks
                .iter()
                .find(|block| *key == block.ref_key && block_ref.idx.is_defined_by(&block.def_idx))
                .map(|block| &block.def_key)
        })
    }

    pub fn find_block_ref(&self, key: &SymbolItemKey) -> Option<&SymbolItem> {
        self.symbols
            .iter()
            .find(|symbol| symbol.kind == SymbolItemKind::BlockRef && &symbol.key == key)
    }

    pub fn get_declared(
        &self,
        node: SyntaxNode,
        kind: SymbolItemKind,
    ) -> impl Iterator<Item = &SymbolItem> {
        let key = node.into();
        self.symbols
            .iter()
            .filter(move |symbol| symbol.kind == kind && symbol.region == key)
    }

    pub fn find_block_references<'a>(
        &'a self,
        def_key: &'a SymbolItemKey,
        with_decl: bool,
    ) -> impl Iterator<Item = &'a SymbolItem> {
        if with_decl {
            self.symbols.iter().find(|symbol| symbol.key == *def_key)
        } else {
            None
        }
        .into_iter()
        .chain(
            self.blocks
                .iter()
                .filter(|block| {
                    block.def_key == *def_key && block.ref_idx.is_defined_by(&block.def_idx)
                })
                .filter_map(|block| {
                    self.symbols
                        .iter()
                        .find(|symbol| symbol.key == block.ref_key)
                }),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SymbolItemKey {
    pub ptr: SyntaxNodePtr<WatLanguage>,
}
impl From<SyntaxNode> for SymbolItemKey {
    fn from(node: SyntaxNode) -> Self {
        SymbolItemKey {
            ptr: SyntaxNodePtr::new(&node),
        }
    }
}

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
    pub ref_idx: Idx,
    pub def_key: SymbolItemKey,
    pub def_idx: Idx,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportItem {
    pub name: String, // with double quotes
    pub range: TextRange,
    pub module: SyntaxNodePtr<WatLanguage>,
}
