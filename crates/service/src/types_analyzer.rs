use rowan::{
    ast::{support::children, AstNode},
    GreenNode, SyntaxNode,
};
use std::fmt;
use wat_syntax::ast::ValType as AstValType;

#[salsa::query_group(TypesAnalyzer)]
pub(crate) trait TypesAnalyzerCtx {
    #[salsa::memoized]
    fn extract_types(&self, node: GreenNode) -> Vec<ValType>;
}
fn extract_types(_: &dyn TypesAnalyzerCtx, node: GreenNode) -> Vec<ValType> {
    let root = SyntaxNode::new_root(node);
    if let Some(ty) = AstValType::cast(root.clone()) {
        vec![ValType::from(ty)]
    } else {
        children::<AstValType>(&root).map(ValType::from).collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ValType {
    I32,
    I64,
    F32,
    F64,
    V128,
    FuncRef,
    ExternRef,
}

impl fmt::Display for ValType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValType::I32 => write!(f, "i32"),
            ValType::I64 => write!(f, "i64"),
            ValType::F32 => write!(f, "f32"),
            ValType::F64 => write!(f, "f64"),
            ValType::V128 => write!(f, "v128"),
            ValType::FuncRef => write!(f, "funcref"),
            ValType::ExternRef => write!(f, "externref"),
        }
    }
}

impl From<AstValType> for ValType {
    fn from(value: AstValType) -> Self {
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
