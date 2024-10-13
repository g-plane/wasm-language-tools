use crate::{files::FilesCtx, InternUri};
use rowan::{
    ast::{
        support::{child, token},
        AstNode, SyntaxNodePtr,
    },
    GreenNode,
};
use wat_syntax::{
    ast::{Index, ModuleFieldFunc, PlainInstr},
    SyntaxKind, SyntaxNode, WatLanguage,
};

#[salsa::query_group(SymbolTables)]
pub(crate) trait SymbolTablesCtx: FilesCtx {
    #[salsa::memoized]
    #[salsa::invoke(create_symbol_table)]
    fn symbol_table(&self, uri: InternUri) -> SymbolTable;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefIdx {
    pub num: u32,
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RefIdx {
    Num(u32),
    Name(String),
}

impl PartialEq<u32> for RefIdx {
    fn eq(&self, other: &u32) -> bool {
        match self {
            RefIdx::Num(num) => num == other,
            RefIdx::Name(..) => false,
        }
    }
}
impl PartialEq<DefIdx> for RefIdx {
    fn eq(&self, other: &DefIdx) -> bool {
        match self {
            RefIdx::Num(num) => *num == other.num,
            RefIdx::Name(name) => other.name.as_ref().is_some_and(|s| name == s),
        }
    }
}
impl PartialEq<RefIdx> for DefIdx {
    fn eq(&self, other: &RefIdx) -> bool {
        match other {
            RefIdx::Num(num) => self.num == *num,
            RefIdx::Name(name) => self.name.as_ref().is_some_and(|s| name == s),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SymbolTable {
    pub symbols: Vec<SymbolItem>,
}
fn create_symbol_table(db: &dyn SymbolTablesCtx, uri: InternUri) -> SymbolTable {
    let root = db.root(uri);
    let mut module_field_id = 0;
    let mut symbols = Vec::with_capacity(2);
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
                    key: node.into(),
                    region,
                    kind: SymbolItemKind::Module,
                });
            }
            SyntaxKind::MODULE_FIELD_FUNC => {
                let current_id = module_field_id;
                module_field_id += 1;
                symbols.push(SymbolItem {
                    key: node.clone().into(),
                    region: if let Some(parent) = node.parent() {
                        parent.into()
                    } else {
                        continue;
                    },
                    kind: SymbolItemKind::Func(DefIdx {
                        num: current_id,
                        name: token(&node, SyntaxKind::IDENT).map(|token| token.text().to_string()),
                    }),
                });
                let Some(func) = ModuleFieldFunc::cast(node.clone()) else {
                    continue;
                };
                let local_index = func
                    .type_use()
                    .iter()
                    .flat_map(|type_use| type_use.params())
                    .fold(0, |i, param| {
                        if let Some(ident) = param.ident_token() {
                            symbols.push(SymbolItem {
                                key: param.syntax().to_owned().into(),
                                region: node.clone().into(),
                                kind: SymbolItemKind::Param(DefIdx {
                                    num: i,
                                    name: Some(ident.text().to_string()),
                                }),
                            });
                            i + 1
                        } else {
                            param.val_types().fold(i, |i, val_type| {
                                symbols.push(SymbolItem {
                                    key: val_type.syntax().to_owned().into(),
                                    region: node.clone().into(),
                                    kind: SymbolItemKind::Param(DefIdx { num: i, name: None }),
                                });
                                i + 1
                            })
                        }
                    });
                func.locals().fold(local_index, |i, local| {
                    if let Some(ident) = local.ident_token() {
                        symbols.push(SymbolItem {
                            key: local.syntax().to_owned().into(),
                            region: node.clone().into(),
                            kind: SymbolItemKind::Local(DefIdx {
                                num: i,
                                name: Some(ident.text().to_string()),
                            }),
                        });
                        i + 1
                    } else {
                        local.val_types().fold(i, |i, val_type| {
                            symbols.push(SymbolItem {
                                key: val_type.syntax().to_owned().into(),
                                region: node.clone().into(),
                                kind: SymbolItemKind::Local(DefIdx { num: i, name: None }),
                            });
                            i + 1
                        })
                    }
                });
            }
            SyntaxKind::MODULE_FIELD_TYPE => {
                let current_id = module_field_id;
                module_field_id += 1;
                symbols.push(SymbolItem {
                    key: node.clone().into(),
                    region: if let Some(parent) = node.parent() {
                        parent.into()
                    } else {
                        continue;
                    },
                    kind: SymbolItemKind::Type(DefIdx {
                        num: current_id,
                        name: token(&node, SyntaxKind::IDENT).map(|token| token.text().to_string()),
                    }),
                });
            }
            SyntaxKind::MODULE_FIELD_GLOBAL => {
                let current_id = module_field_id;
                module_field_id += 1;
                symbols.push(SymbolItem {
                    key: node.clone().into(),
                    region: if let Some(parent) = node.parent() {
                        parent.into()
                    } else {
                        continue;
                    },
                    kind: SymbolItemKind::GlobalDef(DefIdx {
                        num: current_id,
                        name: token(&node, SyntaxKind::IDENT).map(|token| token.text().to_string()),
                    }),
                });
            }
            SyntaxKind::PLAIN_INSTR => {
                let Some(instr) = PlainInstr::cast(node.clone()) else {
                    continue;
                };
                match instr.instr_name().as_ref().map(|token| token.text()) {
                    Some("call" | "ref.func") => {
                        let Some(region) = node
                            .ancestors()
                            .find(|node| node.kind() == SyntaxKind::MODULE)
                            .map(SymbolItemKey::from)
                        else {
                            continue;
                        };
                        symbols.extend(instr.operands().filter_map(|operand| {
                            operand
                                .int()
                                .and_then(|token| token.text().parse().ok())
                                .map(|idx| SymbolItem {
                                    key: operand.syntax().clone().into(),
                                    region: region.clone(),
                                    kind: SymbolItemKind::Call(RefIdx::Num(idx)),
                                })
                                .or_else(|| {
                                    operand.ident().map(|idx| SymbolItem {
                                        key: operand.syntax().clone().into(),
                                        region: region.clone(),
                                        kind: SymbolItemKind::Call(RefIdx::Name(
                                            idx.text().to_string(),
                                        )),
                                    })
                                })
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
                        symbols.extend(instr.operands().filter_map(|operand| {
                            operand
                                .int()
                                .and_then(|token| token.text().parse().ok())
                                .map(|idx| SymbolItem {
                                    key: operand.syntax().clone().into(),
                                    region: region.clone(),
                                    kind: SymbolItemKind::LocalRef(RefIdx::Num(idx)),
                                })
                                .or_else(|| {
                                    operand.ident().map(|idx| SymbolItem {
                                        key: operand.syntax().clone().into(),
                                        region: region.clone(),
                                        kind: SymbolItemKind::LocalRef(RefIdx::Name(
                                            idx.text().to_string(),
                                        )),
                                    })
                                })
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
                        symbols.extend(instr.operands().filter_map(|operand| {
                            operand
                                .int()
                                .and_then(|token| token.text().parse().ok())
                                .map(|idx| SymbolItem {
                                    key: operand.syntax().clone().into(),
                                    region: region.clone(),
                                    kind: SymbolItemKind::GlobalRef(RefIdx::Num(idx)),
                                })
                                .or_else(|| {
                                    operand.ident().map(|idx| SymbolItem {
                                        key: operand.syntax().clone().into(),
                                        region: region.clone(),
                                        kind: SymbolItemKind::GlobalRef(RefIdx::Name(
                                            idx.text().to_string(),
                                        )),
                                    })
                                })
                        }));
                    }
                    _ => {}
                }
            }
            SyntaxKind::MODULE_FIELD_START => {
                let Some(region) = node.parent().map(SymbolItemKey::from) else {
                    continue;
                };
                if let Some(index) = child::<Index>(&node) {
                    if let Some(ident) = index.ident_token() {
                        symbols.push(SymbolItem {
                            key: index.syntax().clone().into(),
                            region,
                            kind: SymbolItemKind::Call(RefIdx::Name(ident.text().to_string())),
                        });
                    } else if let Some(unsigned_int) = index
                        .unsigned_int_token()
                        .and_then(|token| token.text().parse().ok())
                    {
                        symbols.push(SymbolItem {
                            key: index.syntax().clone().into(),
                            region,
                            kind: SymbolItemKind::Call(RefIdx::Num(unsigned_int)),
                        });
                    }
                }
            }
            SyntaxKind::TYPE_USE => {
                let Some(region) = node
                    .ancestors()
                    .find(|node| node.kind() == SyntaxKind::MODULE)
                    .map(SymbolItemKey::from)
                else {
                    continue;
                };
                if let Some(index) = child::<Index>(&node) {
                    if let Some(ident) = index.ident_token() {
                        symbols.push(SymbolItem {
                            key: index.syntax().clone().into(),
                            region,
                            kind: SymbolItemKind::TypeUse(RefIdx::Name(ident.text().to_string())),
                        });
                    } else if let Some(unsigned_int) = index
                        .unsigned_int_token()
                        .and_then(|token| token.text().parse().ok())
                    {
                        symbols.push(SymbolItem {
                            key: index.syntax().clone().into(),
                            region,
                            kind: SymbolItemKind::TypeUse(RefIdx::Num(unsigned_int)),
                        });
                    }
                }
            }
            _ => {}
        }
    }
    SymbolTable { symbols }
}
impl SymbolTable {
    pub fn find_func_defs(&self, key: &SymbolItemKey) -> Option<impl Iterator<Item = &SymbolItem>> {
        self.symbols
            .iter()
            .find_map(|symbol| match symbol {
                SymbolItem {
                    kind: SymbolItemKind::Call(idx),
                    key: call_key,
                    ..
                } if call_key == key => Some((symbol, idx)),
                _ => None,
            })
            .map(|(call, ref_idx)| {
                self.symbols.iter().filter(move |symbol| {
                    symbol.region == call.region
                        && match &symbol.kind {
                            SymbolItemKind::Func(def_idx) => ref_idx == def_idx,
                            _ => false,
                        }
                })
            })
    }

    pub fn find_param_def(&self, key: &SymbolItemKey) -> Option<&SymbolItem> {
        self.find_local_ref(key).and_then(|(local, ref_idx)| {
            self.symbols.iter().find(|symbol| {
                symbol.region == local.region
                    && match &symbol.kind {
                        SymbolItemKind::Param(def_idx) => ref_idx == def_idx,
                        _ => false,
                    }
            })
        })
    }

    pub fn find_local_def(&self, key: &SymbolItemKey) -> Option<&SymbolItem> {
        self.find_local_ref(key).and_then(|(local, ref_idx)| {
            self.symbols.iter().find(|symbol| {
                symbol.region == local.region
                    && match &symbol.kind {
                        SymbolItemKind::Local(def_idx) => ref_idx == def_idx,
                        _ => false,
                    }
            })
        })
    }

    fn find_local_ref(&self, key: &SymbolItemKey) -> Option<(&SymbolItem, &RefIdx)> {
        self.symbols.iter().find_map(|symbol| match symbol {
            SymbolItem {
                kind: SymbolItemKind::LocalRef(idx),
                key: local_ref_key,
                ..
            } if local_ref_key == key => Some((symbol, idx)),
            _ => None,
        })
    }

    pub fn find_type_use_defs(
        &self,
        key: &SymbolItemKey,
    ) -> Option<impl Iterator<Item = &SymbolItem>> {
        self.symbols
            .iter()
            .find_map(|symbol| match symbol {
                SymbolItem {
                    kind: SymbolItemKind::TypeUse(idx),
                    key: type_use_key,
                    ..
                } if type_use_key == key => Some((symbol, idx)),
                _ => None,
            })
            .map(|(type_use, ref_idx)| {
                self.symbols.iter().filter(move |symbol| {
                    symbol.region == type_use.region
                        && match &symbol.kind {
                            SymbolItemKind::Type(def_idx) => ref_idx == def_idx,
                            _ => false,
                        }
                })
            })
    }

    pub fn find_global_defs(
        &self,
        key: &SymbolItemKey,
    ) -> Option<impl Iterator<Item = &SymbolItem>> {
        self.symbols
            .iter()
            .find_map(|symbol| match symbol {
                SymbolItem {
                    kind: SymbolItemKind::GlobalRef(idx),
                    key: global_ref_key,
                    ..
                } if global_ref_key == key => Some((symbol, idx)),
                _ => None,
            })
            .map(|(global, ref_idx)| {
                self.symbols.iter().filter(move |symbol| {
                    symbol.region == global.region
                        && match &symbol.kind {
                            SymbolItemKind::GlobalDef(def_idx) => ref_idx == def_idx,
                            _ => false,
                        }
                })
            })
    }

    pub fn get_declared_params_and_locals(
        &self,
        node: SyntaxNode,
    ) -> impl Iterator<Item = (&SymbolItem, &DefIdx)> {
        let key = node.into();
        self.symbols
            .iter()
            .filter_map(move |symbol| match &symbol.kind {
                SymbolItemKind::Param(idx) | SymbolItemKind::Local(idx) if symbol.region == key => {
                    Some((symbol, idx))
                }
                _ => None,
            })
    }

    pub fn get_declared_functions(
        &self,
        node: SyntaxNode,
    ) -> impl Iterator<Item = (&SymbolItem, &DefIdx)> {
        debug_assert_eq!(node.kind(), SyntaxKind::MODULE);
        let key = node.into();
        self.symbols
            .iter()
            .filter_map(move |symbol| match &symbol.kind {
                SymbolItemKind::Func(idx) if symbol.region == key => Some((symbol, idx)),
                _ => None,
            })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SymbolItemKey {
    pub ptr: SyntaxNodePtr<WatLanguage>,
    pub green: GreenNode,
}
impl From<SyntaxNode> for SymbolItemKey {
    fn from(node: SyntaxNode) -> Self {
        SymbolItemKey {
            ptr: SyntaxNodePtr::new(&node),
            green: node.green().into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SymbolItem {
    pub key: SymbolItemKey,
    pub region: SymbolItemKey,
    pub kind: SymbolItemKind,
}
impl PartialEq for SymbolItem {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl Eq for SymbolItem {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolItemKind {
    Module,
    Func(DefIdx),
    Param(DefIdx),
    Local(DefIdx),
    Call(RefIdx),
    LocalRef(RefIdx),
    Type(DefIdx),
    TypeUse(RefIdx),
    GlobalDef(DefIdx),
    GlobalRef(RefIdx),
}
