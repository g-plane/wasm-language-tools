use crate::{helpers, syntax_tree::SyntaxTreeCtx, uri::UrisCtx, LanguageService};
use lsp_types::{FoldingRange, FoldingRangeKind, FoldingRangeParams};
use rowan::ast::support::token;
use wat_syntax::{SyntaxKind, SyntaxNode};

impl LanguageService {
    /// Handler for `textDocument/foldingRange` request.
    pub fn folding_range(&self, params: FoldingRangeParams) -> Option<Vec<FoldingRange>> {
        let uri = self.uri(params.text_document.uri);
        let line_index = self.line_index(uri);
        let root = SyntaxNode::new_root(self.root(uri));
        let folding_ranges = root
            .descendants()
            .filter_map(|node| {
                token(&node, SyntaxKind::KEYWORD).or_else(|| token(&node, SyntaxKind::L_PAREN))?;
                let range = helpers::rowan_range_to_lsp_range(&line_index, node.text_range());
                if range.start.line == range.end.line {
                    None
                } else {
                    Some(FoldingRange {
                        start_line: range.start.line,
                        start_character: Some(range.start.character),
                        end_line: range.end.line,
                        end_character: Some(range.end.character),
                        kind: Some(FoldingRangeKind::Region),
                        collapsed_text: None,
                    })
                }
            })
            .collect();
        Some(folding_ranges)
    }
}
