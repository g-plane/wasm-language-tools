use crate::{LanguageService, config::ServiceConfig, uri::InternUri};
use line_index::LineIndex;
use rowan::GreenNode;
use salsa::Setter;
use wat_syntax::SyntaxNode;

#[salsa::input(debug)]
pub(crate) struct Document {
    pub uri: InternUri,
    #[returns(ref)]
    pub line_index: LineIndex,
    pub root: GreenNode,
    #[returns(ref)]
    pub syntax_errors: Vec<wat_parser::SyntaxError>,
    #[returns(as_ref)]
    pub config: Option<ServiceConfig>,
}

impl Document {
    pub(crate) fn root_tree(self, db: &dyn salsa::Database) -> SyntaxNode {
        SyntaxNode::new_root(self.root(db))
    }
}

impl LanguageService {
    #[inline]
    /// Commit a document to the service, usually called when handling `textDocument/didOpen` or
    /// `textDocument/didChange` notifications.
    pub fn commit(&mut self, uri: String, source: String) {
        let uri = InternUri::new(self, uri);
        let line_index = LineIndex::new(&source);
        let (green, errors) = wat_parser::parse_to_green(&source);
        if let Some(document) = self.documents.get(&uri).copied() {
            document.set_line_index(self).to(line_index);
            document.set_root(self).to(green);
            document.set_syntax_errors(self).to(errors);
        } else {
            self.documents.insert(
                uri,
                Document::new(self, uri, line_index, green, errors, None),
            );
        };
    }

    pub(crate) fn get_document(&self, uri: impl AsRef<str>) -> Option<Document> {
        self.documents
            .get(&InternUri::new(self, uri.as_ref()))
            .copied()
    }
}
