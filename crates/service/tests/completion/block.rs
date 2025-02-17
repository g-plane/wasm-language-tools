use super::*;
use insta::assert_json_snapshot;
use lspt::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn blocks() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block
      (block $a
        (block
          (block
            (block $b
              (br_table ))))))
    (block)
    (block $c)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position { line: 8, character: 24 }));
    assert_json_snapshot!(response);
}

#[test]
fn blocks_following_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block
      (block $a
        (block
          (block
            (block $b
              (br_table 1))))))
    (block)
    (block $c)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position { line: 8, character: 25 }));
    assert_json_snapshot!(response);
}

#[test]
fn blocks_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block
      (block $a
        (block
          (block
            (block $b
              (br_table $))))))
    (block)
    (block $c)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position { line: 8, character: 25 }));
    assert_json_snapshot!(response);
}

#[test]
fn blocks_following_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block
      (block $a
        (block
          (block
            (block $b
              (br_table $a))))))
    (block)
    (block $c)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position { line: 8, character: 26 }));
    assert_json_snapshot!(response);
}

#[test]
fn multiple_indexes() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block
      (block $a
        (block
          (block
            (block $b
              (br_table $b 1 ))))))
    (block)
    (block $c)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position { line: 8, character: 29 }));
    assert_json_snapshot!(response);
}

#[test]
fn block_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (func (result i32)))
  (func
    (loop (type $t)
      (block $a
        (block (param i32 i32) (result i32)
          (block
            (if $b (result i32 i32)
              (br_table ))))))
    (block)
    (block $c)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position { line: 9, character: 24 }));
    assert_json_snapshot!(response);
}
