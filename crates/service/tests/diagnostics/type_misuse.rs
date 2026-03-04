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
fn cont_bind_subtyping1() {
    let uri = "untitled:test".to_string();
    let source = "
(module $non_final
  (type $ft1 (func (param i32) (result (ref func))))
  (type $ct1 (sub (cont $ft1)))

  (type $ft0 (func (result (ref func))))
  (type $ct0 (sub (cont $ft0)))

  (func (param $x (ref $ct1))
    (i32.const 123)
    (local.get $x)
    ;; Smoke test: using non-final types here
    (cont.bind $ct1 $ct0)
    (drop)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn cont_bind_subtyping2() {
    let uri = "untitled:test".to_string();
    let source = "
(module $subtyping
  (type $f (func))

  (type $ft0_sup (func (result (ref func))))
  (type $ct0_sup (cont $ft0_sup))

  (type $ft1 (func (param i32) (result (ref $f))))
  (type $ct1 (cont $ft1))

  (func (param $x (ref $ct1))
    (i32.const 123)
    (local.get $x)
    ;; Okay: The most natural second continuation type would be $ct0.
    ;; But we have (func (result (ref $f))) <: (func (result (ref $func)))
    ;; This holds without us needing to declare any subtyping relations at all.
    (cont.bind $ct1 $ct0_sup)
    (drop)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn cont_bind_recursive1() {
    let uri = "untitled:test".to_string();
    let source = "
(module $recursive
  (rec
    (type $ft0 (func (result (ref $ct_rec))))
    (type $ft1 (func (param i32) (result (ref $ct_rec))))
    (type $ct_rec (cont $ft1)))
  (type $ct0 (cont $ft0))

  (rec
    (type $ft0' (func (result (ref $ct_rec'))))
    (type $ft1' (func (param i32) (result (ref $ct_rec'))))
    (type $ct_rec' (cont $ft1')))
  (type $ct1 (cont $ft1'))

  ;; Okay: Some simple test where the types involved in cont.bind
  ;; are part of a recursion group
  ;; (more concretely: two equivalent recursion groups)
  (func (param $x (ref $ct1))
    (i32.const 123)
    (local.get $x)
    (cont.bind $ct1 $ct0)
    (drop)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn cont_bind_recursive2() {
    let uri = "untitled:test".to_string();
    let source = "
(module $recursive_subtyping
  ;; We define types such that $ft0 <: $ft0_sup and $ct_rec <: $ct_rec_sup
  (rec
    (type $ft0_sup (sub (func (result (ref $ct_rec_sup)))))
    (type $ft0 (sub $ft0_sup (func (result (ref $ct_rec)))))
    (type $ft1 (sub (func (param i32) (result (ref $ct_rec)))))

    (type $ct_rec_sup (sub (cont $ft0_sup)))
    (type $ct_rec (sub $ct_rec_sup (cont $ft0))))

  (type $ct0_sup (cont $ft0_sup))
  (type $ct0 (cont $ft0))
  (type $ct1 (cont $ft1))

  (func (param $x (ref $ct1))
    (i32.const 123)
    (local.get $x)
    (cont.bind $ct1 $ct0)
    (drop)

    (i32.const 123)
    (local.get $x)
    ;; Okay: We have (func (result (ref $ct_rec))) <: (func (result (ref $ct_rec_sup)))
    (cont.bind $ct1 $ct0_sup)
    (drop)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
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

#[test]
fn resume_subtyping1() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $ft0 (func))
  (type $ct0 (cont $ft0))

  (type $ft_sup (func (param (ref $ft0))))
  (type $ct_sup (cont $ft_sup))

  (type $ft_sub (func (param (ref func))))
  (type $ct_sub (cont $ft_sub)) ;; unused

  (tag $t (result (ref func)))

  (func (param $p (ref $ct0))
    ;; Here we test subtyping with respect to the continuations received by handlers.
    ;;
    ;; The most 'straightforward' type of the continuation in $handler would be (ref $ct_sub).
    ;; Instead, we use $ct_sup. According to the spec, we neither need
    ;; to declare $ft_sub <: $ft_sup or $ct_sub <: $ct_sup for this to work. We
    ;; have (func (param (ref func))) <: (func (param (ref $ft0))), which is
    ;; sufficient
    (block $handler (result (ref $ct_sup))
      (local.get $p)
      (resume $ct0 (on $t $handler))
      (return))
    (unreachable)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn resume_subtyping2() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $ft0 (func))
  (type $ct0 (cont $ft0))

  (tag $t (param (ref $ft0)))

  (func (param $p (ref $ct0))
    ;; Here we test subtyping with respect to the payloads received by handlers.
    ;; Instead of just (ref $ft0), then handler takes func.
    (block $handler (result (ref func) (ref $ct0))
      (local.get $p)
      (resume $ct0 (on $t $handler))
      (return))
    (unreachable)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn resume_subtyping3() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $ft0 (func))
  (type $ct0 (cont $ft0))

  (type $ft_sup (sub (func (param (ref $ft0)))))
  (type $ct_sup (sub (cont $ft_sup)))

  (type $ft_sub (sub $ft_sup (func (param (ref func)))))
  (type $ct_sub (cont $ft_sub))

  (tag $t (param (ref $ct_sub)))

  (func (param $p (ref $ct0))

    ;; This is similar above, but this time we use a continuation as payload.
    ;; But we did not actually declare $ct_sub <: $ct_sub.
    ;;
    ;; This is mostly just to check the following:
    ;; For the continuation received by every handler, we see through the
    ;; continuation type and do structural subtyping on the underlying
    ;; function type.
    ;; However, for continuations that are just payloads ($ct_sup here), we do
    ;; ordinary nominal subtyping.
    (block $handler (result (ref $ct_sup) (ref $ct0))
      (local.get $p)
      (resume $ct0 (on $t $handler))
      (return))
    (unreachable)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn cast_to_cont() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $f (func))
  (type $c (cont $f))

  (func (drop (ref.test contref (unreachable))))
  (func (drop (ref.test nullcontref (unreachable))))
  (func (drop (ref.test (ref $c) (unreachable))))

  (func (drop (ref.cast contref (unreachable))))
  (func (drop (ref.cast nullcontref (unreachable))))
  (func (drop (ref.cast (ref $c) (unreachable))))

  (func
    (block (result contref) (br_on_cast 0 contref contref (unreachable)))
    (drop))
  (func
    (block (result contref) (br_on_cast 0 nullcontref nullcontref (unreachable)))
    (drop))
  (func
    (block (result contref) (br_on_cast 0 (ref $c) (ref $c) (unreachable)))
    (drop))

  (func
    (block (result contref) (br_on_cast_fail 0 contref contref (unreachable)))
    (drop))
  (func
    (block (result contref) (br_on_cast_fail 0 nullcontref nullcontref (unreachable)))
    (drop))
  (func
    (block (result contref) (br_on_cast_fail 0 (ref $c) (ref $c) (unreachable)))
    (drop)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
