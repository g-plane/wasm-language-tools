use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn incorrect() {
    let uri = "untitled:test".to_string();
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
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn correct() {
    let uri = "untitled:test".to_string();
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
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
