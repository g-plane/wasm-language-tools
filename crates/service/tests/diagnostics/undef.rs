use super::*;
use insta::assert_json_snapshot;
use lsp_types::Uri;
use wat_service::LanguageService;

#[test]
fn call_defined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func $foo (call $foo)))";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn call_undefined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func $foo (call $bar)))";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn local_defined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (param $p i32) (local.get 0)))";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn local_undefined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (local.get 0)))";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn type_use_defined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (type $t)) (type $t (func)))";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn type_use_undefined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (type $t)))";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn global_defined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (global.get $foo)) (global $foo i32))";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn global_undefined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (global.get $bar)))";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn memory_defined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (memory 1) (export \"\" (memory 0)))";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn memory_undefined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (export \"\" (memory 0)))";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
