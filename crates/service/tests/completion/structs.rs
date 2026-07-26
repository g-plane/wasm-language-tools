use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn structs() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $a (array i32))
  (type (struct))
  (type $b (struct))
  (func
    struct.new ))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 6, 15));
    assert_json_snapshot!(response);
}

#[test]
fn structs_following_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $a (array i32))
  (type (struct))
  (type $b (struct))
  (func
    struct.new 1))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 6, 16));
    assert_json_snapshot!(response);
}

#[test]
fn structs_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $a (array i32))
  (type (struct))
  (type $b (struct))
  (func
    struct.new $))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 6, 16));
    assert_json_snapshot!(response);
}

#[test]
fn structs_following_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $a (array i32))
  (type (struct))
  (type $b (struct))
  (func
    struct.new $b))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 6, 17));
    assert_json_snapshot!(response);
}
