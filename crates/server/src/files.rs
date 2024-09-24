use ahash::AHashMap;
use lsp_types::Uri;

#[derive(Clone, Debug, Default)]
pub struct Files(AHashMap<Uri, String>);

impl Files {
    pub fn write(&mut self, uri: Uri, source: String) {
        self.0.insert(uri, source);
    }

    pub fn remove(&mut self, uri: &Uri) {
        self.0.remove(uri);
    }
}
