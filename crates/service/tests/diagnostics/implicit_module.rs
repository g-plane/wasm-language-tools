use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

fn disable_other_lints(service: &mut LanguageService, uri: &str) {
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
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn top_level_module_fields() {
    let uri = "untitled:test".to_string();
    let source = "(func) (global i32 i32.const 0)";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
