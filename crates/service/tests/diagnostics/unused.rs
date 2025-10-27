use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn prefixed_with_underscore() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func $_ (param $_ i32) (local $_l i32))
  (func $_f (param $_p i32) (local $_ i32))
  (type $_ (func))
  (type $_t (func))
  (global $_ i32
    i32.const 0)
  (global $_g i32
    i32.const 0)
  (memory $_ 0)
  (memory $_m 0)
  (table $_ 0 funcref)
  (table $_t 0 funcref))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn func_unused() {
    let uri = "untitled:test".to_string();
    let source = "(module (func) (func $f))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn func_used() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func $f
    (call $f))
  (func (export "")))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn param_unused() {
    let uri = "untitled:test".to_string();
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
    let uri = "untitled:test".to_string();
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
    assert!(response.items.is_empty());
}

#[test]
fn params_in_imported_func() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func $_ (import "" "") (param i32) (param $p i32)))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn param_is_ref_type() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (type (func))
  (func (export "") (param (ref 0)) (param $p (ref 0))))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn param_via_type_def() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (func (param $a i32) (param $b i32) (result i32)))
  (func $_ (type 0)
    (local.get 1)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn local_unused() {
    let uri = "untitled:test".to_string();
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
    let uri = "untitled:test".to_string();
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
    assert!(response.items.is_empty());
}

#[test]
fn type_unused() {
    let uri = "untitled:test".to_string();
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
    let uri = "untitled:test".to_string();
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
    assert!(response.items.is_empty());
}

#[test]
fn struct_used() {
    let uri = "untitled:test".to_string();
    let source = r"
(module
  (type $s (struct))
  (func $_
    (drop (struct.new 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn array_used() {
    let uri = "untitled:test".to_string();
    let source = r"
(module
  (type $a (array i32))
  (func $_
    (drop
      (array.new_default 0
        (i32.const 1)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn type_used_in_subtyping() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (type $t (sub (struct)))
  (type $_ (sub 0 (struct))))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn global_unused() {
    let uri = "untitled:test".to_string();
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
    let uri = "untitled:test".to_string();
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
    assert!(response.items.is_empty());
}

#[test]
fn memory_unused() {
    let uri = "untitled:test".to_string();
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
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (memory (export "") 0)
  (memory $m 0)
  (data (memory $m)))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn memory_implicit() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func $_
    i32.const 0
    f32.load
    drop)
  (memory 1))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn memory_explicit() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func $_
    i32.const 0
    f32.load 0
    drop)
  (memory 1))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn memory_dot() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func $_
    memory.size 0
    drop)
  (memory 1))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn table_unused() {
    let uri = "untitled:test".to_string();
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
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func (export "func")
    (table.get 0
      (i32.const 0))
    (drop))
  (table 0 funcref)
  (table $table2 (export "table") 0 funcref)

  (table $table3 0 funcref)
  (elem (table 2)
    (i32.const 0) funcref))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn field_unused() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (rec (type (struct (field $x i32) (field (ref 0))))))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn field_used() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (type (struct (field $x i32) (field $_y i32)))
  (func $_ (param (ref 0))
    local.get 0
    struct.get 0 $x
    unreachable))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn tag_unused() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (tag)
  (tag $e))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn tag_used() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (tag)
  (tag $e)
  (func $_
    try_table (catch $e 0)
      throw 0 0
    end))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn call_indirect_implicit() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (table 0 funcref)
  (func (export "")
    f32.const 0
    i32.const 0
    call_indirect (param f32)))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn call_indirect_explicit() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (table 0 funcref)
  (func (export "")
    f32.const 0
    i32.const 0
    call_indirect 0 (param f32)))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
