use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

fn disable_other_lints(service: &mut LanguageService, uri: String) {
    service.set_config(
        uri,
        ServiceConfig {
            lint: Lints {
                unused: LintLevel::Allow,
                multi_memories: LintLevel::Deny,
                ..Default::default()
            },
            ..Default::default()
        },
    );
}

#[test]
fn no_memories() {
    let uri = "untitled:test".to_string();
    let source = "(module)";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn one_memory() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn many_memories() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 0)
  (memory 1))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn allowed_by_config() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 0)
  (memory 1))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
