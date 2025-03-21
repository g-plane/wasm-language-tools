use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn no_modules() {
    let uri = "untitled:test".to_string();
    let source = "";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn one_module() {
    let uri = "untitled:test".to_string();
    let source = "(module)";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn many_modules() {
    let uri = "untitled:test".to_string();
    let source = "(module) (module) (module)";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn allowed_by_config() {
    let uri = "untitled:test".to_string();
    let source = "(module) (module) (module)";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    service.set_config(
        uri.clone(),
        ServiceConfig {
            format: Default::default(),
            lint: Lints {
                multi_modules: LintLevel::Allow,
                ..Default::default()
            },
        },
    );
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
