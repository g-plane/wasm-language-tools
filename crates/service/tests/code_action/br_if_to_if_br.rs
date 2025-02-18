use super::*;
use insta::assert_json_snapshot;
use lspt::{Position, Range};
use wat_service::LanguageService;

#[test]
fn not_br_if() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    br_table 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 3,
                character: 9,
            },
            end: Position {
                line: 3,
                character: 9,
            },
        },
    ));
    assert!(response.is_none());
}

#[test]
fn sequence_without_condition() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    br_if 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 3,
                character: 9,
            },
            end: Position {
                line: 3,
                character: 9,
            },
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn sequence_with_condition() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    i32.const 0
    br_if 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 4,
                character: 9,
            },
            end: Position {
                line: 4,
                character: 9,
            },
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn folded_without_condition() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (br_if 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 3,
                character: 9,
            },
            end: Position {
                line: 3,
                character: 9,
            },
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn folded_with_single_condition() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (br_if 0 (i32.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 3,
                character: 9,
            },
            end: Position {
                line: 3,
                character: 9,
            },
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn folded_with_multi_conditions() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (br_if 0 (i32.const 0) (i32.const 1))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 3,
                character: 9,
            },
            end: Position {
                line: 3,
                character: 9,
            },
        },
    ));
    assert_json_snapshot!(response);
}
