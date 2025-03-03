use insta::assert_json_snapshot;
use lspt::{DeclarationParams, Position, TextDocumentIdentifier};
use wat_service::LanguageService;

fn create_params(uri: String, position: Position) -> DeclarationParams {
    DeclarationParams {
        text_document: TextDocumentIdentifier { uri },
        position,
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
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
        .goto_declaration(create_params(
            uri.clone(),
            Position {
                line: 1,
                character: 4
            }
        ))
        .is_none());
    assert!(service
        .goto_declaration(create_params(
            uri.clone(),
            Position {
                line: 2,
                character: 29
            }
        ))
        .is_none());
    assert!(service
        .goto_declaration(create_params(
            uri.clone(),
            Position {
                line: 3,
                character: 7
            }
        ))
        .is_none());
    assert!(service
        .goto_declaration(create_params(
            uri.clone(),
            Position {
                line: 3,
                character: 25
            }
        ))
        .is_none());
    assert!(service
        .goto_declaration(create_params(
            uri.clone(),
            Position {
                line: 4,
                character: 14
            }
        ))
        .is_none());
    assert!(service
        .goto_declaration(create_params(
            uri.clone(),
            Position {
                line: 4,
                character: 23
            }
        ))
        .is_none());
}

#[test]
fn func_not_defined() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func
        (call 1) (call $func)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    assert!(service
        .goto_declaration(create_params(
            uri.clone(),
            Position {
                line: 3,
                character: 15
            }
        ))
        .is_none());
    assert!(service
        .goto_declaration(create_params(
            uri.clone(),
            Position {
                line: 3,
                character: 25
            }
        ))
        .is_none());
}

#[test]
fn func_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func
        (call 0)
    )
)
(module (func))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_declaration(create_params(
        uri,
        Position {
            line: 3,
            character: 15,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn func_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $func
        (call $func)
    )
)
(module (func $func))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.goto_declaration(create_params(
        uri,
        Position {
            line: 3,
            character: 18,
        },
    ));
    assert_json_snapshot!(response);
}
