mod call_hierarchy;
mod code_action;
mod completion;
mod definition;
mod diagnostics;
mod document_highlight;
mod document_symbol;
mod folding_range;
mod formatting;
mod hover;
mod inlay_hint;
mod references;
mod rename;
mod selection_range;
mod semantic_tokens;
mod signature_help;
mod type_hierarchy;

pub(crate) use self::semantic_tokens::SemanticTokenKind;
use crate::{LanguageService, document::Document, helpers};
use lspt::Position;
use rowan::TokenAtOffset;
use wat_syntax::{SyntaxNode, SyntaxToken};

fn find_meaningful_token(
    service: &LanguageService,
    document: Document,
    root: &SyntaxNode,
    position: Position,
) -> Option<SyntaxToken> {
    let offset = helpers::lsp_pos_to_rowan_pos(document.line_index(service), position)?;

    match root.token_at_offset(offset) {
        TokenAtOffset::None => None,
        TokenAtOffset::Single(token) => Some(token),
        TokenAtOffset::Between(left, right) => {
            let left_check = left.kind().is_trivia() || left.kind().is_punc();
            let right_check = right.kind().is_trivia() || right.kind().is_punc();
            if left_check && right_check || !left_check && !right_check {
                None
            } else if left_check {
                Some(right)
            } else {
                Some(left)
            }
        }
    }
}
