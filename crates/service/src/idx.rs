use salsa::{InternId, InternKey};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Idx {
    pub num: Option<u32>,
    pub name: Option<InternIdent>,
}

impl Idx {
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
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct InternIdent(InternId);
impl InternKey for InternIdent {
    fn from_intern_id(v: salsa::InternId) -> Self {
        InternIdent(v)
    }
    fn as_intern_id(&self) -> InternId {
        self.0
    }
}

#[salsa::query_group(Idents)]
pub(crate) trait IdentsCtx {
    #[salsa::interned]
    fn ident(&self, ident: String) -> InternIdent;
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
