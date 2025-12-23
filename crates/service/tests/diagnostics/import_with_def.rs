use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn func() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func (import "" "a") (param i32) (result i32))
  (func (import "" "b") (param i32) (result i32) (local i32) (local)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn global() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (global (import "" "a") i32)
  (global (import "" "b") i32
    i32.const 0))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn memory() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (memory (import "" "a") 1)
  (memory (import "" "b") (data)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn table() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (table (import "" "a") 0 funcref)
  (table (import "" "b") 0 funcref
    ref.func 0)
  (func))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn tag() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (tag (import "" "") (param i32)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
