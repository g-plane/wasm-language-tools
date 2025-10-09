use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn index() {
    let uri = "untitled:test".to_string();
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
        (drop (struct.new))
        (drop (array.new (i32.const 0)))
        (drop (array.get))
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
    let uri = "untitled:test".to_string();
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
    let uri = "untitled:test".to_string();
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
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func
        (table.copy 1.0 1.0 (i32.const 1) (i32.const 1) (i32.const 1))
        (table.init $a \"\" (i32.const 1) (i32.const 1) (i32.const 1))
        (drop (struct.get))
        (drop (struct.get 0))
        (array.copy)
    )
    (table $a 0 funcref)
    (type (struct))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn mem_arg_incorrect() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    i32.const 0
    i32.load 0.0
    drop
    i32.const 0
    f32.load 0.0 0
    drop
    i32.const 0
    i64.load 0.0 offset=0
    drop
    i32.const 0
    f64.load 0 0.0
    drop)
  (memory 1))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn mem_arg_correct() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    i32.const 0
    i32.load 0
    drop
    i32.const 0
    f32.load 0 offset=0
    drop
    i32.const 0
    i64.load 0 align=0
    drop
    i32.const 0
    f64.load
    drop
    i32.const 0
    v128.load offset=0
    drop
    i32.const 0
    i32.const 0
    i32.store align=0)
  (memory 1))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn v128_load_store_incorrect() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    i32.const 0
    v128.const 0
    v128.load8_lane 0.0
    drop
    i32.const 0
    v128.const 0
    v128.load16_lane 0.0 0
    drop
    i32.const 0
    v128.const 0
    v128.load32_lane 0.0 offset=0
    drop
    i32.const 0
    v128.const 0
    v128.store8_lane 0 0.0
    i32.const 0
    v128.const 0
    v128.store16_lane 0 align=0 0.0)
  (memory 1))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn v128_load_store_correct() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    i32.const 0
    v128.const 0
    v128.load8_lane 0
    drop
    i32.const 0
    v128.const 0
    v128.load16_lane 0 offset=0
    drop
    i32.const 0
    v128.const 0
    v128.load32_lane 0 align=0
    drop
    i32.const 0
    v128.const 0
    v128.load64_lane 0 align=0 1
    drop
    i32.const 0
    v128.const 0
    v128.store8_lane
    i32.const 0
    v128.const 0
    v128.store16_lane offset=0
    i32.const 0
    v128.const 0
    v128.store32_lane align=0 1)
  (memory 1))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn br_table() {
    let uri = "untitled:test".to_string();
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
    let uri = "untitled:test".to_string();
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
    let uri = "untitled:test".to_string();
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
    assert!(response.items.is_empty());
}

#[test]
fn call_indirect_incorrect() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func
    i32.const 0
    call_indirect 0.0
    i32.const 0
    call_indirect 0.0 0
    i32.const 0
    call_indirect 0.0 (param)
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
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (type (func))
  (func
    i32.const 0
    call_indirect 0
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
    call_indirect
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
    assert!(response.items.is_empty());
}

#[test]
fn memory_grow_and_fill() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (memory 1)
  (func
    i32.const 0
    memory.grow
    drop
    i32.const 0
    memory.grow 0
    drop
    i32.const 0
    memory.grow 0.0
    drop

    i32.const 0
    i32.const 0
    i32.const 0
    memory.fill
    i32.const 0
    i32.const 0
    i32.const 0
    memory.fill 0
    i32.const 0
    i32.const 0
    i32.const 0
    memory.fill 0.0))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn memory_copy() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (memory 1)
  (func
    i32.const 0
    i32.const 0
    i32.const 0
    memory.copy

    i32.const 0
    i32.const 0
    i32.const 0
    memory.copy 0

    i32.const 0
    i32.const 0
    i32.const 0
    memory.copy 0 0

    i32.const 0
    i32.const 0
    i32.const 0
    memory.copy 0.0

    i32.const 0
    i32.const 0
    i32.const 0
    memory.copy 0 0.0

    i32.const 0
    i32.const 0
    i32.const 0
    memory.copy 0.0 0.0))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn memory_init() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (memory 1)
  (func
    i32.const 0
    i32.const 0
    i32.const 0
    memory.init

    i32.const 0
    i32.const 0
    i32.const 0
    memory.init 0

    i32.const 0
    i32.const 0
    i32.const 0
    memory.init 0 0

    i32.const 0
    i32.const 0
    i32.const 0
    memory.init 0.0

    i32.const 0
    i32.const 0
    i32.const 0
    memory.init 0 0.0

    i32.const 0
    i32.const 0
    i32.const 0
    memory.init 0.0 0.0))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn ref_type() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (type (struct))
  (func (param (ref 0))
    local.get 0
    ref.test anyref
    drop
    local.get 0
    ref.cast (ref 0)
    drop
    local.get 0
    ref.test (ref any)
    drop
    local.get 0
    ref.cast (ref null any)
    drop
    local.get 0
    ref.test 0
    drop))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn array_new_fixed() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (type $a (array i32))
  (func
    array.new_fixed
    array.new_fixed 0
    array.new_fixed 0 0
    array.new_fixed $a $a
    unreachable))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn br_on_cast() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (type $a (array i32))
  (func
    block
      br_on_cast
      br_on_cast_fail 0.0
      br_on_cast 0
      br_on_cast_fail 0 $a
      br_on_cast 0 (ref $a)
      br_on_cast_fail 0 (ref $a) $a
      br_on_cast_fail 0 (ref $a) anyref
    end))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn i8x16_shuffle() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func (param v128)
    local.get 0
    local.get 0
    i8x16.shuffle 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0
    drop
    local.get 0
    local.get 0
    i8x16.shuffle 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 0
    drop
    local.get 0
    local.get 0
    i8x16.shuffle 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1
    drop
    local.get 0
    local.get 0
    i8x16.shuffle 32 14 13 12 11 10 9 8 7 6 5 4 3 2 1 33
    drop))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn expected_instr() {
    let uri = "untitled:test".to_string();
    let source = "(module (func (result i32) (i32.add 1 (i32.const 0))))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
