mod definition;
mod document_symbol;

pub use self::{definition::goto_definition, document_symbol::document_symbol};
use crate::{
    binder::{Module, SymbolTable},
    files::FileInputCtx,
    LanguageServiceCtx,
};
use line_index::LineCol;
use lsp_types::{Position, Uri};
use rowan::{
    ast::{AstNode, SyntaxNodePtr},
    TokenAtOffset,
};
use wat_syntax::{is_punc, is_trivia, SyntaxKind, SyntaxNode, SyntaxToken};

fn find_meaningful_token(
    service: &LanguageServiceCtx,
    uri: Uri,
    position: Position,
) -> Option<SyntaxToken> {
    let offset = service
        .line_index(uri.clone())
        .offset(LineCol {
            line: position.line,
            col: position.character,
        })
        .map(|text_size| rowan::TextSize::new(text_size.into()))?;

    match service.root(uri.clone()).syntax().token_at_offset(offset) {
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
) -> Option<&Module> {
    let module_node = ancestors.find(|node| node.kind() == SyntaxKind::MODULE)?;
    let green = module_node.green().into();
    let ptr = SyntaxNodePtr::new(&module_node);
    symbol_table
        .modules
        .iter()
        .find(|module| module.green == green && module.ptr.syntax_node_ptr() == ptr)
}
