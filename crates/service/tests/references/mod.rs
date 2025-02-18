use lspt::{Position, ReferenceContext, ReferenceParams, TextDocumentIdentifier};
use wat_service::LanguageService;

mod block;
mod func;
mod global;
mod local;
mod mem;
mod table;
mod ty;

fn create_params(uri: String, position: Position, include_declaration: bool) -> ReferenceParams {
    ReferenceParams {
        text_document: TextDocumentIdentifier { uri },
        position,
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
        context: ReferenceContext {
            include_declaration,
        },
    }
}

#[test]
fn ignored_tokens() {
    let uri = "untitled:test".to_string();
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
        .find_references(create_params(
            uri.clone(),
            Position {
                line: 1,
                character: 4
            },
            true
        ))
        .is_none());
    assert!(service
        .find_references(create_params(
            uri.clone(),
            Position {
                line: 2,
                character: 29
            },
            true
        ))
        .is_none());
    assert!(service
        .find_references(create_params(
            uri.clone(),
            Position {
                line: 3,
                character: 7
            },
            true
        ))
        .is_none());
    assert!(service
        .find_references(create_params(
            uri.clone(),
            Position {
                line: 3,
                character: 25
            },
            true
        ))
        .is_none());
    assert!(service
        .find_references(create_params(
            uri.clone(),
            Position {
                line: 4,
                character: 14
            },
            true
        ))
        .is_none());
    assert!(service
        .find_references(create_params(
            uri.clone(),
            Position {
                line: 4,
                character: 23
            },
            true
        ))
        .is_none());
}
