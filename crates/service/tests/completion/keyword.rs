use super::*;
use insta::assert_json_snapshot;
use lsp_types::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn module_keyword() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(0, 1)));
    assert_json_snapshot!(response);
}

#[test]
fn module_keyword_incomplete() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(mo";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(0, 3)));
    assert_json_snapshot!(response);
}

#[test]
fn module_keyword_in_empty() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = " ";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(0, 1)));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_keyword() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module ())";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(0, 9)));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_keyword_incomplete() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (f)";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(0, 10)));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_keyword_without_paren() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module )";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(0, 8)));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_func_keyword() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func ( (i32.const 0)))";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(0, 15)));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_func_keyword_incomplete() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (l (i32.const 0)))";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(0, 16)));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_func_keyword_after_other_syntax() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (param) ( (i32.const 0)))";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(0, 23)));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_func_keyword_in_middle() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (param) ( (param) (i32.const 0)))";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(0, 23)));
    assert_json_snapshot!(response);
}

#[test]
fn no_module_field_func_keyword_without_paren() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func p))"; // shouldn't provide "param"
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(0, 15)));
    assert_json_snapshot!(response);
}
