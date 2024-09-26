use crate::{binder::SymbolTable, files::File};
use leptos_reactive::{RwSignal, SignalWith};
use line_index::LineCol;
use lsp_types::{GotoDefinitionResponse, Location, Position, Range, Uri};
use rowan::{ast::AstNode, TokenAtOffset};
use wat_syntax::{is_punc, is_trivia, SyntaxElement, SyntaxKind};

pub fn goto_definition(
    position: Position,
    file: Option<&RwSignal<File>>,
    symbol_table: Option<SymbolTable>,
    uri: Uri,
) -> Option<GotoDefinitionResponse> {
    file?.with(|file| {
        let offset = file
            .line_index
            .offset(LineCol {
                line: position.line,
                col: position.character,
            })
            .map(|text_size| rowan::TextSize::new(text_size.into()))?;

        let token = (match file.tree.syntax().token_at_offset(offset) {
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
        })?;
        if token.kind() != SyntaxKind::IDENT {
            return None;
        }
        if token
            .parent()
            .and_then(|parent| parent.parent())
            .filter(|node| node.kind() == SyntaxKind::PLAIN_INSTR)
            .is_some_and(|instr| {
                instr.children_with_tokens().any(|element| {
                    if let SyntaxElement::Token(token) = element {
                        token.kind() == SyntaxKind::INSTR_NAME && token.text() == "call"
                    } else {
                        false
                    }
                })
            })
        {
            let name = token.text();
            Some(GotoDefinitionResponse::Array(
                symbol_table?
                    .functions
                    .iter()
                    .filter(|func| func.idx.name.as_deref().is_some_and(|n| n == name))
                    .map(|func| {
                        let range = func.ptr.syntax_node_ptr().text_range();
                        let start = file.line_index.line_col(range.start());
                        let end = file.line_index.line_col(range.end());
                        Location {
                            uri: uri.clone(),
                            range: Range::new(
                                Position::new(start.line, start.col),
                                Position::new(end.line, end.col),
                            ),
                        }
                    })
                    .collect(),
            ))
        } else {
            None
        }
    })
}
