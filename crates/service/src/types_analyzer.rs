use crate::{
    binder::{DefIdx, SymbolItem, SymbolItemKind, SymbolTablesCtx},
    files::FilesCtx,
    helpers, InternUri,
};
use rowan::{
    ast::{
        support::{child, children},
        AstNode,
    },
    GreenNode, Language, NodeOrToken, SyntaxNode,
};
use std::fmt;
use wat_syntax::{
    ast::{Param, Result, TypeUse, ValType as AstValType},
    SyntaxKind, WatLanguage,
};

#[salsa::query_group(TypesAnalyzer)]
pub(crate) trait TypesAnalyzerCtx: FilesCtx + SymbolTablesCtx {
    #[salsa::memoized]
    fn extract_type(&self, node: GreenNode) -> Option<ValType>;
    #[salsa::memoized]
    fn extract_global_type(&self, node: GreenNode) -> Option<ValType>;
    #[salsa::memoized]
    fn extract_func_sig(&self, node: GreenNode) -> FuncSig;

    #[salsa::memoized]
    fn render_func_header(&self, uri: InternUri, symbol: SymbolItem) -> String;
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

fn render_func_header(db: &dyn TypesAnalyzerCtx, uri: InternUri, symbol: SymbolItem) -> String {
    debug_assert!(matches!(symbol.kind, SymbolItemKind::Func(..)));
    let mut content = "(func".to_string();
    if let SymbolItemKind::Func(DefIdx {
        name: Some(name), ..
    }) = symbol.kind
    {
        content.push(' ');
        content.push_str(&db.lookup_ident(name));
    }
    if let Some(type_use) = symbol.green.children().find_map(|child| match child {
        NodeOrToken::Node(node) if node.kind() == SyntaxKind::TYPE_USE.into() => Some(node),
        _ => None,
    }) {
        if type_use.children().any(|child| {
            let kind = child.kind();
            kind == SyntaxKind::PARAM.into() || kind == SyntaxKind::RESULT.into()
        }) {
            let sig = db.extract_func_sig(type_use.to_owned());
            if !sig.params.is_empty() || !sig.results.is_empty() {
                content.push(' ');
                content.push_str(&sig.to_string());
            }
        } else {
            let node = symbol.key.ptr.to_node(&db.root(uri));
            let symbol_table = db.symbol_table(uri);
            if let Some(func_type) = child::<TypeUse>(&node)
                .and_then(|type_use| type_use.index())
                .and_then(|idx| symbol_table.find_type_use_defs(&idx.syntax().clone().into()))
                .and_then(|mut symbols| symbols.next())
                .and_then(|symbol| helpers::ast::find_func_type_of_type_def(&symbol.green))
            {
                let sig = db.extract_func_sig(func_type);
                if !sig.params.is_empty() || !sig.results.is_empty() {
                    content.push(' ');
                    content.push_str(&sig.to_string());
                }
            }
        }
    }
    content.push(')');
    content
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
