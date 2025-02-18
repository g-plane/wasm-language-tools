use super::*;
use insta::assert_json_snapshot;
use lspt::{Position, Range};
use wat_service::LanguageService;

#[test]
fn single() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param i32))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 2,
                character: 19,
            },
            end: Position {
                line: 2,
                character: 19,
            },
        },
    ));
    assert!(response.is_none());
}

#[test]
fn param() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param i32 f64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 2,
                character: 19,
            },
            end: Position {
                line: 2,
                character: 19,
            },
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn result() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (result i32 f64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 2,
                character: 20,
            },
            end: Position {
                line: 2,
                character: 20,
            },
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn local() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (local i32 f64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 2,
                character: 19,
            },
            end: Position {
                line: 2,
                character: 19,
            },
        },
    ));
    assert_json_snapshot!(response);
}
