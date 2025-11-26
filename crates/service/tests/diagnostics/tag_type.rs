use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn result_type() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (import "" "" (tag (result i32)))
  (tag (result i32)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn non_func() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (type (array i32))
  (tag (type 0)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
