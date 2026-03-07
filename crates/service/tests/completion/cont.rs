use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn cont_def() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (func))
  (type $ct1 (cont $ft1))
  (type $ft2 (func))
  (type (cont ))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 14));
    assert_json_snapshot!(response);
}

#[test]
fn cont_def_following_int() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (func))
  (type $ct1 (cont $ft1))
  (type $ft2 (func))
  (type (cont 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 15));
    assert_json_snapshot!(response);
}

#[test]
fn cont_def_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (func))
  (type $ct1 (cont $ft1))
  (type $ft2 (func))
  (type (cont $))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 15));
    assert_json_snapshot!(response);
}

#[test]
fn cont_def_following_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (func))
  (type $ct1 (cont $ft1))
  (type $ft2 (func))
  (type (cont $f))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 16));
    assert_json_snapshot!(response);
}

#[test]
fn cont_def_sort() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $ft1 (func (param i32)))
  (type $ct1 (cont $ft1))
  (type $ft2 (func (param (ref null $ct3)) (result i32)))
  (type $ct2 (cont $ft2))
  (type $ft3 (func (result i32)))
  (type $ct3 (cont $)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 7, 20));
    assert_json_snapshot!(response);
}
