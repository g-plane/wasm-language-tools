use super::{
    signature::Signature,
    types::{FieldType, Fields, HeapType, OperandType, RefType, StorageType, ValType},
};
use crate::{helpers::RenderWithDb, idx::InternIdent};
use std::fmt::{self, Display, Write};
use wat_syntax::SyntaxKind;

impl<'db> Signature<'db> {
    pub(crate) fn render(&self, db: &'db dyn salsa::Database) -> RenderWithDb<'db, (&Self, bool)> {
        RenderWithDb {
            value: (self, false),
            db,
        }
    }
    pub(crate) fn render_compact(
        &self,
        db: &'db dyn salsa::Database,
    ) -> RenderWithDb<'db, (&Self, bool)> {
        RenderWithDb {
            value: (self, true),
            db,
        }
    }
}
impl Display for RenderWithDb<'_, (&Signature<'_>, bool)> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value.1 {
            write!(f, "[")?;
            let mut params = self.value.0.params.iter();
            if let Some((ty, _)) = params.next() {
                write!(f, "{}", ty.render(self.db))?;
                params.try_for_each(|(ty, _)| write!(f, ", {}", ty.render(self.db)))?;
            }
            write!(f, "] -> [")?;
            let mut results = self.value.0.results.iter();
            if let Some(ty) = results.next() {
                write!(f, "{}", ty.render(self.db))?;
                results.try_for_each(|ty| write!(f, ", {}", ty.render(self.db)))?;
            }
            write!(f, "]")
        } else {
            let mut has_params = false;
            let mut params = self.value.0.params.iter();
            if let Some((ty, name)) = params.next() {
                has_params = true;
                if let Some(name) = name {
                    write!(f, "(param {} {})", name.ident(self.db), ty.render(self.db))?;
                } else {
                    write!(f, "(param {})", ty.render(self.db))?;
                }
                params.try_for_each(|(ty, name)| {
                    if let Some(name) = name {
                        write!(f, " (param {} {})", name.ident(self.db), ty.render(self.db))
                    } else {
                        write!(f, " (param {})", ty.render(self.db))
                    }
                })?;
            }
            let mut results = self.value.0.results.iter();
            if let Some(ty) = results.next() {
                if has_params {
                    write!(f, " ")?;
                }
                write!(f, "(result {})", ty.render(self.db))?;
                results.try_for_each(|ty| write!(f, " (result {})", ty.render(self.db)))
            } else {
                Ok(())
            }
        }
    }
}

pub(crate) fn render_func_header<'db>(
    db: &'db dyn salsa::Database,
    name: Option<InternIdent<'db>>,
    signature: Signature<'db>,
) -> String {
    render_header(db, "func", name, signature)
}

pub(crate) fn render_block_header<'db>(
    db: &'db dyn salsa::Database,
    kind: SyntaxKind,
    name: Option<InternIdent<'db>>,
    signature: Signature<'db>,
) -> String {
    render_header(
        db,
        match kind {
            SyntaxKind::BLOCK_IF => "if",
            SyntaxKind::BLOCK_LOOP => "loop",
            SyntaxKind::BLOCK_TRY_TABLE => "try_table",
            SyntaxKind::MODULE_FIELD_FUNC => "func",
            _ => "block",
        },
        name,
        signature,
    )
}

pub(crate) fn render_header<'db>(
    db: &'db dyn salsa::Database,
    keyword: &str,
    name: Option<InternIdent<'db>>,
    signature: Signature<'db>,
) -> String {
    let mut content = format!("({keyword}");
    if let Some(name) = name {
        content.push(' ');
        content.push_str(name.ident(db));
    }
    if !signature.params.is_empty() || !signature.results.is_empty() {
        content.push(' ');
        let _ = write!(content, "{}", signature.render(db));
    }
    content.push(')');
    content
}

impl<'db> RefType<'db> {
    pub(crate) fn render(&self, db: &'db dyn salsa::Database) -> RenderWithDb<'db, &Self> {
        RenderWithDb { value: self, db }
    }
}
impl Display for RenderWithDb<'_, &RefType<'_>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if matches!(self.value.heap_ty, HeapType::DefFunc(..)) {
            write!(f, "(func ")?;
        } else {
            write!(f, "(ref ")?;
        }
        if self.value.nullable {
            write!(f, "null ")?;
        }
        match self.value.heap_ty {
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
            HeapType::Exn => write!(f, "exn")?,
            HeapType::NoExn => write!(f, "noexn")?,
            HeapType::Extern => write!(f, "extern")?,
            HeapType::NoExtern => write!(f, "noextern")?,
            HeapType::Rec(..) => unreachable!("rec type is only for internal use"),
        }
        write!(f, ")")
    }
}

impl<'db> ValType<'db> {
    pub(crate) fn render(&self, db: &'db dyn salsa::Database) -> RenderWithDb<'db, &Self> {
        RenderWithDb { value: self, db }
    }
}
impl Display for RenderWithDb<'_, &ValType<'_>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
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
    pub(crate) fn render(&self, db: &'db dyn salsa::Database) -> RenderWithDb<'db, &Self> {
        RenderWithDb { value: self, db }
    }
}
impl Display for RenderWithDb<'_, &OperandType<'_>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value {
            OperandType::Val(ty) => write!(f, "{}", ty.render(self.db)),
            OperandType::Any => write!(f, "any"),
        }
    }
}

impl<'db> FieldType<'db> {
    pub(crate) fn render(&self, db: &'db dyn salsa::Database) -> RenderWithDb<'db, &Self> {
        RenderWithDb { value: self, db }
    }
}
impl Display for RenderWithDb<'_, &FieldType<'_>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value.mutable {
            write!(f, "(mut ")?;
        }
        match &self.value.storage {
            StorageType::Val(ty) => write!(f, "{}", ty.render(self.db))?,
            StorageType::PackedI8 => write!(f, "i8")?,
            StorageType::PackedI16 => write!(f, "i16")?,
        }
        if self.value.mutable {
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl<'db> Fields<'db> {
    pub(crate) fn render(&self, db: &'db dyn salsa::Database) -> RenderWithDb<'db, &Self> {
        RenderWithDb { value: self, db }
    }
}
impl Display for RenderWithDb<'_, &Fields<'_>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.0.iter().try_fold(true, |first, field| {
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
