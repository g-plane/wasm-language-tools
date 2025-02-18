use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

fn disable_other_lints(service: &mut LanguageService, uri: String) {
    service.set_config(
        uri,
        ServiceConfig {
            lint: Lints {
                unused: LintLevel::Allow,
                needless_mut: LintLevel::Warn,
                ..Default::default()
            },
            ..Default::default()
        },
    );
}

#[test]
fn immutable_global() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global i32
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn inline_exported_global() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (global (export "") (mut i32)
    i32.const 0))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn module_field_export() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (export "" (global 0))
  (global (mut i32)
    i32.const 0))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn global_set_with_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    i32.const 0
    global.set 0)
  (global $global (mut i32)
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn global_set_with_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    i32.const 0
    global.set $global)
  (global $global (mut i32)
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn global_get_with_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    global.get $global
    drop)
  (global $global (mut i32)
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn global_get_with_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    global.get $global
    drop)
  (global $global (mut i32)
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn unused_global() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global (mut i32)
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn imported_global() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (import "" "" (global $global (mut i32)))
  (func
    global.get $global
    drop))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
