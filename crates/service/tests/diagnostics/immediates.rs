use super::*;
use insta::assert_json_snapshot;
use lsp_types::Uri;
use wat_service::LanguageService;

#[test]
fn index() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func
        (call \"\")
        (local.get 1.0) (drop)
        (local.set 1.0 (i32.const 0))
        (global.get 1.0) (drop)
        (global.set \"\" (i32.const 0))
        (call)
        (local.get) (drop)
        (local.set (i32.const 0))
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn int() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (result i32 i64 v128 i32 i64 v128)
        (i32.const 1.0)
        (i64.const 1.0)
        (v128.const 1.0)
        (i32.const)
        (i64.const)
        (v128.const)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn float() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (result f32 f64 f32 f64)
        (f32.const 1)
        (f64.const $a)
        (f32.const)
        (f64.const)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn indexes() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func
        (table.copy 1.0 1.0 (i32.const 1) (i32.const 1) (i32.const 1))
        (table.init $a \"\" (i32.const 1) (i32.const 1) (i32.const 1))
    )
    (table $a 0 funcref)
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn mem_arg() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func
        (i32.load 1 (i32.const 0)) (drop)
        (f64.store 1 (i32.const 0) (f64.const 0.0))
        (drop (i32.load (i32.const 0)))
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn mem_arg_and_index() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func
        (v128.load8_lane 1 \"\" (i32.const 0) (v128.const 0))
        (drop)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn br_table() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
  (func
    (block $a
      (br_table 0 1.0 $a "" (unreachable))))
  (func
    block
      i32.const 0
      br_table
    end))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn select_incorrect() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
  (type $t (func (result i32)))
  (func
    (drop
      (select $t
        (unreachable))))
  (func
    (drop
      (select (type $t)
        (unreachable))))
  (func
    (drop
      (select (result i32) $t
        (unreachable))))
  (func
    (drop
      (select (param i32) (result i32)
        (unreachable))))
  (func
    (drop
      (select (result i32) (result)
        (unreachable))))
  (func
    (drop
      (select (result)
        (unreachable))))
  (func
    (drop
      (select (result i32 i32)
        (unreachable)))))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn select_correct() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
  (func
    (select
      (i32.const 0)
      (i32.const 1)
      (i32.const 2))
    (drop
      (select (result i32)
        (unreachable)))))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn call_indirect_incorrect() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
  (func
    i32.const 0
    call_indirect
    i32.const 0
    call_indirect 0.0
    i32.const 0
    call_indirect 0.0 0
    i32.const 0
    call_indirect 0.0 (param)
    i32.const 0
    call_indirect 0
    i32.const 0
    call_indirect 0 0.0)
  (table 0 funcref))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn call_indirect_correct() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
  (type (func))
  (func
    i32.const 0
    call_indirect 0 (type 0)
    i32.const 0
    call_indirect 0 (param)
    i32.const 0
    call_indirect 0 (param) (param)
    i32.const 0
    call_indirect 0 (result)
    i32.const 0
    call_indirect 0 (result) (result)
    i32.const 0
    call_indirect 0 (param) (result)
    i32.const 0
    call_indirect (type 0)
    i32.const 0
    call_indirect (param)
    i32.const 0
    call_indirect (param) (param)
    i32.const 0
    call_indirect (result)
    i32.const 0
    call_indirect (result) (result)
    i32.const 0
    call_indirect (param) (result))
  (table 0 funcref))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn expected_instr() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (result i32) (i32.add 1 (i32.const 0))))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
