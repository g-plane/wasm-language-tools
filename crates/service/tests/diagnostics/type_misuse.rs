use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn structs() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $func (func))
  (type $struct (struct (field (mut i32))))
  (type $array (array i32))
  (func (param (ref $struct))
    local.get 0
    local.get 0
    struct.get $struct 0
    struct.set $struct 0)
  (func (param (ref $struct))
    local.get 0
    local.get 0
    struct.get $func 0
    struct.set $array 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn array() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $func (func))
  (type $struct (struct (field i32)))
  (type $array (array (mut i32)))
  (func (param (ref $array))
    local.get 0
    local.get 0
    i32.const 0
    array.get $array
    i32.const 0
    array.set $array)
  (func (param (ref $array))
    local.get 0
    local.get 0
    i32.const 0
    array.get $func
    i32.const 0
    array.set $struct))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn array_copy() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $func (func))
  (type $struct (struct))
  (type $dst_array (array (mut i32)))
  (type $src_array (array i64))
  (func (param (ref $dst_array) (ref $src_array))
    local.get 0
    i32.const 0
    local.get 1
    i32.const 0
    i32.const 0
    array.copy $struct $struct)
  (func (param (ref $dst_array) (ref $src_array))
    local.get 0
    i32.const 0
    local.get 1
    i32.const 0
    i32.const 0
    array.copy $func $func)
  (func (param (ref $dst_array) (ref $src_array))
    local.get 0
    i32.const 0
    local.get 1
    i32.const 0
    i32.const 0
    array.copy $dst_array $src_array))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn func() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $func (func))
  (type $struct (struct (field i32)))
  (type $array (array (mut i32)))
  (func (param (ref $func))
    local.get 0
    call_ref $func
    local.get 0
    return_call_ref $func)
  (func (param (ref $func))
    local.get 0
    call_ref $struct
    local.get 0
    return_call_ref $array))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn br_on_cast() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (struct))
  (func (param (ref any)) (result (ref $t))
    (block (result (ref any))
      (br_on_cast 1 (ref null any) (ref null $t)
        (local.get 0)))
    (unreachable))
  (func (param (ref any)) (result (ref null $t))
    (block (result (ref any))
      (br_on_cast 1 (ref any) (ref null $t)
        (local.get 0)))
    (unreachable))

  (func (result anyref)
    (br_on_cast 0 eqref anyref
      (unreachable)))
  (func (result anyref)
    (br_on_cast 0 structref arrayref
      (unreachable)))

  (func (param (ref any)) (result (ref $t))
    (block (result (ref any))
      (br_on_cast 1 (ref any) (ref $t)
        (local.get 0)))
    (unreachable))
  (func (param (ref null any)) (result (ref $t))
    (block (result (ref null any))
      (br_on_cast 1 (ref null any) (ref $t)
        (local.get 0)))
    (unreachable))
  (func (param (ref null any)) (result (ref null $t))
    (block (result (ref null any))
      (br_on_cast 1 (ref null any) (ref null $t)
        (local.get 0)))
    (unreachable)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn br_on_cast_fail() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (struct))
  (func (param (ref any)) (result (ref any))
    (block (result (ref null $t))
      (br_on_cast_fail 1 (ref any) (ref null $t)
        (local.get 0)))
    (ref.as_non_null))
  (func (param (ref null any)) (result (ref any))
    (block (result (ref $t))
      (br_on_cast_fail 1 (ref null any) (ref $t)
        (local.get 0))))

  (func (result anyref)
    (br_on_cast_fail 0 eqref anyref
      (unreachable)))
  (func (result anyref)
    (br_on_cast_fail 0 structref arrayref
      (unreachable)))

  (func (param (ref any)) (result (ref any))
    (block (result (ref $t))
      (br_on_cast_fail 1 (ref any) (ref $t)
        (local.get 0))))
  (func (param (ref null any)) (result (ref null any))
    (block (result (ref $t))
      (br_on_cast_fail 1 (ref null any) (ref $t)
        (local.get 0))))
  (func (param (ref null any)) (result (ref null any))
    (block (result (ref null $t))
      (br_on_cast_fail 1 (ref null any) (ref null $t)
        (local.get 0)))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn table_ref_type() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func)
  (table 10 (ref func)
    ref.func 0)
  (func
    i32.const 0
    call_indirect 0)
  (func
    i32.const 0
    return_call_indirect 0)
  (table (import "" "") 10 anyref)
  (func
    i32.const 0
    call_indirect 1)
  (func
    i32.const 0
    return_call_indirect 1))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn return_call_result_type() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func)
  (func (result i32) (return_call 0) (i32.const 0))

  (type (func (result i64)))
  (func (result i32) (return_call_ref 0 (ref.func 3)) (i32.const 0))
  (func (result i64) (i64.const 1))

  (table 0 funcref)
  (func (result i32) (return_call_indirect 0 (result i64) (i32.const 0))))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn throw_with_results() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (tag (result i32))
  (func (result i32) throw 0))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
