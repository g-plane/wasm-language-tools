#[cfg(test)]
mod func;
#[cfg(test)]
mod func_type;
#[cfg(test)]
mod global;
#[cfg(test)]
mod instr;
#[cfg(test)]
mod keyword;
#[cfg(test)]
mod local;
#[cfg(test)]
mod param;
#[cfg(test)]
mod result;

use lsp_types::{
    CompletionParams, Position, TextDocumentIdentifier, TextDocumentPositionParams, Uri,
};

fn create_params(uri: Uri, position: Position) -> CompletionParams {
    CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position,
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
        context: Default::default(),
    }
}
