use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn has_types() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field (mut f32)))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 22, 2, 22));
    assert!(response.is_none());
}

#[test]
fn has_error() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field mut))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 22, 2, 22));
    assert!(response.is_none());
}

#[test]
fn has_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param $a)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 12, 2, 12));
    assert!(response.is_none());
}

#[test]
fn has_annotations() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (local (@annot))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 26, 2, 26));
    assert!(response.is_none());
}

#[test]
fn param() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param )))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 12, 2, 12));
    assert_json_snapshot!(response);
}

#[test]
fn result() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result (;;))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 12, 2, 12));
    assert_json_snapshot!(response);
}

#[test]
fn local() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (local ;;
  )))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 12, 2, 12));
    assert_json_snapshot!(response);
}

#[test]
fn field() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct ( field))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 21, 2, 21));
    assert_json_snapshot!(response);
}
