use super::{find_meaningful_token, locate_module};
use crate::{binder::SymbolTablesCtx, files::FilesCtx, helpers, LanguageService};
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location};
use wat_syntax::{SyntaxElement, SyntaxKind};

impl LanguageService {
    pub fn goto_definition(&self, params: GotoDefinitionParams) -> Option<GotoDefinitionResponse> {
        let uri = self.ctx.uri(
            params
                .text_document_position_params
                .text_document
                .uri
                .clone(),
        );
        let token = find_meaningful_token(
            &self.ctx,
            uri,
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
            let line_index = self.ctx.line_index(uri);
            let symbol_table = self.ctx.symbol_table(uri);
            let module = locate_module(&symbol_table, token.parent_ancestors())?;

            match token.kind() {
                SyntaxKind::IDENT => {
                    let name = token.text();
                    let uri = params.text_document_position_params.text_document.uri;
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
                    let uri = params.text_document_position_params.text_document.uri;
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
}
