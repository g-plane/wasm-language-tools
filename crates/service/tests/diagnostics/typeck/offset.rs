use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn invalid() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (table 0 funcref)
  (elem (table 0)
    (offset
      i64.const 0))

  (memory 1)
  (data (memory 0)
    (i64.const 0)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn valid() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (table 0 funcref)
  (elem (table 0)
    (offset
      i32.const 0))

  (memory 1)
  (data (memory 0)
    (i32.const 0)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
