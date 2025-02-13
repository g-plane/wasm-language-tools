use super::*;
use insta::assert_json_snapshot;
use lsp_types::Uri;
use wat_service::LanguageService;

#[test]
fn block_type_in_stack() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (param i32 i32) (result i32)
    (block (result i32 i32)
      (local.get 0)
      (local.get 1))
    i32.add))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn block_type_folded() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (param i32 i32) (result i32)
    (i32.add
      (block (result i32 i32)
        (local.get 0)
        (local.get 1)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn folded() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (param i32 f32) (result i32)
    (block (result i32)
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
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (param i32 f32) (result i32)
    block (result i32)
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
fn new_stack_for_new_block() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (result i32)
    i32.const 0
    i32.const 1
    block (result i32)
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
fn params_boundary_missing() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    i32.const 0
    block (param i32 i32)
      i32.add
      drop
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn params_boundary_mismatched() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    i32.const 0
    f32.const 1
    block (param i32 i32)
      i32.add
      drop
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn params_mismatched() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    i32.const 0
    f32.const 1
    block (param i32 f32)
      i32.add
      drop
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn params_correct() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    i32.const 0
    i32.const 1
    block (param i32 i32)
      i32.add
      drop
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn results_folded() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (result i32 i32)
    (block (result i32 i32)
      (i32.const 0)))
  (func (result i32 i32)
    (block (result i32 i32)
      (i32.const 0)
      (f32.const 0)))
  (func (result i32 i32)
    (block (result i32 i32)
      (i32.const 0)
      (i32.const 0)
      (i32.const 0)))
  (func (result i32 i32)
    (block (result i32 i32)
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
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (result i32 i32)
    block (result i32 i32)
      i32.const 0
    end)
  (func (result i32 i32)
    block (result i32 i32)
      i32.const 0
      f32.const 0
    end)
  (func (result i32 i32)
    block (result i32 i32)
      i32.const 0
      i32.const 0
      i32.const 0
    end)
  (func (result i32 i32)
    block (result i32 i32)
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
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (type $t (func (result i32 i32)))
  (func (result i32 i32)
    (block (result i32 i32)
      unreachable))
  (func (result i32 i32)
    (block (result i32 i32)
      (i32.const 0)
      (f32.const 0)
      (unreachable)))
  (func (result i32 i32)
    (block (result i32 i32)
      (f32.const 0)
      (i32.const 0)
      (unreachable)))
  (func (result i32 i32)
    (block (result i32 i32)
      (i32.const 0)
      (i32.const 0)))
  (func (result i32 i32)
    (block (type $t)
      (i32.const 0)
      (i32.const 0)))
  (func (result i32 i32)
    block (result i32 i32)
      unreachable
    end)
  (func (result i32 i32)
    block (result i32 i32)
      i32.const 0
      f32.const 0
      unreachable
    end)
  (func (result i32 i32)
    block (result i32 i32)
      f32.const 0
      i32.const 0
      unreachable
    end)
  (func (result i32 i32)
    block (result i32 i32)
      i32.const 0
      i32.const 0
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}
