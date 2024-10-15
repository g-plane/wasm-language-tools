use lsp_types::{
    GotoDefinitionParams, Position, TextDocumentIdentifier, TextDocumentPositionParams, Uri,
};

#[cfg(test)]
mod goto_declaration;
#[cfg(test)]
mod goto_definition;
#[cfg(test)]
mod goto_type_definition;

fn create_params(uri: Uri, position: Position) -> GotoDefinitionParams {
    GotoDefinitionParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position,
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
    }
}
