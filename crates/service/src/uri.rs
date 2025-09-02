#[salsa::interned(no_lifetime, debug)]
pub(crate) struct InternUri {
    pub raw: String,
}
