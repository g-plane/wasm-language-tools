use super::*;
use insta::assert_json_snapshot;
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
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 8, 24));
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
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 8, 25));
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
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 8, 25));
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
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 8, 26));
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
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 8, 29));
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
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 9, 24));
    assert_json_snapshot!(response);
}

#[test]
fn catch() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (tag)
  (func
    block $b
      try_table (catch 0 )
      end
    end))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 25));
    assert_json_snapshot!(response);
}

#[test]
fn catch_all() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (tag)
  (func
    block $b
      try_table (catch_all )
      end
    end))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 27));
    assert_json_snapshot!(response);
}
