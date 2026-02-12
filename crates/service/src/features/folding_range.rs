use crate::{LanguageService, helpers::LineIndexExt};
use lspt::{FoldingRange, FoldingRangeKind, FoldingRangeParams};
use wat_syntax::{SyntaxKind, ast::support};

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
                    let range = line_index.convert(node.text_range());
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
