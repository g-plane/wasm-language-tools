#[salsa::interned(no_lifetime, debug)]
pub(crate) struct InternUri {
    pub raw: String,
}

pub(crate) trait IntoInternUri {
    fn into_intern_uri(self, db: &dyn salsa::Database) -> InternUri;
}
impl IntoInternUri for InternUri {
    fn into_intern_uri(self, _: &dyn salsa::Database) -> InternUri {
        self
    }
}
impl<S> IntoInternUri for S
where
    S: AsRef<str>,
{
    fn into_intern_uri(self, db: &dyn salsa::Database) -> InternUri {
        InternUri::new(db, self.as_ref())
    }
}
