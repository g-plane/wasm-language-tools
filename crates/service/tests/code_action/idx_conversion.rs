use super::*;
use insta::assert_json_snapshot;
use lspt::{Position, Range, Uri};
use wat_service::LanguageService;

#[test]
fn not_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (i32.const 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range { start: Position { line: 3, character: 16 }, end: Position { line: 3, character: 16 } },
    ));
    assert!(response.is_none());
}

#[test]
fn not_defined() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (call 1)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range { start: Position { line: 3, character: 11 }, end: Position { line: 3, character: 11 } },
    ));
    assert!(response.is_none());
}

#[test]
fn no_name() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (call 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range { start: Position { line: 3, character: 11 }, end: Position { line: 3, character: 11 } },
    ));
    assert!(response.is_none());
}

#[test]
fn ident_to_num() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f
    (call $f)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range { start: Position { line: 3, character: 11 }, end: Position { line: 3, character: 11 } },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn num_to_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f)
  (start 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range { start: Position { line: 3, character: 10 }, end: Position { line: 3, character: 10 } },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn param() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param $p i32) (local $l i32)
    (local.get 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range { start: Position { line: 3, character: 16 }, end: Position { line: 3, character: 16 } },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn local() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param $p i32) (local $l i32)
    (local.get $l)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range { start: Position { line: 3, character: 16 }, end: Position { line: 3, character: 16 } },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn block() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block $a
      (loop $b
        (br 1)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range { start: Position { line: 5, character: 13 }, end: Position { line: 5, character: 13 } },
    ));
    assert_json_snapshot!(response);
}
