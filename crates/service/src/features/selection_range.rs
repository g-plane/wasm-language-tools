use crate::{LanguageService, helpers};
use line_index::LineIndex;
use lspt::{SelectionRange, SelectionRangeParams};
use wat_syntax::SyntaxNode;

impl LanguageService {
    /// Handler for `textDocument/selectionRange` request.
    pub fn selection_range(&self, params: SelectionRangeParams) -> Option<Vec<SelectionRange>> {
        let document = self.get_document(params.text_document.uri)?;
        let line_index = document.line_index(self);
        let root = document.root_tree(self);
        Some(
            params
                .positions
                .into_iter()
                .filter_map(|position| {
                    super::find_meaningful_token(self, *document, &root, position).map(|token| {
                        SelectionRange {
                            range: helpers::rowan_range_to_lsp_range(
                                line_index,
                                token.text_range(),
                            ),
                            parent: token.parent().map(|parent| {
                                Box::new(SelectionRange {
                                    range: helpers::rowan_range_to_lsp_range(
                                        line_index,
                                        parent.text_range(),
                                    ),
                                    parent: get_parent_range(parent, line_index),
                                })
                            }),
                        }
                    })
                })
                .collect(),
        )
    }
}

fn get_parent_range(current: SyntaxNode, line_index: &LineIndex) -> Option<Box<SelectionRange>> {
    current.parent().map(|parent| {
        Box::new(SelectionRange {
            range: helpers::rowan_range_to_lsp_range(line_index, parent.text_range()),
            parent: get_parent_range(parent, line_index),
        })
    })
}
