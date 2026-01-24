use crate::{LanguageService, helpers::LineIndexExt};
use line_index::LineIndex;
use lspt::{SelectionRange, SelectionRangeParams};
use wat_syntax::SyntaxNode;

impl LanguageService {
    /// Handler for `textDocument/selectionRange` request.
    pub fn selection_range(&self, params: SelectionRangeParams) -> Option<Vec<SelectionRange>> {
        let document = self.get_document(params.text_document.uri)?;
        self.with_db(|db| {
            let line_index = document.line_index(db);
            let root = document.root_tree(db);
            params
                .positions
                .into_iter()
                .filter_map(|position| {
                    super::find_meaningful_token(db, document, &root, position).map(|token| SelectionRange {
                        range: line_index.convert(token.text_range()),
                        parent: token.parent().map(|parent| {
                            Box::new(SelectionRange {
                                range: line_index.convert(parent.text_range()),
                                parent: get_parent_range(parent, line_index),
                            })
                        }),
                    })
                })
                .collect()
        })
    }
}

fn get_parent_range(current: SyntaxNode, line_index: &LineIndex) -> Option<Box<SelectionRange>> {
    current.parent().map(|parent| {
        Box::new(SelectionRange {
            range: line_index.convert(parent.text_range()),
            parent: get_parent_range(parent, line_index),
        })
    })
}
