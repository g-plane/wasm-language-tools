use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn incorrect() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    select
    drop)
  (func
    i32.const 0
    select
    drop)
  (func
    f32.const 0
    select
    drop)
  (func
    f32.const 0
    f32.const 0
    select
    drop)
  (func
    f32.const 0
    i32.const 0
    select
    drop)
  (func
    f32.const 0
    f64.const 0
    i32.const 0
    select
    drop)
  (func (result f64)
    f64.const 0
    f32.const 0
    i32.const 0
    select)
  (func (result f64)
    f32.const 0
    f32.const 0
    i32.const 0
    select)

  (func
    select (result f64))
  (func
    i32.const 0
    select (result f64)
    drop)
  (func
    f32.const 0
    select (result f64)
    drop)
  (func
    f32.const 0
    f32.const 0
    select (result f64)
    drop)
  (func
    f32.const 0
    f32.const 0
    select (result f32)
    drop)
  (func
    f32.const 0
    i32.const 0
    select (result f32)
    drop)
  (func
    f32.const 0
    f64.const 0
    i32.const 0
    select (result f32)
    drop)
  (func (result f32)
    f64.const 0
    f32.const 0
    i32.const 0
    select (result f32))
  (func (result f64)
    f32.const 0
    f32.const 0
    i32.const 0
    select (result f64)))
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
  (func
    f32.const 0
    f32.const 0
    i32.const 0
    select
    drop)
  (func (result f32)
    f32.const 0
    f32.const 0
    i32.const 0
    select)

  (func
    f32.const 0
    f32.const 0
    i32.const 0
    select (result f32)
    drop)
  (func (result f32)
    f32.const 0
    f32.const 0
    i32.const 0
    select (result f32)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}
