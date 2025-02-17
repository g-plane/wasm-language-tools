use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn correct() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    i64.const 0
    drop))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn incorrect() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    drop))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
