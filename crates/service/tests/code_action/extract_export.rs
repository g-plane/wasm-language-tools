use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn no_name() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (export))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 13, 2, 13));
    assert!(response.is_none());
}

#[test]
fn ident() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func $func (export "f"))
  (func))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 19, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn func() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func (export "f"))
  (func))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 13, 2, 13));
    assert_json_snapshot!(response);
}

#[test]
fn global() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (global (export "g"))
  (func))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 13, 2, 13));
    assert_json_snapshot!(response);
}

#[test]
fn memory() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (memory (export "m"))
  (func))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 13, 2, 13));
    assert_json_snapshot!(response);
}

#[test]
fn table() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (table (export "t"))
  (func))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 13, 2, 13));
    assert_json_snapshot!(response);
}

#[test]
fn tag() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (tag (export "t"))
  (func))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 13, 2, 13));
    assert_json_snapshot!(response);
}
