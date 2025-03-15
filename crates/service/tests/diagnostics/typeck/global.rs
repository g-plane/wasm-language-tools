use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn set() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global (mut i32) i32.const 0)
  (func
    f32.const 0
    global.set 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
