use crate::{files::FilesCtx, InternUri};
use rowan::{
    ast::{support::token, AstNode, SyntaxNodePtr},
    GreenNode,
};
use wat_syntax::{
    ast::{ModuleFieldFunc, PlainInstr},
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
                symbols.push(SymbolItem {
                    key: node.into(),
                    parent: None,
                    kind: SymbolItemKind::Module,
                });
            }
            SyntaxKind::MODULE_FIELD_FUNC => {
                let current_id = module_field_id;
                module_field_id += 1;
                symbols.push(SymbolItem {
                    key: node.clone().into(),
                    parent: node.parent().map(SymbolItemKey::from),
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
                        symbols.push(SymbolItem {
                            key: param.syntax().to_owned().into(),
                            parent: Some(node.clone().into()),
                            kind: SymbolItemKind::Param(DefIdx {
                                num: i,
                                name: param.ident_token().map(|token| token.text().to_string()),
                            }),
                        });
                        i + 1
                    });
                func.locals().fold(local_index, |i, local| {
                    symbols.push(SymbolItem {
                        key: local.syntax().to_owned().into(),
                        parent: Some(node.clone().into()),
                        kind: SymbolItemKind::Local(DefIdx {
                            num: i,
                            name: local.ident_token().map(|token| token.text().to_string()),
                        }),
                    });
                    i + 1
                });
            }
            SyntaxKind::MODULE_FIELD_TYPE => {
                let current_id = module_field_id;
                module_field_id += 1;
                symbols.push(SymbolItem {
                    key: node.clone().into(),
                    parent: node.parent().map(SymbolItemKey::from),
                    kind: SymbolItemKind::Type(DefIdx {
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
                    Some("call") => {
                        let Some(parent) = node
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
                                    parent: Some(parent.clone()),
                                    kind: SymbolItemKind::Call(RefIdx::Num(idx)),
                                })
                                .or_else(|| {
                                    operand.ident().map(|idx| SymbolItem {
                                        key: operand.syntax().clone().into(),
                                        parent: Some(parent.clone()),
                                        kind: SymbolItemKind::Call(RefIdx::Name(
                                            idx.text().to_string(),
                                        )),
                                    })
                                })
                        }));
                    }
                    Some("local.get" | "local.set" | "local.tee") => {
                        let Some(parent) = node
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
                                    parent: Some(parent.clone()),
                                    kind: SymbolItemKind::LocalRef(RefIdx::Num(idx)),
                                })
                                .or_else(|| {
                                    operand.ident().map(|idx| SymbolItem {
                                        key: operand.syntax().clone().into(),
                                        parent: Some(parent.clone()),
                                        kind: SymbolItemKind::LocalRef(RefIdx::Name(
                                            idx.text().to_string(),
                                        )),
                                    })
                                })
                        }));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    SymbolTable { symbols }
}
impl SymbolTable {
    pub fn find_func_defs(
        &self,
        call: &SymbolItemKey,
    ) -> Option<impl Iterator<Item = &SymbolItem> + Clone> {
        self.symbols
            .iter()
            .find_map(|symbol| match symbol {
                SymbolItem {
                    kind: SymbolItemKind::Call(idx),
                    key,
                    ..
                } if key == call => Some((symbol, idx)),
                _ => None,
            })
            .and_then(|(symbol, idx)| symbol.parent.as_ref().map(|parent| (parent, idx)))
            .map(|(module_key, idx)| {
                self.symbols.iter().filter(move |symbol| {
                    symbol
                        .parent
                        .as_ref()
                        .is_some_and(|parent| parent == module_key)
                        && matches!(&symbol.kind, SymbolItemKind::Func(func_idx) if idx == func_idx)
                })
            })
    }

    pub fn find_param_def(&self, local_ref: &SymbolItemKey) -> Option<&SymbolItem> {
        self.find_local_ref(local_ref)
            .and_then(|(symbol, idx)| symbol.parent.as_ref().map(|parent| (parent, idx)))
            .and_then(|(func_key, idx)| {
                self.symbols.iter().find(|symbol| {
                    symbol
                        .parent
                        .as_ref()
                        .is_some_and(|parent| parent == func_key)
                        && matches!(&symbol.kind, SymbolItemKind::Param(param_idx) if idx == param_idx)
                })
            })
    }

    pub fn find_local_def(&self, local_ref: &SymbolItemKey) -> Option<&SymbolItem> {
        self.find_local_ref(local_ref)
            .and_then(|(symbol, idx)| symbol.parent.as_ref().map(|parent| (parent, idx)))
            .and_then(|(func_key, idx)| {
                self.symbols.iter().find(|symbol| {
                    symbol
                        .parent
                        .as_ref()
                        .is_some_and(|parent| parent == func_key)
                        && matches!(&symbol.kind, SymbolItemKind::Local(local_idx) if idx == local_idx)
                })
            })
    }

    fn find_local_ref(&self, local_ref: &SymbolItemKey) -> Option<(&SymbolItem, &RefIdx)> {
        self.symbols.iter().find_map(|symbol| match symbol {
            SymbolItem {
                kind: SymbolItemKind::LocalRef(idx),
                key,
                ..
            } if key == local_ref => Some((symbol, idx)),
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
    pub parent: Option<SymbolItemKey>,
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
}

#[derive(Clone, Debug)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
    V128,
    FuncRef,
    ExternRef,
}
impl From<wat_syntax::ast::ValType> for ValType {
    fn from(value: wat_syntax::ast::ValType) -> Self {
        if let Some(num_type) = value.num_type() {
            match num_type.text() {
                "i32" => ValType::I32,
                "i64" => ValType::I64,
                "f32" => ValType::F32,
                "f64" => ValType::F64,
                _ => unreachable!("unsupported numtype"),
            }
        } else if value.vec_type().is_some() {
            ValType::V128
        } else if let Some(ref_type) = value.ref_type() {
            match ref_type.text() {
                "funcref" => ValType::FuncRef,
                "externref" => ValType::ExternRef,
                _ => unreachable!("unsupported reftype"),
            }
        } else {
            unreachable!("unsupported valtype");
        }
    }
}
