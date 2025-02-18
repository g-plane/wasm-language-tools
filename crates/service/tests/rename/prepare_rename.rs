use insta::assert_json_snapshot;
use lspt::{Position, PrepareRenameParams, TextDocumentIdentifier};
use wat_service::LanguageService;

fn create_params(uri: String, position: Position) -> PrepareRenameParams {
    PrepareRenameParams {
        text_document: TextDocumentIdentifier { uri },
        position,
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
        .prepare_rename(create_params(
            uri.clone(),
            Position {
                line: 1,
                character: 4
            }
        ))
        .is_none());
    assert!(service
        .prepare_rename(create_params(
            uri.clone(),
            Position {
                line: 2,
                character: 29
            }
        ))
        .is_none());
    assert!(service
        .prepare_rename(create_params(
            uri.clone(),
            Position {
                line: 3,
                character: 7
            }
        ))
        .is_none());
    assert!(service
        .prepare_rename(create_params(
            uri.clone(),
            Position {
                line: 3,
                character: 18
            }
        ))
        .is_none());
    assert!(service
        .prepare_rename(create_params(
            uri.clone(),
            Position {
                line: 4,
                character: 15
            }
        ))
        .is_none());
    assert!(service
        .prepare_rename(create_params(
            uri.clone(),
            Position {
                line: 4,
                character: 23
            }
        ))
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
    let response = service.prepare_rename(create_params(
        uri,
        Position {
            line: 2,
            character: 14,
        },
    ));
    assert_json_snapshot!(response);
}
