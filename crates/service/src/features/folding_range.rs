use crate::LanguageService;
use lspt::{FoldingRange, FoldingRangeKind, FoldingRangeParams};
use wat_syntax::{AmberNode, NodeOrToken, SyntaxKind};

impl LanguageService {
    /// Handler for `textDocument/foldingRange` request.
    pub fn folding_range(&self, params: FoldingRangeParams) -> Option<Vec<FoldingRange>> {
        let document = self.get_document(params.text_document.uri)?;
        self.with_db(|db| {
            let line_index = document.line_index(db);
            let mut ranges = Vec::with_capacity(8);
            let mut node_stack = vec![(AmberNode::new_root(document.root(db)), 0)];
            while let Some((parent, index)) = node_stack.last_mut() {
                match parent.child_or_token_at(*index) {
                    Some(NodeOrToken::Node(node)) => {
                        node_stack.push((node, 0));
                    }
                    Some(NodeOrToken::Token(token)) => {
                        if *index == 0
                            && matches!(token.kind(), SyntaxKind::L_PAREN | SyntaxKind::KEYWORD)
                            && let Some(range) = line_index.convert(parent.text_range())
                            && range.start.line != range.end.line
                        {
                            ranges.push(FoldingRange {
                                start_line: range.start.line,
                                start_character: Some(range.start.character),
                                end_line: range.end.line,
                                end_character: Some(range.end.character),
                                kind: Some(FoldingRangeKind::Region),
                                collapsed_text: None,
                            });
                        }
                        *index += 1;
                    }
                    None => {
                        node_stack.pop();
                        if let Some((_, index)) = node_stack.last_mut() {
                            *index += 1;
                        }
                    }
                }
            }
            ranges
        })
    }
}
