use lspt::{CompletionParams, Position, TextDocumentIdentifier};

mod array;
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
mod structs;
mod table;
mod ty_decl;

fn create_params(uri: String, position: Position) -> CompletionParams {
    CompletionParams {
        text_document: TextDocumentIdentifier { uri },
        position,
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
        context: Default::default(),
    }
}
