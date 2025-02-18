use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn blocks() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (fnuc))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn top_level_error_token() {
    let uri = "untitled:test".to_string();
    let source = "(module))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
