use insta::assert_json_snapshot;
use lspt::{Position, PrepareRenameParams, TextDocumentIdentifier};
use wat_service::LanguageService;

fn create_params(uri: String, line: u32, character: u32) -> PrepareRenameParams {
    PrepareRenameParams {
        text_document: TextDocumentIdentifier { uri },
        position: Position { line, character },
        work_done_token: Default::default(),
    }
}

#[test]
fn ignored_tokens() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $func (export \"func\")
        (unreachable $func)
        (call 0) ;; comment
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    assert!(service
        .prepare_rename(create_params(uri.clone(), 1, 4))
        .is_none());
    assert!(service
        .prepare_rename(create_params(uri.clone(), 2, 29))
        .is_none());
    assert!(service
        .prepare_rename(create_params(uri.clone(), 3, 7))
        .is_none());
    assert!(service
        .prepare_rename(create_params(uri.clone(), 3, 18))
        .is_none());
    assert!(service
        .prepare_rename(create_params(uri.clone(), 4, 15))
        .is_none());
    assert!(service
        .prepare_rename(create_params(uri.clone(), 4, 23))
        .is_none());
}

#[test]
fn ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $func)
    (start $func)
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.prepare_rename(create_params(uri, 2, 14));
    assert_json_snapshot!(response);
}
