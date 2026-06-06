use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn non_compact() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (import "" "" (func (param i32)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 19, 2, 19));
    assert!(response.is_none());
}

#[test]
fn encoding1() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (import "" (item "" (func (param i32))))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 25, 2, 25));
    assert!(response.is_none());
}

#[test]
fn multi() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (import "" (item "") (item) (item "") (func (param i32)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 45, 2, 45));
    assert_json_snapshot!(response);
}

#[test]
fn no_leading_ws() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (import "" (item "") (;;)(func (param i32)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 31, 2, 31));
    assert_json_snapshot!(response);
}
