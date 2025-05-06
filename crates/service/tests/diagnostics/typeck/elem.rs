use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn invalid() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (type $f1 (func))
  (type $f2 (func (param i32)))
  (type $f3 (func (result i32)))
  (elem (ref null $f1)
    (item
      ref.null $f2)
    (item
      ref.null $f3)))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn valid() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (type $f1 (func))
  (elem (ref null $f1)
    (item
      ref.null $f1)))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
