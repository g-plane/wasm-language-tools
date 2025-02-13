use lsp_types::{
    CompletionParams, Position, TextDocumentIdentifier, TextDocumentPositionParams, Uri,
};

mod block;
mod data;
mod elem;
mod func;
mod func_type;
mod global;
mod import;
mod instr;
mod keyword;
mod local;
mod mem_arg;
mod memory;
mod param;
mod result;
mod table;
mod ty_decl;

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
