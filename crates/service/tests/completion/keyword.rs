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
