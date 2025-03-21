use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn global_get() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global i32
    i32.const 0)
  (func (result i32)
    (global.get 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn undef() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (global.set 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn mutable() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global (mut i32)
    i32.const 0)
  (func
    (global.set 0
      (i32.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn immutable() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global i32
    i32.const 0)
  (func
    (global.set 0
      (i32.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn imported_immutable() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (import "" "" (global $global i32))
  (func
    i32.const 0
    global.set $global))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
