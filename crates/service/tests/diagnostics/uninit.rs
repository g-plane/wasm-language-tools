use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn param() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param (ref any))
    local.get 0
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn defaultable() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (local i32 i64 f32 f64 v128 (ref null any))
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    local.get 4
    local.get 5
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn set_then_get() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param (ref any)) (local (ref any))
    local.get 0
    local.set 1
    local.get 1
    unreachable)
  (func (param (ref any)) (local (ref any))
    (local.get 1
      (local.set 1
        (local.get 0)))
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn tee_then_get() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param (ref any)) (local (ref any))
    local.get 0
    local.tee 1
    local.get 1
    unreachable)
  (func (param (ref any)) (local (ref any))
    (local.get 1
      (local.tee 1
        (local.get 0)))
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn undef() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    local.get 0
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn unset() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (local (ref any))
    local.get 0
    unreachable)
  (func (param (ref any)) (local (ref any))
    (local.get 1
      (local.set 1
        (local.get 1)))
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn block_block() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param (ref any)) (local (ref any))
    block
      local.get 1
      drop
      local.get 0
      local.set 1
      local.get 1
      drop
    end
    local.get 1
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn block_loop() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param (ref any)) (local (ref any))
    loop
      local.get 1
      drop
      local.get 0
      local.set 1
      local.get 1
      drop
    end
    local.get 1
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn if_condition() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param (ref any)) (local (ref any))
    (if
      (i32.const 0)
      (local.set 1
        (local.get 0))
      (then
        (drop
          (local.get 1))))
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn then_block() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param (ref any)) (local (ref any))
    (if
      (i32.const 0)
      (then
        (drop
          (local.get 1))
        (local.set 1
          (local.get 0))
        (drop
          (local.get 1))))
    local.get 1
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn else_block() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param (ref any)) (local (ref any))
    (if
      (i32.const 0)
      (then)
      (else
        (drop
          (local.get 1))
        (local.set 1
          (local.get 0))
        (drop
          (local.get 1))))
    local.get 1
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn all_branches() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param (ref any)) (local (ref any))
    (if
      (i32.const 0)
      (then
        (local.set 1
          (local.get 0)))
      (else
        (local.set 1
          (local.get 0))))
    local.get 1
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
