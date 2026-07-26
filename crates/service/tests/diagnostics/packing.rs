use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn struct_get() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct (field i8)))
  (func (param (ref $s)) (result i32)
    local.get 0
    struct.get $s 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn struct_get_s() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct (field i32)))
  (func (param (ref $s)) (result i32)
    local.get 0
    struct.get_s $s 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn struct_get_u() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct (field i32)))
  (func (param (ref $s)) (result i32)
    local.get 0
    struct.get_u $s 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn array_get() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $a (array i8))
  (func (param (ref $a)) (result i32)
    local.get 0
    i32.const 0
    array.get $a))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn array_get_s() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $a (array i32))
  (func (param (ref $a)) (result i32)
    local.get 0
    i32.const 0
    array.get_s $a))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn array_get_u() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $a (array i32))
  (func (param (ref $a)) (result i32)
    local.get 0
    i32.const 0
    array.get_u $a))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
