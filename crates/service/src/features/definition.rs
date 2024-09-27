use super::find_meaningful_token;
use crate::{binder::SymbolTablesCtx, files::FileInputCtx, LanguageServiceCtx};
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location, Position, Range};
use wat_syntax::{SyntaxElement, SyntaxKind};

pub fn goto_definition(
    service: &LanguageServiceCtx,
    params: GotoDefinitionParams,
) -> Option<GotoDefinitionResponse> {
    let uri = params.text_document_position_params.text_document.uri;
    let token = find_meaningful_token(
        service,
        uri.clone(),
        params.text_document_position_params.position,
    )?;

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
        if token.kind() != SyntaxKind::IDENT {
            return None;
        }
        let name = token.text();
        let line_index = service.line_index(uri.clone());
        Some(GotoDefinitionResponse::Array(
            service
                .symbol_table(uri.clone())
                .functions
                .iter()
                .filter(|func| func.idx.name.as_deref().is_some_and(|n| n == name))
                .map(|func| {
                    let range = func.ptr.syntax_node_ptr().text_range();
                    let start = line_index.line_col(range.start());
                    let end = line_index.line_col(range.end());
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
}
