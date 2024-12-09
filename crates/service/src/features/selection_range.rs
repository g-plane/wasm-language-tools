use super::{find_meaningful_token, FilesCtx};
use crate::{helpers, LanguageService};
use line_index::LineIndex;
use lsp_types::{SelectionRange, SelectionRangeParams};
use wat_syntax::SyntaxNode;

impl LanguageService {
    /// Handler for `textDocument/selectionRange` request.
    pub fn selection_range(&self, params: SelectionRangeParams) -> Option<Vec<SelectionRange>> {
        let uri = self.uri(params.text_document.uri);
        let line_index = self.line_index(uri);
        let root = SyntaxNode::new_root(self.root(uri));
        Some(
            params
                .positions
                .into_iter()
                .filter_map(|position| {
                    find_meaningful_token(self, uri, &root, position).map(|token| SelectionRange {
                        range: helpers::rowan_range_to_lsp_range(&line_index, token.text_range()),
                        parent: token.parent().map(|parent| {
                            Box::new(SelectionRange {
                                range: helpers::rowan_range_to_lsp_range(
                                    &line_index,
                                    parent.text_range(),
                                ),
                                parent: get_parent_range(parent, &line_index),
                            })
                        }),
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
