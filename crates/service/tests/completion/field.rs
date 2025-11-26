use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn after_typeidx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field $x i32) (field $y (mut (ref 0))) (field f32 f64)))
  (func
    struct.get 0 ))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 4, 17));
    assert_json_snapshot!(response);
}

#[test]
fn following_int() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field $x i32) (field $y (mut (ref 0))) (field f32 f64)))
  (func
    struct.get 0 2))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 4, 18));
    assert_json_snapshot!(response);
}

#[test]
fn following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field $x i32) (field $y (mut (ref 0))) (field f32 f64)))
  (func
    struct.get 0 $))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 4, 18));
    assert_json_snapshot!(response);
}

#[test]
fn following_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field $x i32) (field $y (mut (ref 0))) (field f32 f64)))
  (func
    struct.get 0 $x))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 4, 19));
    assert_json_snapshot!(response);
}

#[test]
fn different_structs() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field $x i32) (field $y (mut (ref 0))) (field f32 f64)))
  (type (struct (field $x i64)))
  (func
    struct.set 1 ))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 17));
    assert_json_snapshot!(response);
}
