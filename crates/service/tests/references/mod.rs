use lsp_types::{
    Position, ReferenceContext, ReferenceParams, TextDocumentIdentifier,
    TextDocumentPositionParams, Uri,
};
use wat_service::LanguageService;

mod block;
mod func;
mod global;
mod local;
mod mem;
mod table;
mod ty;

fn create_params(uri: Uri, position: Position, include_declaration: bool) -> ReferenceParams {
    ReferenceParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position,
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
        context: ReferenceContext {
            include_declaration,
        },
    }
}

#[test]
fn ignored_tokens() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $func (export \"func\")
        (unreachable $func)
        (cal 0) ;; typo
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    assert!(service
        .find_references(create_params(uri.clone(), Position::new(1, 4), true))
        .is_none());
    assert!(service
        .find_references(create_params(uri.clone(), Position::new(2, 29), true))
        .is_none());
    assert!(service
        .find_references(create_params(uri.clone(), Position::new(3, 7), true))
        .is_none());
    assert!(service
        .find_references(create_params(uri.clone(), Position::new(3, 25), true))
        .is_none());
    assert!(service
        .find_references(create_params(uri.clone(), Position::new(4, 14), true))
        .is_none());
    assert!(service
        .find_references(create_params(uri.clone(), Position::new(4, 23), true))
        .is_none());
}
