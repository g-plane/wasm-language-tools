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

#[test]
fn cont_new() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $ft1 (func (param i32) (result i32)))
  (type $ct1 (cont $ft1))

  (func (param $x (ref null $ct1))
    (local.get $x)
    (cont.new $ft1)
    (drop)
  )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn cont_bind() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $ft0 (func (result i32)))
  (type $ct0 (cont $ft0))

  (type $ft1 (func (param i32) (result i32)))
  (type $ct1 (cont $ft1))

  (type $ft2 (func (param i64 i32) (result i32)))
  (type $ct2 (cont $ft2))

  (type $ft1_alt (func (param i64) (result i32)))
  (type $ct1_alt (cont $ft1_alt))

  (type $ft1_alt2 (func (param i32) (result i64)))
  (type $ct1_alt2 (cont $ft1_alt2))

  (func
    (param $p_ft0 (ref $ft0))
    ;; error: non-continuation type on cont.bind
    (local.get $p_ft0)
    (cont.bind $ft0 $ft0)
    (drop)
  )

  (func
    (param $p_ct2 (ref $ct2))
    ;; error: two continuation types not agreeing on arg types
    (i64.const 123)
    (local.get $p_ct2)
    (cont.bind $ct2 $ct1_alt)
    (drop)
  )

  (func
    (param $p_ct2 (ref $ct2))
    ;; error: two continuation types not agreeing on return types
    (i64.const 123)
    (local.get $p_ct2)
    (cont.bind $ct2 $ct1_alt2)
    (drop)
  )

  (func
    (param $p_ct0 (ref $ct0))
    ;; error: trying to go from 0 to 1 args
    (local.get $p_ct0)
    (cont.bind $ct0 $ct1)
    (drop)
  )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn resume() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $ft0 (func))
  (type $ct0 (cont $ft0))

  (type $ft1 (func (param f32) (result f64)))
  (type $ct1 (cont $ft1))

  (type $ft2 (func (param i64) (result f64)))
  (type $ct2 (cont $ft2))

  (type $ft3 (func (param i32) (result f64)))
  (type $ct3 (cont $ft3))

  (type $ft4 (func (param i32)))
  (type $ct4 (cont $ft4))

  (tag $t0)
  (tag $t1)
  (tag $t2 (param i32) (result i64))
  (tag $t3 (param i64) (result i32))
  (tag $t4 (param i32))

  ;; Multiple tags, all types handled correctly
  (func (param $x (ref $ct1)) (result f64)
    (block $handler0 (result i32 (ref $ct2))
      (block $handler1 (result i64 (ref $ct3))
        (f32.const 1.23)
        (local.get $x)
        (resume $ct1 (on $t2 $handler0) (on $t3 $handler1))
        (return)
      )
      (unreachable)
    )
    (unreachable)
  )

  ;; Same as above, but we provide two handlers for the same tag
  (func (param $x (ref $ct1)) (result f64)
    (block $handler0 (result i32 (ref $ct2))
      (block $handler1 (result i32 (ref $ct2))
        (f32.const 1.23)
        (local.get $x)
        (resume $ct1 (on $t2 $handler0) (on $t2 $handler1))
        (return)
      )
      (unreachable)
    )
    (unreachable)
  )

  ;; Nothing wrong with using the same handler block for multiple tags
  (func (param $x (ref $ct0))
    (block $handler (result (ref null $ct0))
      (local.get $x)
      (resume $ct0 (on $t0 $handler) (on $t1 $handler))
      (return)
    )
    (unreachable)
  )

  (func (param $x (ref $ct0))
    (local.get $x)
    (resume $ft0)
  )

  (func (param $x (ref $ct0))
    (block $handler (result (ref $ft0))
      (local.get $x)
      (resume $ct0 (on $t0 $handler))
      (return)
    )
    (unreachable)
  )

  (func
    (param $p (ref $ct0))
    (block $handler (result (ref $ct0))
      ;; error: handler block has insufficient number of results
      (local.get $p)
      (resume $ct0 (on $t4 $handler))
      (return)
    )
    (unreachable)
  )

  (func
    (param $p (ref $ct0))
    (block $handler (result i32 i32 (ref $ct0))
      ;; error: handler block has too many results
      (local.get $p)
      (resume $ct0 (on $t4 $handler))
      (return)
    )
    (unreachable)
  )

  (func
    (param $p (ref $ct0))
    (block $handler (result i64 (ref $ct0))
      ;; error: type mismatch in non-continuation handler result type
      (local.get $p)
      (resume $ct0 (on $t4 $handler))
      (return)
    )
    (unreachable)
  )

  (func
    (param $p (ref $ct0))
    (block $handler (result i32 (ref $ct4))
      ;; error: type mismatch in continuation handler result type
      (local.get $p)
      (resume $ct0 (on $t4 $handler))
      (return)
    )
    (unreachable)
  )

  (func (param $p (ref $ct0))
    (local.get $p)
    (resume $ct0 (on $t4 switch))
  )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
