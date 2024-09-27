use crate::files::FileInputCtx;
use lsp_types::Uri;
use rowan::{
    ast::{AstNode, AstPtr},
    GreenNode,
};
use wat_syntax::ast::{Module, ModuleField, ModuleFieldFunc};

#[salsa::query_group(SymbolTables)]
pub trait SymbolTablesCtx: FileInputCtx {
    #[salsa::memoized]
    #[salsa::invoke(create_symbol_table)]
    fn symbol_table(&self, uri: Uri) -> SymbolTable;
}
fn create_symbol_table(db: &dyn SymbolTablesCtx, uri: Uri) -> SymbolTable {
    db.root(uri)
        .modules()
        .next()
        .as_ref()
        .map(SymbolTable::new)
        .unwrap_or_default()
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SymbolTable {
    pub functions: Vec<Function>,
}
impl SymbolTable {
    pub fn new(module: &Module) -> SymbolTable {
        Self {
            functions: module
                .module_fields()
                .filter_map(|field| {
                    if let ModuleField::Func(func) = field {
                        Some(func)
                    } else {
                        None
                    }
                })
                .enumerate()
                .map(|(id, func)| Function::new(id, func))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Idx {
    pub num: usize,
    pub name: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub green: GreenNode,
    pub ptr: AstPtr<ModuleFieldFunc>,
    pub idx: Idx,
    pub params: Vec<Param>,
    pub results: Vec<Result>,
}
impl Function {
    pub fn new(id: usize, func: ModuleFieldFunc) -> Self {
        let idx = if let Some(token) = func.ident_token() {
            tracing::event!(tracing::Level::DEBUG, "function name: {}", token.text());
            Idx {
                num: id,
                name: Some(token.text().to_string()),
            }
        } else {
            Idx {
                num: id,
                name: None,
            }
        };

        let params = vec![];
        let mut results = vec![];
        if let Some(type_use) = func.type_use() {
            results.extend(
                type_use
                    .results()
                    .flat_map(|result| result.val_types())
                    .enumerate()
                    .map(|(id, ty)| Result {
                        ty: ty.into(),
                        idx: Idx {
                            num: id,
                            name: None,
                        },
                    }),
            );
        }
        Self {
            green: func.syntax().green().into(),
            ptr: AstPtr::new(&func),
            idx,
            params,
            results,
        }
    }
}
impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.green == other.green
    }
}
impl Eq for Function {}

#[derive(Clone, Debug)]
pub struct Param {
    pub ty: ValType,
    pub idx: Idx,
}

#[derive(Clone, Debug)]
pub struct Result {
    pub ty: ValType,
    pub idx: Idx,
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
