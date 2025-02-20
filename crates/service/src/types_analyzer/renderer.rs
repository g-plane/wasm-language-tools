use super::{
    signature::Signature,
    types::{HeapType, OperandType, ValType},
    TypesAnalyzerCtx,
};
use crate::idx::InternIdent;
use itertools::Itertools;
use std::fmt::{self, Display};
use wat_syntax::SyntaxKind;

pub(super) fn render_sig(db: &dyn TypesAnalyzerCtx, signature: Signature) -> String {
    let mut ret = String::with_capacity(signature.params.len() * 9 + signature.results.len() * 10);
    let params = signature
        .params
        .into_iter()
        .map(|(ty, name)| {
            if let Some(name) = name {
                format!("(param {} {})", db.lookup_ident(name), ty.render(db))
            } else {
                format!("(param {})", ty.render(db))
            }
        })
        .join(" ");
    ret.push_str(&params);
    let results = signature
        .results
        .into_iter()
        .map(|ty| format!("(result {})", ty.render(db)))
        .join(" ");
    if !params.is_empty() && !results.is_empty() {
        ret.push(' ');
    }
    ret.push_str(&results);
    ret
}

pub(super) fn render_compact_sig(db: &dyn TypesAnalyzerCtx, signature: Signature) -> String {
    let params = signature
        .params
        .iter()
        .map(|(ty, _)| ty.render_compact(db))
        .join(", ");
    let results = signature
        .results
        .iter()
        .map(|ty| ty.render_compact(db))
        .join(", ");
    format!("[{params}] -> [{results}]")
}

pub(super) fn render_func_header(
    db: &dyn TypesAnalyzerCtx,
    name: Option<InternIdent>,
    signature: Option<Signature>,
) -> String {
    let mut content = "(func".to_string();
    if let Some(name) = name {
        content.push(' ');
        content.push_str(&db.lookup_ident(name));
    }
    if let Some(sig) = signature {
        if !sig.params.is_empty() || !sig.results.is_empty() {
            content.push(' ');
            content.push_str(&db.render_sig(sig));
        }
    }
    content.push(')');
    content
}

pub(super) fn render_block_header(
    db: &dyn TypesAnalyzerCtx,
    kind: SyntaxKind,
    name: Option<InternIdent>,
    signature: Option<Signature>,
) -> String {
    let mut content = format!(
        "({}",
        match kind {
            SyntaxKind::BLOCK_IF => "if",
            SyntaxKind::BLOCK_LOOP => "loop",
            _ => "block",
        }
    );
    if let Some(name) = name {
        content.push(' ');
        content.push_str(&db.lookup_ident(name));
    }
    if let Some(sig) = signature {
        if !sig.params.is_empty() || !sig.results.is_empty() {
            content.push(' ');
            content.push_str(&db.render_sig(sig));
        }
    }
    content.push(')');
    content
}

impl ValType {
    pub(crate) fn render<'a>(&'a self, db: &'a dyn TypesAnalyzerCtx) -> ValTypeRender<'a, false> {
        ValTypeRender { ty: self, db }
    }

    pub(crate) fn render_compact<'a>(
        &'a self,
        db: &'a dyn TypesAnalyzerCtx,
    ) -> ValTypeRender<'a, true> {
        ValTypeRender { ty: self, db }
    }
}
pub(crate) struct ValTypeRender<'a, const COMPACT: bool> {
    ty: &'a ValType,
    db: &'a dyn TypesAnalyzerCtx,
}
impl Display for ValTypeRender<'_, false> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.ty {
            ValType::I32 => write!(f, "i32"),
            ValType::I64 => write!(f, "i64"),
            ValType::F32 => write!(f, "f32"),
            ValType::F64 => write!(f, "f64"),
            ValType::V128 => write!(f, "v128"),
            ValType::Ref(ty) => {
                write!(f, "(ref ")?;
                if ty.nullable {
                    write!(f, "null ")?;
                }
                match ty.heap_ty {
                    HeapType::Type(idx) => {
                        if let Some(name) = idx.name {
                            write!(f, "{}", self.db.lookup_ident(name))?;
                        } else if let Some(num) = idx.num {
                            write!(f, "{num}")?;
                        }
                    }
                    HeapType::Any => write!(f, "any")?,
                    HeapType::Eq => write!(f, "eq")?,
                    HeapType::I31 => write!(f, "i31")?,
                    HeapType::Struct => write!(f, "struct")?,
                    HeapType::Array => write!(f, "array")?,
                    HeapType::None => write!(f, "none")?,
                    HeapType::Func => write!(f, "func")?,
                    HeapType::NoFunc => write!(f, "nofunc")?,
                    HeapType::Extern => write!(f, "extern")?,
                    HeapType::NoExtern => write!(f, "noextern")?,
                }
                write!(f, ")")
            }
        }
    }
}
impl Display for ValTypeRender<'_, true> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.ty {
            ValType::I32 => write!(f, "i32"),
            ValType::I64 => write!(f, "i64"),
            ValType::F32 => write!(f, "f32"),
            ValType::F64 => write!(f, "f64"),
            ValType::V128 => write!(f, "v128"),
            ValType::Ref(ty) => {
                if ty.nullable {
                    write!(f, "nullable ")?;
                }
                match ty.heap_ty {
                    HeapType::Type(idx) => {
                        if let Some(name) = idx.name {
                            write!(f, "type `{}`", self.db.lookup_ident(name))?;
                        } else if let Some(num) = idx.num {
                            write!(f, "type `{num}`")?;
                        }
                    }
                    HeapType::Any => write!(f, "any")?,
                    HeapType::Eq => write!(f, "eq")?,
                    HeapType::I31 => write!(f, "i31")?,
                    HeapType::Struct => write!(f, "struct")?,
                    HeapType::Array => write!(f, "array")?,
                    HeapType::None => write!(f, "none")?,
                    HeapType::Func => write!(f, "func")?,
                    HeapType::NoFunc => write!(f, "nofunc")?,
                    HeapType::Extern => write!(f, "extern")?,
                    HeapType::NoExtern => write!(f, "noextern")?,
                }
                write!(f, " ref")
            }
        }
    }
}

impl OperandType {
    pub(crate) fn render<'a>(&'a self, db: &'a dyn TypesAnalyzerCtx) -> OperandTypeRender<'a> {
        OperandTypeRender { ty: self, db }
    }
}
pub(crate) struct OperandTypeRender<'a> {
    ty: &'a OperandType,
    db: &'a dyn TypesAnalyzerCtx,
}
impl Display for OperandTypeRender<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.ty {
            OperandType::Val(ty) => write!(f, "{}", ty.render_compact(self.db)),
            OperandType::Any => write!(f, "any"),
        }
    }
}
