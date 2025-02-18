use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn folded() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param i32 f32) (result i32)
    (loop (result i32)
      (i32.add
        (local.get 0)
        (local.get 1)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn sequence() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param i32 f32) (result i32)
    loop (result i32)
      local.get 0
      local.get 1
      i32.add
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn results_folded() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result i32 i32)
    (loop (result i32 i32)
      (i32.const 0)))
  (func (result i32 i32)
    (loop (result i32 i32)
      (i32.const 0)
      (f32.const 0)))
  (func (result i32 i32)
    (loop (result i32 i32)
      (i32.const 0)
      (i32.const 0)
      (i32.const 0)))
  (func (result i32 i32)
    (loop (result i32 i32)
      (i32.const 0)
      (i32.const 0)
      (f32.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn results_sequence() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result i32 i32)
    loop (result i32 i32)
      i32.const 0
    end)
  (func (result i32 i32)
    loop (result i32 i32)
      i32.const 0
      f32.const 0
    end)
  (func (result i32 i32)
    loop (result i32 i32)
      i32.const 0
      i32.const 0
      i32.const 0
    end)
  (func (result i32 i32)
    loop (result i32 i32)
      i32.const 0
      i32.const 0
      f32.const 0
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn results_correct() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result i32 i32)
    (loop (result i32 i32)
      unreachable))
  (func (result i32 i32)
    (loop (result i32 i32)
      (i32.const 0)
      (f32.const 0)
      (unreachable)))
  (func (result i32 i32)
    (loop (result i32 i32)
      (f32.const 0)
      (i32.const 0)
      (unreachable)))
  (func (result i32 i32)
    (loop (result i32 i32)
      (i32.const 0)
      (i32.const 0)))
  (func (result i32 i32)
    loop (result i32 i32)
      unreachable
    end)
  (func (result i32 i32)
    loop (result i32 i32)
      i32.const 0
      f32.const 0
      unreachable
    end)
  (func (result i32 i32)
    loop (result i32 i32)
      f32.const 0
      i32.const 0
      unreachable
    end)
  (func (result i32 i32)
    loop (result i32 i32)
      i32.const 0
      i32.const 0
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
