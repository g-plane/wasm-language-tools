use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn null() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct))
  (func (result (ref struct))
    ref.null struct)
  (func (result (ref 0))
    ref.null 0)
  (func (result (ref $s))
    ref.null $s))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn as_non_null() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct))
  (func (result (ref 0))
    ref.as_non_null)
  (func (param (ref null $s)) (result (ref $s))
    local.get 0
    ref.as_non_null))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn test() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param (ref null any)) (result i32)
    local.get 0
    ref.test funcref)
  (func (param anyref) (result i32)
    local.get 0
    ref.test (ref null func))
  (func (param (ref null func))
    local.get 0
    ref.test funcref)
  (func (param (ref struct)) (result i32)
    local.get 0
    ref.test anyref)
  (func (param (ref struct)) (result i32)
    local.get 0
    ref.test (ref null any))
  (func (param (ref null struct)) (result i32)
    local.get 0
    ref.test (ref any))
  (func (param anyref) (result i32)
    local.get 0
    ref.test (ref struct))
  (func (param nullfuncref) (result i32)
    local.get 0
    ref.test funcref)
  (func (param funcref) (result i32)
    local.get 0
    ref.test (ref nofunc)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
