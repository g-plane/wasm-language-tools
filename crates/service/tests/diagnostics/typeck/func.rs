use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn param() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param i64) (result i32)
        (i32.add (local.get 0) (i32.const 1))
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
fn results_incorrect() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result i32 i32)
    block (result i32)
      unreachable
    end)
  (func (result i32 i32)
    block (result i32 f32)
      unreachable
    end)
  (func (result i32 i32)
    block (result i32 i32 i32)
      unreachable
    end)
  (func (result i32 i32)
    i32.const 0
    i32.const 0
    f32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn results_correct() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result i32 i32)
    unreachable)
  (func (result i32 i32)
    (f32.const 0)
    (unreachable))
  (func (result i32 i32)
    (f32.const 0)
    (f32.const 0)
    (unreachable))
  (func (result i32 i32)
    block (result i32 i32)
      unreachable
    end))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn sequence_type_mismatch_from_func_params() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $swap (param i32 i32) (result i32 i32)
        local.get 1
        local.get 0)
    (func (param f32 i32) (result i32)
        local.get 0
        local.get 1
        call $swap
        i32.sub)

    (func (param f32 f64))
    (func
        i32.const 0
        call 2))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn sequence_type_mismatch_from_func_results() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $swap (param i32 i32) (result i32 f32)
        local.get 1
        local.get 0)
    (func (param i32 i32) (result i32)
        local.get 0
        local.get 1
        call $swap
        i32.sub))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
