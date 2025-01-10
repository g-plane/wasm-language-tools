use lsp_types::Uri;
use salsa::{InternId, InternKey};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct InternUri(InternId);

impl InternKey for InternUri {
    fn from_intern_id(v: salsa::InternId) -> Self {
        InternUri(v)
    }
    fn as_intern_id(&self) -> InternId {
        self.0
    }
}

#[salsa::query_group(Uris)]
pub(crate) trait UrisCtx: salsa::Database {
    #[salsa::interned]
    fn uri(&self, uri: Uri) -> InternUri;
}
