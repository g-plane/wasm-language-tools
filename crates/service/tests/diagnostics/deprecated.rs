use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

fn disable_other_lints(service: &mut LanguageService, uri: &str) {
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
fn func() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (@deprecated)
  (import "" "" (func))
  (func)
  (@deprecated "this is deprecated")
  (func $f)
  (func
    call 0
    call 1
    call $f))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn types() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (rec
    (@deprecated) (type (func)))
  (type (func))
  (@deprecated "this is deprecated")
  (type $t (func))
  (func (type 0))
  (func (type $t)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn global() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (@deprecated)
  (import "" "" (global i32))
  (global i32
    i32.const 0)
  (@deprecated "this is deprecated")
  (global $g i32
    i32.const 0)
  (func
    global.get 0
    drop
    global.get $g
    drop))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn memory() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (@deprecated)
  (import "" "" (memory 0))
  (memory 0)
  (@deprecated "this is deprecated")
  (memory $m 0)
  (func
    memory.size 0
    drop
    memory.size $m
    drop))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn table() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (@"deprecated")
  (import "" "" (table 0 funcref))
  (table 0 funcref)
  (@"deprecated" "this is deprecated")
  (table $t 0 funcref)
  (func
    table.size 0
    drop
    table.size $t
    drop))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn tag() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (@deprecated)
  (; comment ;)
  (import "" "" (tag))
  (tag)
  (@deprecated "this is deprecated")
  ;; comment
  (tag $e)
  (func
    try_table (catch 0 0) (catch $e 0)
    end))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
