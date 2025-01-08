use super::*;
use insta::assert_json_snapshot;
use lsp_types::Uri;
use wat_service::LanguageService;

#[test]
fn func_unused() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func) (func $f))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn func_used() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
  (func $f
    (call $f))
  (func (export "")))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn param_unused() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
  (func (export "") (param $p i32) (param i32)))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn param_used() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
  (func (export "") (param $p i32) (param i32)
    (local.get 0)
    (local.get 1)
    (drop)
    (drop)))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn local_unused() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
  (func (export "") (local $l i32) (local i32)))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn local_used() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
  (func (export "") (local $l i32) (local i32)
    (local.get 0)
    (local.get 1)
    (drop)
    (drop)))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn type_unused() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
  (type (func))
  (type $t (func)))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn type_used() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
  (type (func))
  (type $t (func))
  (func (export "a") (type 0))
  (func (export "b") (type $t)))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn global_unused() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
  (global i32 i32.const 0)
  (global $g i32 i32.const 0))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn global_used() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
  (global (export "g") i32 i32.const 0)
  (global $g i32 i32.const 0)
  (func (export "f")
    (drop (global.get $g))))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn memory_unused() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
  (memory 0)
  (memory $m 0))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn memory_used() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
  (memory (export "") 0)
  (memory $m 0)
  (data (memory $m)))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn table_unused() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
  (table 0 funcref)
  (table $table 0 funcref))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn table_used() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
  (func (export "func")
    (table.get 1
      (i32.const 0))
    (drop))
  (table 0 funcref)
  (table $table (export "table") 0 funcref))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}
