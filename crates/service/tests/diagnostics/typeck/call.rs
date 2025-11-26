use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn type_mismatch_from_func_results() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $getTwo (result i64 i32)
        (i64.const 2) (i32.const 3)
    )
    (func $add (result i32)
        (i32.add (call $getTwo))
    )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn call_type_mismatch() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $f1 (param f32))
    (func (call $f1 (i32.const 0)))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn return_call_mismatch() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $f1 (param f32))
    (func (return_call $f1 (i32.const 0)))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn call_ref_match() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (func (param i32)))
  (type (func (param i32)))
  (func (param (ref 1))
    i32.const 0
    local.get 0
    call_ref 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn call_ref_mismatch() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (func (param f32) (result f64)))
  (type (func (param f32) (result f64)))
  (func
    call_ref 0)
  (func (param (ref 1))
    local.get 0
    call_ref 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn return_call_ref_match() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (func (param i32)))
  (type (func (param i32)))
  (func (param (ref 1))
    i32.const 0
    local.get 0
    return_call_ref 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn return_call_ref_mismatch() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (func (param f32) (result f64)))
  (type (func (param f32) (result f64)))
  (func
    return_call_ref 0)
  (func (param (ref 1))
    local.get 0
    return_call_ref 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
