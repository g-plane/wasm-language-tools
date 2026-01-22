use crate::{LanguageService, helpers};
use lspt::{FoldingRange, FoldingRangeKind, FoldingRangeParams};
use rowan::ast::support;
use wat_syntax::SyntaxKind;

impl LanguageService {
    /// Handler for `textDocument/foldingRange` request.
    pub fn folding_range(&self, params: FoldingRangeParams) -> Option<Vec<FoldingRange>> {
        let document = self.get_document(params.text_document.uri)?;
        self.with_db(|db| {
            let line_index = document.line_index(db);
            let root = document.root_tree(db);
            root.descendants()
                .filter_map(|node| {
                    support::token(&node, SyntaxKind::KEYWORD)
                        .or_else(|| support::token(&node, SyntaxKind::L_PAREN))?;
                    let range = helpers::rowan_range_to_lsp_range(line_index, node.text_range());
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
                .collect()
        })
    }
}
