use crate::{files::FilesCtx, InternUri};
use rowan::{
    ast::{support::token, AstNode, SyntaxNodePtr},
    GreenNode,
};
use salsa::{InternId, InternKey};
use std::{hash::Hash, rc::Rc};
use wat_syntax::{
    ast::{ModuleFieldFunc, PlainInstr},
    SyntaxKind, SyntaxNode, WatLanguage,
};

#[salsa::query_group(SymbolTables)]
pub(crate) trait SymbolTablesCtx: FilesCtx {
    #[salsa::memoized]
    #[salsa::invoke(create_symbol_table)]
    fn symbol_table(&self, uri: InternUri) -> Rc<SymbolTable>;

    #[salsa::interned]
    fn ident(&self, ident: String) -> InternIdent;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DefIdx {
    pub num: u32,
    pub name: Option<InternIdent>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RefIdx {
    Num(u32),
    Name(InternIdent),
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
fn create_symbol_table(db: &dyn SymbolTablesCtx, uri: InternUri) -> Rc<SymbolTable> {
    fn create_module_field_symbol(
        db: &dyn SymbolTablesCtx,
        node: SyntaxNode,
        id: u32,
        kind: fn(DefIdx) -> SymbolItemKind,
    ) -> Option<SymbolItem> {
        node.parent().map(|parent| SymbolItem {
            key: node.clone().into(),
            green: node.green().into(),
            region: parent.into(),
            kind: kind(DefIdx {
                num: id,
                name: token(&node, SyntaxKind::IDENT)
                    .map(|token| db.ident(token.text().to_string())),
            }),
        })
    }
    fn create_ref_symbol(
        db: &dyn SymbolTablesCtx,
        node: SyntaxNode,
        region: SymbolItemKey,
        kind: fn(RefIdx) -> SymbolItemKind,
    ) -> Option<SymbolItem> {
        token(&node, SyntaxKind::IDENT)
            .map(|ident| SymbolItem {
                key: node.clone().into(),
                green: node.green().into(),
                region: region.clone(),
                kind: kind(RefIdx::Name(db.ident(ident.text().to_string()))),
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
                        kind: kind(RefIdx::Num(num)),
                    })
            })
    }

    let root = SyntaxNode::new_root(db.root(uri));
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
                    green: node.green().into(),
                    key: node.into(),
                    region,
                    kind: SymbolItemKind::Module,
                });
            }
            SyntaxKind::MODULE_FIELD_FUNC => {
                if let Some(symbol) = create_module_field_symbol(
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
                                kind: SymbolItemKind::Param(DefIdx {
                                    num: i,
                                    name: Some(db.ident(ident.text().to_string())),
                                }),
                            });
                            i + 1
                        } else {
                            param.val_types().fold(i, |i, val_type| {
                                let val_type = val_type.syntax();
                                symbols.push(SymbolItem {
                                    key: val_type.to_owned().into(),
                                    green: val_type.green().into(),
                                    region: node.clone().into(),
                                    kind: SymbolItemKind::Param(DefIdx { num: i, name: None }),
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
                            kind: SymbolItemKind::Local(DefIdx {
                                num: i,
                                name: Some(db.ident(ident.text().to_string())),
                            }),
                        });
                        i + 1
                    } else {
                        local.val_types().fold(i, |i, val_type| {
                            let val_type = val_type.syntax();
                            symbols.push(SymbolItem {
                                key: val_type.to_owned().into(),
                                green: val_type.green().into(),
                                region: node.clone().into(),
                                kind: SymbolItemKind::Local(DefIdx { num: i, name: None }),
                            });
                            i + 1
                        })
                    }
                });
            }
            SyntaxKind::MODULE_FIELD_TYPE => {
                if let Some(symbol) = create_module_field_symbol(
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
                if let Some(symbol) = create_module_field_symbol(
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
                    _ => {}
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
                if let Some(symbol) = create_module_field_symbol(
                    db,
                    node.clone(),
                    module_field_id,
                    SymbolItemKind::MemoryDef,
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
            _ => {}
        }
    }
    Rc::new(SymbolTable { symbols })
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

    pub fn find_param_or_local_def(&self, key: &SymbolItemKey) -> Option<&SymbolItem> {
        self.find_local_ref(key).and_then(|(local, ref_idx)| {
            self.symbols.iter().find(|symbol| {
                symbol.region == local.region
                    && match &symbol.kind {
                        SymbolItemKind::Param(def_idx) | SymbolItemKind::Local(def_idx) => {
                            ref_idx == def_idx
                        }
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

    pub fn find_memory_defs(
        &self,
        key: &SymbolItemKey,
    ) -> Option<impl Iterator<Item = &SymbolItem>> {
        self.symbols
            .iter()
            .find_map(|symbol| match symbol {
                SymbolItem {
                    kind: SymbolItemKind::MemoryRef(idx),
                    key: memory_ref_key,
                    ..
                } if memory_ref_key == key => Some((symbol, idx)),
                _ => None,
            })
            .map(|(memory, ref_idx)| {
                self.symbols.iter().filter(move |symbol| {
                    symbol.region == memory.region
                        && match &symbol.kind {
                            SymbolItemKind::MemoryDef(def_idx) => ref_idx == def_idx,
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

    pub fn get_declared_func_types(
        &self,
        node: SyntaxNode,
    ) -> impl Iterator<Item = (&SymbolItem, &DefIdx)> {
        debug_assert_eq!(node.kind(), SyntaxKind::MODULE);
        let key = node.into();
        self.symbols
            .iter()
            .filter_map(move |symbol| match &symbol.kind {
                SymbolItemKind::Type(idx) if symbol.region == key => Some((symbol, idx)),
                _ => None,
            })
    }

    pub fn get_declared_globals(
        &self,
        node: SyntaxNode,
    ) -> impl Iterator<Item = (&SymbolItem, &DefIdx)> {
        debug_assert_eq!(node.kind(), SyntaxKind::MODULE);
        let key = node.into();
        self.symbols
            .iter()
            .filter_map(move |symbol| match &symbol.kind {
                SymbolItemKind::GlobalDef(idx) if symbol.region == key => Some((symbol, idx)),
                _ => None,
            })
    }

    pub fn get_declared_memories(
        &self,
        node: SyntaxNode,
    ) -> impl Iterator<Item = (&SymbolItem, &DefIdx)> {
        debug_assert_eq!(node.kind(), SyntaxKind::MODULE);
        let key = node.into();
        self.symbols
            .iter()
            .filter_map(move |symbol| match &symbol.kind {
                SymbolItemKind::MemoryDef(idx) if symbol.region == key => Some((symbol, idx)),
                _ => None,
            })
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
}
impl PartialEq for SymbolItem {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.kind == other.kind
    }
}
impl Eq for SymbolItem {}
impl Hash for SymbolItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    MemoryDef(DefIdx),
    MemoryRef(RefIdx),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct InternIdent(InternId);
impl InternKey for InternIdent {
    fn from_intern_id(v: salsa::InternId) -> Self {
        InternIdent(v)
    }
    fn as_intern_id(&self) -> InternId {
        self.0
    }
}
