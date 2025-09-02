use super::{
    signature::Signature,
    types::{FieldType, Fields, HeapType, OperandType, RefType, StorageType, ValType},
};
use crate::idx::InternIdent;
use itertools::Itertools;
use std::fmt::{self, Display};
use wat_syntax::SyntaxKind;

pub(crate) fn render_sig<'db>(db: &'db dyn salsa::Database, signature: Signature<'db>) -> String {
    let mut ret = String::with_capacity(signature.params.len() * 9 + signature.results.len() * 10);
    let params = signature
        .params
        .iter()
        .map(|(ty, name)| {
            if let Some(name) = name {
                format!("(param {} {})", name.ident(db), ty.render(db))
            } else {
                format!("(param {})", ty.render(db))
            }
        })
        .join(" ");
    ret.push_str(&params);
    let results = signature
        .results
        .iter()
        .map(|ty| format!("(result {})", ty.render(db)))
        .join(" ");
    if !params.is_empty() && !results.is_empty() {
        ret.push(' ');
    }
    ret.push_str(&results);
    ret
}

pub(crate) fn render_compact_sig<'db>(
    db: &'db dyn salsa::Database,
    signature: Signature<'db>,
) -> String {
    let params = signature
        .params
        .iter()
        .map(|(ty, _)| ty.render(db))
        .join(", ");
    let results = signature.results.iter().map(|ty| ty.render(db)).join(", ");
    format!("[{params}] -> [{results}]")
}

#[salsa::tracked]
pub(crate) fn render_func_header<'db>(
    db: &'db dyn salsa::Database,
    name: Option<InternIdent<'db>>,
    signature: Signature<'db>,
) -> String {
    let mut content = "(func".to_string();
    if let Some(name) = name {
        content.push(' ');
        content.push_str(name.ident(db));
    }
    if !signature.params.is_empty() || !signature.results.is_empty() {
        content.push(' ');
        content.push_str(&render_sig(db, signature));
    }
    content.push(')');
    content
}

#[salsa::tracked]
pub(crate) fn render_block_header<'db>(
    db: &'db dyn salsa::Database,
    kind: SyntaxKind,
    name: Option<InternIdent<'db>>,
    signature: Signature<'db>,
) -> String {
    let mut content = format!(
        "({}",
        match kind {
            SyntaxKind::BLOCK_IF => "if",
            SyntaxKind::BLOCK_LOOP => "loop",
            SyntaxKind::MODULE_FIELD_FUNC => "func",
            _ => "block",
        }
    );
    if let Some(name) = name {
        content.push(' ');
        content.push_str(name.ident(db));
    }
    if !signature.params.is_empty() || !signature.results.is_empty() {
        content.push(' ');
        content.push_str(&render_sig(db, signature));
    }
    content.push(')');
    content
}

impl<'db> RefType<'db> {
    pub(crate) fn render<'a>(&'a self, db: &'db dyn salsa::Database) -> RefTypeRender<'a, 'db> {
        RefTypeRender { ty: self, db }
    }
}
pub(crate) struct RefTypeRender<'a, 'db> {
    ty: &'a RefType<'db>,
    db: &'db dyn salsa::Database,
}
impl Display for RefTypeRender<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if matches!(self.ty.heap_ty, HeapType::DefFunc(..)) {
            write!(f, "(func ")?;
        } else {
            write!(f, "(ref ")?;
        }
        if self.ty.nullable {
            write!(f, "null ")?;
        }
        match self.ty.heap_ty {
            HeapType::Type(idx) | HeapType::DefFunc(idx) => {
                if let Some(name) = idx.name {
                    write!(f, "{}", name.ident(self.db))?;
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
            HeapType::Rec(..) => unreachable!("rec type is only for internal use"),
        }
        write!(f, ")")
    }
}

impl<'db> ValType<'db> {
    pub(crate) fn render<'a>(&'a self, db: &'db dyn salsa::Database) -> ValTypeRender<'a, 'db> {
        ValTypeRender { ty: self, db }
    }
}
pub(crate) struct ValTypeRender<'a, 'db> {
    ty: &'a ValType<'db>,
    db: &'db dyn salsa::Database,
}
impl Display for ValTypeRender<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.ty {
            ValType::I32 => write!(f, "i32"),
            ValType::I64 => write!(f, "i64"),
            ValType::F32 => write!(f, "f32"),
            ValType::F64 => write!(f, "f64"),
            ValType::V128 => write!(f, "v128"),
            ValType::Ref(ty) => ty.render(self.db).fmt(f),
        }
    }
}

impl<'db> OperandType<'db> {
    pub(crate) fn render<'a>(&'a self, db: &'db dyn salsa::Database) -> OperandTypeRender<'a, 'db> {
        OperandTypeRender { ty: self, db }
    }
}
pub(crate) struct OperandTypeRender<'a, 'db> {
    ty: &'a OperandType<'db>,
    db: &'db dyn salsa::Database,
}
impl Display for OperandTypeRender<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.ty {
            OperandType::Val(ty) => write!(f, "{}", ty.render(self.db)),
            OperandType::Any => write!(f, "any"),
        }
    }
}

impl<'db> FieldType<'db> {
    pub(crate) fn render<'a>(&'a self, db: &'db dyn salsa::Database) -> FieldTypeRender<'a, 'db> {
        FieldTypeRender { ty: self, db }
    }
}
pub(crate) struct FieldTypeRender<'a, 'db> {
    ty: &'a FieldType<'db>,
    db: &'db dyn salsa::Database,
}
impl Display for FieldTypeRender<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.ty.mutable {
            write!(f, "(mut ")?;
        }
        match &self.ty.storage {
            StorageType::Val(ty) => write!(f, "{}", ty.render(self.db))?,
            StorageType::PackedI8 => write!(f, "i8")?,
            StorageType::PackedI16 => write!(f, "i16")?,
        }
        if self.ty.mutable {
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl<'db> Fields<'db> {
    pub(crate) fn render<'a>(&'a self, db: &'db dyn salsa::Database) -> FieldsRender<'a, 'db> {
        FieldsRender { fields: self, db }
    }
}
pub(crate) struct FieldsRender<'a, 'db> {
    fields: &'a Fields<'db>,
    db: &'db dyn salsa::Database,
}
impl Display for FieldsRender<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fields.0.iter().try_fold(true, |first, field| {
            if !first {
                write!(f, " ")?;
            }
            write!(f, "(field ")?;
            if let Some(name) = field.1.name {
                write!(f, "{} ", name.ident(self.db))?;
            }
            write!(f, "{}", field.0.render(self.db))?;
            write!(f, ")")?;
            Ok(false)
        })?;
        Ok(())
    }
}
