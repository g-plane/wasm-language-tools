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
fn valid() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (tag)
  (tag)
  (func
    block
      try_table (catch 0 1) (catch 1 2) (catch_all 1)
      end
    end))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn after_catch() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (tag $e)
  (func
    block
      try_table (catch 0 1) (catch $e 2) (catch_all 1)
      end
    end))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn after_catch_all() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (tag)
  (func
    block
      try_table (catch_all 1) (catch 0 2) (catch_all 2)
      end
    end))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn after_catch_and_catch_all() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (tag $e)
  (func
    block
      try_table (catch $e 1) (catch_all 1) (catch 0 2)
      end
    end))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
