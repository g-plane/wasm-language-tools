use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn func() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (func))
  (func
    (block (type 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn r#struct() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct))
  (func
    (block (type 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn array() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (array i32))
  (func
    (block (type 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
