use lspt::{CompletionParams, Position, TextDocumentIdentifier};

mod array;
mod block;
mod data;
mod elem;
mod field;
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
mod ref_instr;
mod result;
mod structs;
mod table;
mod tag;
mod ty_decl;

fn create_params(uri: String, line: u32, character: u32) -> CompletionParams {
    CompletionParams {
        text_document: TextDocumentIdentifier { uri },
        position: Position { line, character },
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
        context: Default::default(),
    }
}
