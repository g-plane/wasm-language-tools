use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn no_name() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func)
  (export (func 0)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 3, 6, 3, 6));
    assert!(response.is_none());
}

#[test]
fn missing_extern_idx() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func)
  (export "" (func)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 3, 6, 3, 6));
    assert!(response.is_none());
}

#[test]
fn undef() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (export "" (func 0)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 6, 2, 6));
    assert!(response.is_none());
}

#[test]
fn func() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (export "" (func 0))
  (func $f))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 6, 2, 6));
    assert_json_snapshot!(response);
}

#[test]
fn global() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (export "g" (global $g))
  (global $g))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 6, 2, 6));
    assert_json_snapshot!(response);
}

#[test]
fn memory() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (memory (export "m"))
  (export "" (memory 0)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 3, 6, 3, 6));
    assert_json_snapshot!(response);
}

#[test]
fn table() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (table)
  (export "" (table 0)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 3, 6, 3, 6));
    assert_json_snapshot!(response);
}

#[test]
fn tag() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (tag $e (param i32))
  (export "" (tag 0)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 3, 6, 3, 6));
    assert_json_snapshot!(response);
}
