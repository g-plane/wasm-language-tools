use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn empty() {
    let uri = "untitled:test".to_string();
    let source = "(module)";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn no_imports() {
    let uri = "untitled:test".to_string();
    let source = "(module (func))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn single() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (import "a" "b" (func))
  (func))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn multi() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (import "a" "b" (func))
  (import "c" "d" (func))
  (func))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn single_after_other_fields() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func)
  (import "a" "b" (func)))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn multi_after_other_fields() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func)
  (import "a" "b" (func))
  (import "c" "d" (func)))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
