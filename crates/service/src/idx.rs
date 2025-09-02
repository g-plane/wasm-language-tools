use crate::helpers::RenderWithDb;
use std::fmt;
use wat_syntax::ast::Immediate;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub struct Idx<'db> {
    pub num: Option<u32>,
    pub name: Option<InternIdent<'db>>,
}

impl<'db> Idx<'db> {
    pub fn from_immediate(immediate: &Immediate, db: &'db dyn salsa::Database) -> Self {
        Idx {
            num: immediate.int().and_then(|int| int.text().parse().ok()),
            name: immediate
                .ident()
                .map(|ident| InternIdent::new(db, ident.text())),
        }
    }

    pub fn is_def(&self) -> bool {
        matches!(self, Idx { num: Some(..), .. })
    }

    pub fn is_ref(&self) -> bool {
        if self.name.is_some() {
            self.num.is_none()
        } else {
            self.num.is_some()
        }
    }

    pub fn is_defined_by(&self, other: &Self) -> bool {
        debug_assert!(self.is_ref());
        debug_assert!(other.is_def());
        match (self, other) {
            (
                Idx { num: Some(num), .. },
                Idx {
                    num: Some(other_num),
                    ..
                },
            ) => num == other_num,
            (
                Idx {
                    name: Some(name), ..
                },
                Idx {
                    name: Some(other_name),
                    ..
                },
            ) => name == other_name,
            _ => false,
        }
    }

    pub fn render(&self, db: &'db dyn salsa::Database) -> RenderWithDb<'db, &Self> {
        RenderWithDb { value: self, db }
    }
}

impl fmt::Display for RenderWithDb<'_, &Idx<'_>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.value.name {
            name.ident(self.db).fmt(f)
        } else if let Some(num) = self.value.num {
            num.fmt(f)
        } else {
            Ok(())
        }
    }
}

#[salsa::interned(debug)]
pub(crate) struct InternIdent<'db> {
    #[returns(ref)]
    pub(crate) ident: String,
}

#[derive(Default)]
pub(crate) struct IdxGen(u32);

impl IdxGen {
    /// Get numeric idx then increment for next.
    pub fn pull(&mut self) -> u32 {
        let idx = self.0;
        self.0 += 1;
        idx
    }
}
