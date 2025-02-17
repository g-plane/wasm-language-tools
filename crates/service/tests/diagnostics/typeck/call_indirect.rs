use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn incorrect() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table 0 funcref)
  (type (func (param f32) (result f64)))
  (func
    call_indirect (type 0))
  (func
    i32.const 0
    call_indirect (type 0))
  (func
    f32.const 0
    call_indirect (type 0))
  (func
    i32.const 0
    f32.const 0
    call_indirect 0 (type 0))
  (func
    call_indirect (param f32) (result f64))
  (func
    i32.const 0
    call_indirect (param f32) (result f64))
  (func
    f32.const 0
    call_indirect (param f32) (result f64))
  (func
    i32.const 0
    f32.const 0
    call_indirect 0 (param f32) (result f64)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn correct() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table 0 funcref)
  (type (func (param f32) (result f64)))
  (func (result f64)
    f32.const 0
    i32.const 0
    call_indirect (type 0))
  (func (result f64)
    f32.const 0
    i32.const 0
    call_indirect (param f32) (result f64)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}
