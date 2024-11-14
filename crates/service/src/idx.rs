use salsa::{InternId, InternKey};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DefIdx {
    pub num: u32,
    pub name: Option<InternIdent>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RefIdx {
    Num(u32),
    Name(InternIdent),
}

impl PartialEq<u32> for RefIdx {
    fn eq(&self, other: &u32) -> bool {
        match self {
            RefIdx::Num(num) => num == other,
            RefIdx::Name(..) => false,
        }
    }
}
impl PartialEq<DefIdx> for RefIdx {
    fn eq(&self, other: &DefIdx) -> bool {
        match self {
            RefIdx::Num(num) => *num == other.num,
            RefIdx::Name(name) => other.name.as_ref().is_some_and(|s| name == s),
        }
    }
}
impl PartialEq<RefIdx> for DefIdx {
    fn eq(&self, other: &RefIdx) -> bool {
        match other {
            RefIdx::Num(num) => self.num == *num,
            RefIdx::Name(name) => self.name.as_ref().is_some_and(|s| name == s),
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
