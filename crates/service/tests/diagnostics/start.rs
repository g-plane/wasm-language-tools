use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn has_params() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param i32))
  (start 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn has_results() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result i32)
    unreachable)
  (start 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn has_params_and_results() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param i32) (result i32)
    unreachable)
  (start 0))
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
  (func)
  (start 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
