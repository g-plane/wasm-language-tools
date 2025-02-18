use super::*;
use insta::assert_json_snapshot;
use lspt::{Position, Range};
use wat_service::LanguageService;

#[test]
fn has_params() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (func (param $a i32)))
  (func (type $t) (param i32)
    (unreachable)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 3,
                character: 11,
            },
            end: Position {
                line: 3,
                character: 11,
            },
        },
    ));
    assert!(response.is_none());
}

#[test]
fn has_results() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (func (result i32)))
  (func (type $t) (result i32)
    (unreachable)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 3,
                character: 11,
            },
            end: Position {
                line: 3,
                character: 11,
            },
        },
    ));
    assert!(response.is_none());
}

#[test]
fn missing_index() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (func (param $a i32) (param i32 i32) (result f64)))
  (func (type)
    (unreachable)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 3,
                character: 11,
            },
            end: Position {
                line: 3,
                character: 11,
            },
        },
    ));
    assert!(response.is_none());
}

#[test]
fn undefined_func_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (func (param $a i32) (param i32 i32) (result f64)))
  (func (type 1)
    (unreachable)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 3,
                character: 11,
            },
            end: Position {
                line: 3,
                character: 11,
            },
        },
    ));
    assert!(response.is_none());
}

#[test]
fn no_func_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t)
  (func (type $t)
    (unreachable)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 3,
                character: 11,
            },
            end: Position {
                line: 3,
                character: 11,
            },
        },
    ));
    assert!(response.is_none());
}

#[test]
fn empty_func_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (func))
  (func (type $t)
    (unreachable)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 3,
                character: 11,
            },
            end: Position {
                line: 3,
                character: 11,
            },
        },
    ));
    assert!(response.is_none());
}

#[test]
fn single_param() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (func (param $a i32) ))
  (func (type 0)
    (unreachable)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 3,
                character: 11,
            },
            end: Position {
                line: 3,
                character: 11,
            },
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn params_and_results() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (func (param $a i32) (param i32 i32) (result f64)))
  (func (type $t)
    (unreachable)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 3,
                character: 11,
            },
            end: Position {
                line: 3,
                character: 11,
            },
        },
    ));
    assert_json_snapshot!(response);
}
