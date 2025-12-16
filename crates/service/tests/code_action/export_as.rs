use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn no_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 6, 2, 6));
    assert!(response.is_none());
}

#[test]
fn exported_by_inline() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func $f (export "func")))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 6, 2, 6));
    assert!(response.is_none());
}

#[test]
fn exported_by_module_field() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func $f)
  (export "f" (func $f))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 6, 2, 6));
    assert!(response.is_none());
}

#[test]
fn func() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 6, 2, 6));
    assert_json_snapshot!(response);
}

#[test]
fn global() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global $g)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 6, 2, 6));
    assert_json_snapshot!(response);
}

#[test]
fn memory() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory $m)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 6, 2, 6));
    assert_json_snapshot!(response);
}

#[test]
fn table() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $t)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 6, 2, 6));
    assert_json_snapshot!(response);
}

#[test]
fn tag() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (tag $e)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 6, 2, 6));
    assert_json_snapshot!(response);
}
