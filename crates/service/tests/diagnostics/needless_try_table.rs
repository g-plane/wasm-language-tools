use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

fn disable_other_lints(service: &mut LanguageService, uri: String) {
    service.set_config(
        uri,
        Some(ServiceConfig {
            lint: Lints {
                unused: LintLevel::Allow,
                ..Default::default()
            },
            ..Default::default()
        }),
    );
}

#[test]
fn invalid() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (tag $e)
  (func
    try_table
      (try_table)
    end))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
