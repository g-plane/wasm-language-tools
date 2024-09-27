use super::{find_meaningful_token, locate_module};
use crate::{binder::SymbolTablesCtx, files::FileInputCtx, helpers, LanguageServiceCtx};
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location};
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
        let line_index = service.line_index(uri.clone());
        let symbol_table = service.symbol_table(uri.clone());
        let module = locate_module(&symbol_table, token.parent_ancestors())?;

        match token.kind() {
            SyntaxKind::IDENT => {
                let name = token.text();
                Some(GotoDefinitionResponse::Array(
                    module
                        .functions
                        .iter()
                        .filter(|func| func.idx.name.as_deref().is_some_and(|n| n == name))
                        .map(|func| Location {
                            uri: uri.clone(),
                            range: helpers::rowan_range_to_lsp_range(
                                &line_index,
                                func.ptr.syntax_node_ptr().text_range(),
                            ),
                        })
                        .collect(),
                ))
            }
            SyntaxKind::INT => {
                let num: usize = token.text().parse().ok()?;
                Some(GotoDefinitionResponse::Array(
                    module
                        .functions
                        .iter()
                        .filter(|func| func.idx.num == num)
                        .map(|func| Location {
                            uri: uri.clone(),
                            range: helpers::rowan_range_to_lsp_range(
                                &line_index,
                                func.ptr.syntax_node_ptr().text_range(),
                            ),
                        })
                        .collect(),
                ))
            }
            _ => None,
        }
    } else {
        None
    }
}
