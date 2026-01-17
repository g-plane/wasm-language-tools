use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn no_starts() {
    let uri = "untitled:test".to_string();
    let source = "(module)";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn one_start() {
    let uri = "untitled:test".to_string();
    let source = "(module (func) (start 0))";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn many_starts() {
    let uri = "untitled:test".to_string();
    let source = "(module (func) (start 0) (start 0))";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn different_modules() {
    let uri = "untitled:test".to_string();
    let source = "
(module (func) (start 0))
(module (func) (start 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    service.set_config(
        &uri,
        Some(ServiceConfig {
            lint: Lints {
                unused: LintLevel::Allow,
                multi_modules: LintLevel::Allow,
                ..Default::default()
            },
            ..Default::default()
        }),
    );
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
