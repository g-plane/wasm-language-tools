use crate::{files::FilesCtx, InternUri};
use rowan::{
    ast::{support::token, SyntaxNodePtr},
    GreenNode,
};
use wat_syntax::{SyntaxKind, WatLanguage};

#[salsa::query_group(SymbolTables)]
pub(crate) trait SymbolTablesCtx: FilesCtx {
    #[salsa::memoized]
    #[salsa::invoke(create_symbol_table)]
    fn symbol_table(&self, uri: InternUri) -> SymbolTable;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Idx {
    pub num: usize,
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolTable {
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
                    key: SymbolItemKey {
                        ptr: SyntaxNodePtr::new(&node),
                        green: node.green().into(),
                    },
                    parent: None,
                    kind: SymbolItemKind::Module,
                });
            }
            SyntaxKind::MODULE_FIELD_FUNC => {
                let current_id = module_field_id;
                module_field_id += 1;
                symbols.push(SymbolItem {
                    key: SymbolItemKey {
                        ptr: SyntaxNodePtr::new(&node),
                        green: node.green().into(),
                    },
                    parent: node.parent().map(|parent| SymbolItemKey {
                        ptr: SyntaxNodePtr::new(&parent),
                        green: parent.green().into(),
                    }),
                    kind: SymbolItemKind::Func(Idx {
                        num: current_id,
                        name: token(&node, SyntaxKind::IDENT).map(|token| token.text().to_string()),
                    }),
                });
            }
            SyntaxKind::MODULE_FIELD_TYPE => {
                let current_id = module_field_id;
                module_field_id += 1;
                symbols.push(SymbolItem {
                    key: SymbolItemKey {
                        ptr: SyntaxNodePtr::new(&node),
                        green: node.green().into(),
                    },
                    parent: node.parent().map(|parent| SymbolItemKey {
                        ptr: SyntaxNodePtr::new(&parent),
                        green: parent.green().into(),
                    }),
                    kind: SymbolItemKind::Type(Idx {
                        num: current_id,
                        name: token(&node, SyntaxKind::IDENT).map(|token| token.text().to_string()),
                    }),
                });
            }
            _ => {}
        }
    }
    SymbolTable { symbols }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SymbolItemKey {
    pub ptr: SyntaxNodePtr<WatLanguage>,
    pub green: GreenNode,
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
    Func(Idx),
    Type(Idx),
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
