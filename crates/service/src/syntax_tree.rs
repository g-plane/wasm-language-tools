use crate::{
    uri::{InternUri, UrisCtx},
    LanguageService,
};
use line_index::LineIndex;
use rowan::GreenNode;
use std::sync::Arc;
use wat_parser::{parse_to_green, SyntaxError};

#[salsa::query_group(SyntaxTree)]
pub(crate) trait SyntaxTreeCtx: salsa::Database {
    #[salsa::input]
    fn line_index(&self, uri: InternUri) -> Arc<LineIndex>;

    #[salsa::input]
    fn root(&self, uri: InternUri) -> GreenNode;

    #[salsa::input]
    fn syntax_errors(&self, uri: InternUri) -> Arc<Vec<SyntaxError>>;
}

impl LanguageService {
    #[inline]
    /// Commit a document to the service, usually called when handling `textDocument/didOpen` or
    /// `textDocument/didChange` notifications.
    pub fn commit(&mut self, uri: String, source: String) {
        let uri = self.uri(uri);
        self.set_line_index(uri, Arc::new(LineIndex::new(&source)));
        let (green, errors) = parse_to_green(&source);
        self.set_root(uri, green);
        self.set_syntax_errors(uri, Arc::new(errors));
    }
}
