mod definition;
mod document_symbol;

use crate::{
    binder::{SymbolItem, SymbolItemKind, SymbolTable},
    files::FilesCtx,
    InternUri, LanguageServiceCtx,
};
use line_index::LineCol;
use lsp_types::Position;
use rowan::{ast::SyntaxNodePtr, TokenAtOffset};
use wat_syntax::{is_punc, is_trivia, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

fn find_meaningful_token(
    service: &LanguageServiceCtx,
    uri: InternUri,
    position: Position,
) -> Option<SyntaxToken> {
    let offset = service
        .line_index(uri)
        .offset(LineCol {
            line: position.line,
            col: position.character,
        })
        .map(|text_size| rowan::TextSize::new(text_size.into()))?;

    match service.root(uri).token_at_offset(offset) {
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

fn locate_module(
    symbol_table: &SymbolTable,
    mut ancestors: impl Iterator<Item = SyntaxNode>,
) -> Option<&SymbolItem> {
    let module_node = ancestors.find(|node| node.kind() == SyntaxKind::MODULE)?;
    let green = module_node.green().into();
    let ptr = SyntaxNodePtr::new(&module_node);
    symbol_table.symbols.iter().find(|symbol| {
        matches!(symbol.kind, SymbolItemKind::Module)
            && symbol.key.green == green
            && symbol.key.ptr == ptr
    })
}

fn is_call(node: &SyntaxNode) -> bool {
    node.children_with_tokens().any(|element| {
        if let SyntaxElement::Token(token) = element {
            token.kind() == SyntaxKind::INSTR_NAME && token.text() == "call"
        } else {
            false
        }
    })
}
