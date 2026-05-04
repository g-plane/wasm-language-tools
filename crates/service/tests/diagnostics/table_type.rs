use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn overflow() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (import "" "a" (table 9999999999 99999999999999999999999 funcref))
  (import "" "b" (table i64 9999999999 99999999999999999999999 funcref))
  (table 1000000 1000000 funcref)
  (table 9999999999 99999999999999999999999 funcref)
  (table i64 9999999999 99999999999999999999999 funcref))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn less_than() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table 2 1 funcref))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn valid() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table 1 1 funcref)
  (table 2 3 funcref))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
