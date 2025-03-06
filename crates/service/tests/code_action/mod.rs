use lspt::{CodeActionContext, CodeActionParams, Position, Range, TextDocumentIdentifier};

mod br_if_to_if_br;
mod fix_invalid_mem_arg;
mod func_header_join;
mod func_header_split;
mod idx_conversion;
mod if_br_to_br_if;
mod inline_func_type;
mod remove_mut;

fn create_params(
    uri: String,
    start_line: u32,
    start_character: u32,
    end_line: u32,
    end_character: u32,
) -> CodeActionParams {
    CodeActionParams {
        text_document: TextDocumentIdentifier { uri },
        range: Range {
            start: Position {
                line: start_line,
                character: start_character,
            },
            end: Position {
                line: end_line,
                character: end_character,
            },
        },
        context: CodeActionContext {
            diagnostics: vec![],
            only: None,
            trigger_kind: None,
        },
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    }
}
