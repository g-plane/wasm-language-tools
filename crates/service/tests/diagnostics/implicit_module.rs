use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

fn configure(service: &mut LanguageService, uri: String) {
    service.set_config(
        uri,
        Some(ServiceConfig {
            lint: Lints {
                unused: LintLevel::Allow,
                implicit_module: LintLevel::Deny,
                ..Default::default()
            },
            ..Default::default()
        }),
    );
}

#[test]
fn incomplete_module() {
    let uri = "untitled:test".to_string();
    let source = "(module";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    configure(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn top_level_module_fields() {
    let uri = "untitled:test".to_string();
    let source = "(func) (global i32 i32.const 0)";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    configure(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
