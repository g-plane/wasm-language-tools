use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn invalid() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global (mut i32)
    (i32.add
      (i32.const 1)
      (i32.const 2))
    global.set 0
    block
    end
    (loop)
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn valid() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global i32
    i32.const 0)
  (global i32
    global.get 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
