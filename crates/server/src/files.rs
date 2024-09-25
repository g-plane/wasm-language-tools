use ahash::AHashMap;
use comemo::Tracked;
use line_index::LineIndex;
use lsp_types::Uri;

#[derive(Clone, Debug, Default)]
pub struct Files(AHashMap<Uri, String>);

#[comemo::track]
impl Files {
    pub fn read(&self, uri: &Uri) -> String {
        self.0.get(uri).cloned().unwrap_or_default()
    }
}

impl Files {
    pub fn write(&mut self, uri: Uri, source: String) {
        self.0.insert(uri, source);
    }

    pub fn remove(&mut self, uri: &Uri) {
        self.0.remove(uri);
    }
}

#[comemo::memoize]
pub(crate) fn get_line_index(uri: &Uri, files: Tracked<Files>) -> LineIndex {
    LineIndex::new(&files.read(uri))
}
