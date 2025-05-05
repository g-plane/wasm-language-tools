use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn overflow() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 1000000 1000000)
  (memory 99999999999999999999999 99999999999999999999999))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn less_than() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 2 1))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn overflow_and_less_than() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 1100000 1000000))
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
  (memory 1 1)
  (memory 2 3))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
