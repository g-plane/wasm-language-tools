mod call_hierarchy;
mod code_action;
mod completion;
mod definition;
mod document_symbol;
mod hover;
mod inlay_hint;
mod references;
mod rename;
mod semantic_tokens;

pub(crate) use self::semantic_tokens::SemanticTokenKind;
use crate::{files::FilesCtx, helpers, InternUri, LanguageServiceCtx};
use lsp_types::Position;
use rowan::TokenAtOffset;
use wat_syntax::{is_punc, is_trivia, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

fn find_meaningful_token(
    service: &LanguageServiceCtx,
    uri: InternUri,
    root: &SyntaxNode,
    position: Position,
) -> Option<SyntaxToken> {
    let offset = helpers::lsp_pos_to_rowan_pos(&service.line_index(uri), position)?;

    match root.token_at_offset(offset) {
        TokenAtOffset::None => None,
        TokenAtOffset::Single(token) => Some(token),
        TokenAtOffset::Between(left, right) => {
            let left_check = is_trivia(&left) || is_punc(&left);
            let right_check = is_trivia(&right) || is_punc(&right);
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

fn is_call(node: &SyntaxNode) -> bool {
    node.children_with_tokens().any(|element| {
        if let SyntaxElement::Token(token) = element {
            token.kind() == SyntaxKind::INSTR_NAME
                && matches!(token.text(), "call" | "ref.func" | "return_call")
        } else {
            false
        }
    })
}
