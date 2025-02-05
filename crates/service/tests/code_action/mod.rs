use lsp_types::{CodeActionContext, CodeActionParams, Range, TextDocumentIdentifier, Uri};

mod br_if_to_if_br;
mod fix_invalid_mem_arg;
mod func_header_join;
mod func_header_split;
mod if_br_to_br_if;
mod inline_func_type;
mod remove_mut;

fn create_params(uri: Uri, range: Range) -> CodeActionParams {
    CodeActionParams {
        text_document: TextDocumentIdentifier { uri },
        range,
        context: CodeActionContext {
            diagnostics: vec![],
            only: None,
            trigger_kind: None,
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
    }
}
