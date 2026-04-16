use super::create_params;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn instr_name() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.const 0))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn two_slots_instr_op_code() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (v128.store (unreachable)))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn three_slots_instr_op_code() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (f64x2.sqrt (unreachable)))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn select() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (select)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 2, 12));
    assert_json_snapshot!(response);
}

#[test]
fn select_with_result() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (select (result i32))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 2, 12));
    assert_json_snapshot!(response);
}

#[test]
fn ref_test_with_non_null_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    ref.test (ref any)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 3, 8));
    assert_json_snapshot!(response);
}

#[test]
fn ref_test_with_abbr_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    ref.test anyref))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 3, 8));
    assert_json_snapshot!(response);
}

#[test]
fn ref_test_with_null_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    ref.test (ref null any)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 3, 8));
    assert_json_snapshot!(response);
}

#[test]
fn ref_cast_with_non_null_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    ref.cast (ref any)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 3, 8));
    assert_json_snapshot!(response);
}

#[test]
fn ref_cast_with_abbr_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    ref.cast anyref))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 3, 8));
    assert_json_snapshot!(response);
}

#[test]
fn ref_cast_with_null_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    ref.cast (ref null any)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 3, 8));
    assert_json_snapshot!(response);
}

#[test]
fn seven_types_on_stack() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    i32.const 0
    i64.const 0
    f32.const 0
    f64.const 0
    i32.const 0
    i64.const 0
    f32.const 0
    drop))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 10, 7));
    assert_json_snapshot!(response);
}

#[test]
fn eight_types_on_stack() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    i32.const 0
    i64.const 0
    f32.const 0
    f64.const 0
    i32.const 0
    i64.const 0
    f32.const 0
    f64.const 0
    drop))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 11, 7));
    assert_json_snapshot!(response);
}

#[test]
fn nine_types_on_stack() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    i32.const 0
    i64.const 0
    f32.const 0
    f64.const 0
    i32.const 0
    i64.const 0
    f32.const 0
    f64.const 0
    i32.const 0
    drop))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 12, 7));
    assert_json_snapshot!(response);
}

#[test]
fn init_stack_func() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param i32) (result i64)
    drop))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 3, 7));
    assert_json_snapshot!(response);
}

#[test]
fn init_stack_block_block() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    block (param i32) (result i64)
      drop
    end))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 4, 9));
    assert_json_snapshot!(response);
}

#[test]
fn init_stack_block_loop() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    loop (param i32) (result i64)
      drop
    end))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 4, 9));
    assert_json_snapshot!(response);
}

#[test]
fn init_stack_block_try_table() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    try_table (param i32) (result i64)
      drop
    end))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 4, 9));
    assert_json_snapshot!(response);
}

#[test]
fn init_stack_block_if_then() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (if (param i32) (result i64)
      (f32.const 0)
      (then (drop))
      (else (drop))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 5, 15));
    assert_json_snapshot!(response);
}

#[test]
fn init_stack_block_if_else() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (if (param i32) (result i64)
      (f32.const 0)
      (then (drop))
      (else (drop))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 6, 15));
    assert_json_snapshot!(response);
}

#[test]
fn stack_polymorphic() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (drop (unreachable (i32.add (i32.const 1) (i32.const 2)))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 3, 7));
    assert_json_snapshot!(response);
}

#[test]
fn block() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    f32.const 0
    (block (param i64) (result f64)
      (i32.const 0 (f32.const 0)))
    (nop)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 6, 7));
    assert_json_snapshot!(response);
}

#[test]
fn block_if_cond() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (if (result f64)
      (i32.const 0 (f32.const 0)))
    (nop)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.hover(create_params(uri, 5, 7));
    assert_json_snapshot!(response);
}
