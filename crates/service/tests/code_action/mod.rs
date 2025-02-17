use lspt::{CodeActionContext, CodeActionParams, Range, TextDocumentIdentifier};

mod br_if_to_if_br;
mod fix_invalid_mem_arg;
mod func_header_join;
mod func_header_split;
mod idx_conversion;
mod if_br_to_br_if;
mod inline_func_type;
mod remove_mut;

fn create_params(uri: String, range: Range) -> CodeActionParams {
    CodeActionParams {
        text_document: TextDocumentIdentifier { uri },
        range,
        context: CodeActionContext {
            diagnostics: vec![],
            only: None,
            trigger_kind: None,
        },
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    }
}
