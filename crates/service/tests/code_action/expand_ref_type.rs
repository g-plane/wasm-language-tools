use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn anyref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param anyref)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 18, 2, 18));
    assert_json_snapshot!(response);
}

#[test]
fn eqref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param eqref)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 18, 2, 18));
    assert_json_snapshot!(response);
}

#[test]
fn i31ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param i31ref)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 18, 2, 18));
    assert_json_snapshot!(response);
}

#[test]
fn structref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param structref)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 18, 2, 18));
    assert_json_snapshot!(response);
}

#[test]
fn arrayref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param arrayref)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 18, 2, 18));
    assert_json_snapshot!(response);
}

#[test]
fn nullref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param nullref)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 18, 2, 18));
    assert_json_snapshot!(response);
}

#[test]
fn funcref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param funcref)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 18, 2, 18));
    assert_json_snapshot!(response);
}

#[test]
fn nullfuncref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param nullfuncref)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 18, 2, 18));
    assert_json_snapshot!(response);
}

#[test]
fn exnref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param exnref)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 18, 2, 18));
    assert_json_snapshot!(response);
}

#[test]
fn nullexnref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param nullexnref)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 18, 2, 18));
    assert_json_snapshot!(response);
}

#[test]
fn externref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param externref)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 18, 2, 18));
    assert_json_snapshot!(response);
}

#[test]
fn nullexternref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param nullexternref)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 18, 2, 18));
    assert_json_snapshot!(response);
}
