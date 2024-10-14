use lsp_types::{
    CodeActionContext, CodeActionKind, CodeActionParams, Range, TextDocumentIdentifier, Uri,
};

#[cfg(test)]
mod fix_invalid_mem_arg;

fn create_params(uri: Uri, range: Range) -> CodeActionParams {
    CodeActionParams {
        text_document: TextDocumentIdentifier { uri },
        range,
        context: CodeActionContext {
            diagnostics: vec![],
            only: Some(vec![CodeActionKind::QUICKFIX]),
            trigger_kind: None,
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
    }
}
