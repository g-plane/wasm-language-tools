use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn single() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param i32))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 19, 2, 19));
    assert!(response.is_none());
}

#[test]
fn param() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param i32 f64))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 19, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn result() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (result i32 f64))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 20, 2, 20));
    assert_json_snapshot!(response);
}

#[test]
fn local() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (local i32 f64))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 19, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn field() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field i16 (mut f32)))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 25, 2, 25));
    assert_json_snapshot!(response);
}
