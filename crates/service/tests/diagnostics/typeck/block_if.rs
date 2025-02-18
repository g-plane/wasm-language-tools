use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn then_folded() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param i32 f32) (result i32)
    (if (result i32)
      (i32.const 0)
      (then
        (i32.add
          (local.get 0)
          (local.get 1)))
      (else
        (i32.const 0)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn else_folded() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param i32 f32) (result i32)
    (if (result i32)
      (i32.const 0)
      (then
        (i32.const 0))
      (else
        (i32.add
          (local.get 0)
          (local.get 1))))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn then_sequence() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param i32 f32) (result i32)
    i32.const 0
    if (result i32)
      local.get 0
      local.get 1
      i32.add
    else
      i32.const 0
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn else_sequence() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param i32 f32) (result i32)
    i32.const 0
    if (result i32)
      i32.const 0
    else
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
fn then_results_folded() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (i32.const 0))
      (else
        (unreachable))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (i32.const 0)
        (f32.const 0))
      (else
        (unreachable))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (i32.const 0)
        (i32.const 0)
        (i32.const 0))
      (else
        (unreachable))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (i32.const 0)
        (i32.const 0)
        (f32.const 0))
      (else
        (unreachable)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn then_results_sequence() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      i32.const 0
    else
      unreachable
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      i32.const 0
      f32.const 0
    else
      unreachable
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      i32.const 0
      i32.const 0
      i32.const 0
    else
      unreachable
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      i32.const 0
      i32.const 0
      f32.const 0
    else
      unreachable
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn then_results_correct() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (unreachable))
      (else
        (unreachable))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (i32.const 0)
        (f32.const 0)
        (unreachable))
      (else
        (unreachable))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (f32.const 0)
        (i32.const 0)
        (unreachable))
      (else
        (unreachable))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (i32.const 0)
        (i32.const 0))
      (else
        (unreachable))))
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      unreachable
    else
      unreachable
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      f32.const 0
      i32.const 0
      unreachable
    else
      unreachable
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      i32.const 0
      f32.const 0
      unreachable
    else
      unreachable
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      i32.const 0
      i32.const 0
    else
      unreachable
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn else_results_folded() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then (i32.const 0) (i32.const 0))
      (else
        (i32.const 0))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then (i32.const 0) (i32.const 0))
      (else
        (i32.const 0)
        (f32.const 0))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then (i32.const 0) (i32.const 0))
      (else
        (i32.const 0)
        (i32.const 0)
        (i32.const 0))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then (i32.const 0) (i32.const 0))
      (else
        (i32.const 0)
        (i32.const 0)
        (f32.const 0)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn else_results_sequence() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      i32.const 0
      i32.const 0
    else
      i32.const 0
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      i32.const 0
      i32.const 0
    else
      i32.const 0
      f32.const 0
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      i32.const 0
      i32.const 0
    else
      i32.const 0
      i32.const 0
      i32.const 0
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      i32.const 0
      i32.const 0
    else
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
fn else_results_correct() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (unreachable))
      (else
        (unreachable))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (unreachable))
      (else
        (i32.const 0)
        (i32.const 0))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (unreachable))
      (else
        (f32.const 0)
        (i32.const 0)
        (unreachable))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (unreachable))
      (else
        (i32.const 0)
        (f32.const 0)
        (unreachable))))
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      unreachable
    else
      unreachable
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      unreachable
    else
      f32.const 0
      i32.const 0
      unreachable
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      unreachable
    else
      i32.const 0
      f32.const 0
      unreachable
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      unreachable
    else
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

#[test]
fn missing_then() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (if
      (i32.const 0))
    (if
      (i32.const 0)
      (else))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn missing_else() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (drop
      (if (result i32)
        (i32.const 1)
        (then
          (i32.const 0)))))
  (func
    i32.const 1
    if (result i32)
      i32.const 0
    end)
  (func
    i32.const 1
    if
     ;; missing else is allowed since both branches return nothing
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn if_cond_incorrect() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (if
      (then)))
  (func
    (if
      (f32.const 1)
      (then)))
  (func
    (if
      (i32.add
        (i32.const 0)
        (f32.const 0))
      (then)))
  (func
    (if
      (i32.const 1)
      (i32.const 1)
      (then)))
  (func
    if
    end)
  (func
    f32.const 1
    if
    end)
  (func
    i32.const 1
    i32.const 1
    if
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn if_cond_correct() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (if
      (i32.const 1)
      (then)))
  (func (result f32)
    (if
      (f32.const 1)
      (i32.const 1)
      (then)))
  (func
    i32.const 1
    if
    end)
  (func (result f32)
    f32.const 1
    i32.const 1
    if
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
