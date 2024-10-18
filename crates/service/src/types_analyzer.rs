use rowan::{ast::support::children, GreenNode, Language, NodeOrToken, SyntaxNode};
use std::fmt;
use wat_syntax::{
    ast::{Param, Result, ValType as AstValType},
    SyntaxKind, WatLanguage,
};

#[salsa::query_group(TypesAnalyzer)]
pub(crate) trait TypesAnalyzerCtx {
    #[salsa::memoized]
    fn extract_type(&self, node: GreenNode) -> Option<ValType>;
    #[salsa::memoized]
    fn extract_global_type(&self, node: GreenNode) -> Option<ValType>;
    #[salsa::memoized]
    fn extract_func_sig(&self, node: GreenNode) -> FuncSig;
}
fn extract_type(_: &dyn TypesAnalyzerCtx, node: GreenNode) -> Option<ValType> {
    node.clone().try_into().ok().or_else(|| {
        node.children().find_map(|child| match child {
            NodeOrToken::Node(node) if node.kind() == SyntaxKind::VAL_TYPE.into() => {
                node.to_owned().try_into().ok()
            }
            _ => None,
        })
    })
}

fn extract_global_type(db: &dyn TypesAnalyzerCtx, node: GreenNode) -> Option<ValType> {
    node.children()
        .find_map(|child| match child {
            NodeOrToken::Node(node) if node.kind() == SyntaxKind::GLOBAL_TYPE.into() => Some(node),
            _ => None,
        })
        .and_then(|global_type| db.extract_type(global_type.to_owned()))
}

fn extract_func_sig(_: &dyn TypesAnalyzerCtx, node: GreenNode) -> FuncSig {
    let root = SyntaxNode::new_root(node);
    let params = children::<Param>(&root).fold(vec![], |mut acc, param| {
        if let Some((ident, ty)) = param.ident_token().zip(param.val_types().next()) {
            acc.push((ValType::from(ty), Some(ident.text().to_string())));
        } else {
            acc.extend(
                param
                    .val_types()
                    .map(|val_type| (ValType::from(val_type), None)),
            );
        }
        acc
    });
    let results = children::<Result>(&root)
        .flat_map(|result| result.val_types())
        .map(ValType::from)
        .collect();
    FuncSig { params, results }
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

impl TryFrom<GreenNode> for ValType {
    type Error = ();
    fn try_from(node: GreenNode) -> std::result::Result<Self, Self::Error> {
        node.children()
            .find_map(|child| {
                if let NodeOrToken::Token(token) = child {
                    match WatLanguage::kind_from_raw(token.kind()) {
                        SyntaxKind::NUM_TYPE => match token.text() {
                            "i32" => Some(ValType::I32),
                            "i64" => Some(ValType::I64),
                            "f32" => Some(ValType::F32),
                            "f64" => Some(ValType::F64),
                            _ => None,
                        },
                        SyntaxKind::VEC_TYPE => Some(ValType::V128),
                        SyntaxKind::REF_TYPE => match token.text() {
                            "funcref" => Some(ValType::FuncRef),
                            "externref" => Some(ValType::ExternRef),
                            _ => None,
                        },
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .ok_or(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FuncSig {
    pub(crate) params: Vec<(ValType, Option<String>)>,
    pub(crate) results: Vec<ValType>,
}
impl fmt::Display for FuncSig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut written = false;
        self.params.iter().try_for_each(|param| {
            if written {
                write!(f, " ")?;
            }
            write!(f, "(param")?;
            if let Some(name) = &param.1 {
                write!(f, " {}", name)?;
            }
            write!(f, " {})", param.0)?;
            written = true;
            Ok(())
        })?;
        self.results.iter().try_for_each(|result| {
            if written {
                write!(f, " ")?;
            }
            write!(f, "(result {})", result)?;
            written = true;
            Ok(())
        })?;
        Ok(())
    }
}
