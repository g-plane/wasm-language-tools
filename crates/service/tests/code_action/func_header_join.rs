use super::*;
use insta::assert_json_snapshot;
use lspt::{Position, Range};
use wat_service::LanguageService;

#[test]
fn not_covered() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param i32) (param f64))
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
fn too_wide() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param i32) (param f64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 2,
                character: 9,
            },
            end: Position {
                line: 2,
                character: 33,
            },
        },
    ));
    assert!(response.is_none());
}

#[test]
fn single() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param i32) (param f64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 2,
                character: 10,
            },
            end: Position {
                line: 2,
                character: 21,
            },
        },
    ));
    assert!(response.is_none());
}

#[test]
fn one_by_one() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param i32) (param f64) (param f32))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 2,
                character: 10,
            },
            end: Position {
                line: 2,
                character: 33,
            },
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn many() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param i32 i64) (param f64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 2,
                character: 10,
            },
            end: Position {
                line: 2,
                character: 37,
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
    (func (result i32 i64) (result f64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 2,
                character: 10,
            },
            end: Position {
                line: 2,
                character: 39,
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
    (func (local i32 i64) (local f64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 2,
                character: 10,
            },
            end: Position {
                line: 2,
                character: 37,
            },
        },
    ));
    assert_json_snapshot!(response);
}
