use ahash::AHashMap;
use lsp_types::Uri;
use rowan::ast::AstPtr;
use wat_syntax::ast::{Module, ModuleField, ModuleFieldFunc};

#[derive(Clone, Debug, Default)]
pub struct SymbolTables(AHashMap<Uri, SymbolTable>);
#[comemo::track]
impl SymbolTables {
    pub fn read(&self, uri: &Uri) -> SymbolTable {
        self.0.get(uri).cloned().unwrap_or_default()
    }
}
impl SymbolTables {
    pub fn write(&mut self, uri: Uri, symbol_table: SymbolTable) {
        self.0.insert(uri, symbol_table);
    }
    pub fn remove(&mut self, uri: &Uri) {
        self.0.remove(uri);
    }
}

#[derive(Clone, Debug, Default, Hash)]
pub struct SymbolTable {
    pub functions: Vec<Function>,
}
impl SymbolTable {
    #[comemo::memoize]
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

#[derive(Clone, Debug, Hash)]
pub struct Idx {
    pub num: usize,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Hash)]
pub struct Function {
    pub node: AstPtr<ModuleFieldFunc>,
    pub idx: Idx,
    pub params: Vec<Param>,
    pub results: Vec<Result>,
}
impl Function {
    pub fn new(id: usize, func: ModuleFieldFunc) -> Self {
        let idx = if let Some(token) = func.ident_token() {
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
            node: AstPtr::new(&func),
            idx,
            params,
            results,
        }
    }
}

#[derive(Clone, Debug, Hash)]
pub struct Param {
    pub ty: ValType,
    pub idx: Idx,
}

#[derive(Clone, Debug, Hash)]
pub struct Result {
    pub ty: ValType,
    pub idx: Idx,
}

#[derive(Clone, Debug, Hash)]
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
