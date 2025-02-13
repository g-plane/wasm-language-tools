use super::*;
use insta::assert_json_snapshot;
use lsp_types::Uri;
use wat_service::LanguageService;

#[test]
fn incorrect() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (result i32)
    (block
      (return))
    (unreachable))
  (func (result i32)
    (block
      (f32.const 0)
      (return))
    (unreachable))
  (func (result i32)
    block
      return
    end
    unreachable)
  (func (result i32)
    block
      f32.const 0
      return
    end
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn correct() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (result i32)
    (block
      (i32.const 0)
      (return))
    (unreachable))
  (func (result i32)
    (block
      (i32.const 0)
      (i32.const 0)
      (return))
    (unreachable))
  (func (result i32)
    block
      i32.const 0
      return
    end
    unreachable)
  (func (result i32)
    block
      i32.const 0
      i32.const 0
      return
    end
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}
