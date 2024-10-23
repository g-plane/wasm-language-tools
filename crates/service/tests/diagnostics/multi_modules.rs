use super::*;
use insta::assert_json_snapshot;
use lsp_types::Uri;
use wat_service::LanguageService;

#[test]
fn no_modules() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn one_module() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module)";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn many_modules() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module) (module) (module)";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
