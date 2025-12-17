use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn typeidx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param (ref null 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 18, 2, 18));
    assert!(response.is_none());
}

#[test]
fn non_nullable() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param (ref func))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 18, 2, 18));
    assert!(response.is_none());
}

#[test]
fn anyref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param (ref null any))))
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
  (func (param (ref null eq))))
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
  (func (param (ref null i31))))
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
  (func (param (ref null struct))))
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
  (func (param (ref null array))))
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
  (func (param (ref null none))))
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
  (func (param (ref null func))))
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
  (func (param (ref null nofunc))))
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
  (func (param (ref null exn))))
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
  (func (param (ref null noexn))))
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
  (func (param (ref null extern))))
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
  (func (param (ref null noextern))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 18, 2, 18));
    assert_json_snapshot!(response);
}
