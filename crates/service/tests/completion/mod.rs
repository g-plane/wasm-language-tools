use lsp_types::{
    CompletionParams, Position, TextDocumentIdentifier, TextDocumentPositionParams, Uri,
};

#[cfg(test)]
mod block;
#[cfg(test)]
mod data;
#[cfg(test)]
mod elem;
#[cfg(test)]
mod func;
#[cfg(test)]
mod func_type;
#[cfg(test)]
mod global;
#[cfg(test)]
mod import;
#[cfg(test)]
mod instr;
#[cfg(test)]
mod keyword;
#[cfg(test)]
mod local;
#[cfg(test)]
mod mem_arg;
#[cfg(test)]
mod memory;
#[cfg(test)]
mod param;
#[cfg(test)]
mod result;
#[cfg(test)]
mod table;
#[cfg(test)]
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
