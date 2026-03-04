use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn wasmfx_types() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $ft1 (func))
  (type $ct1 (cont $ft1))

  (type $ft2 (func (param i32) (result i32)))
  (type $ct2 (cont $ft2))

  (func $test
    (param $p1 (ref cont))
    (param $p2 (ref nocont))
    (param $p3 (ref $ct1))
    (param $p4 (ref $ct2))
    (param $p5 (ref null $ct1))

    (local $x1 (ref cont))
    (local $x2 (ref nocont))
    (local $x3 (ref $ct1))
    (local $x4 (ref $ct2))
    (local $x5 (ref null $ct1))

    ;; nocont <: cont
    (local.set $x1 (local.get $p2))

    ;; nocont <: $ct1
    (local.set $x3 (local.get $p2))

    ;; $ct1 <: $cont
    (local.set $x3 (local.get $p3))

    ;; (ref $ct1) <: (ref null $cont)
    (local.set $x5 (local.get $p3))

    ;; cont </: nocont
    (local.set $p2 (local.get $p1))

    ;; $ct1 </: nocont
    (local.set $p2 (local.get $p3))

    ;; $cont </: $ct1
    (local.set $p3 (local.get $p1))

    ;; (ref null $ct1) </: (ref $ct1)
    (local.set $x3 (local.get $p5))

    ;; (ref $ct1) </: (ref $ct2)
    (local.set $p4 (local.get $p3))
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
fn subtyping() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $f (func))

  (type $ft1 (sub (func (param (ref $f)) (result (ref func)))))
  (type $ct1 (sub (cont $ft1)))

  (type $ft2 (func (param (ref any)) (result (ref func))))
  (type $ct2 (cont $ft2))

  (type $ft3 (sub $ft1 (func (param (ref func)) (result (ref $f)))))
  (type $ct3 (cont $ft3))

  (type $ft4 (func (param (ref func)) (result (ref $f))))
  (type $ct4 (cont $ft4))

  (type $ct_sub (sub $ct1 (cont $ft3)))
  (func (param $p1 (ref $ct1)) (param $p2 (ref $ct_sub))
    ;; ok: (ref $ct_sub) <: (ref $ct1)
    (local.set $p1
      (local.get $p2)))

  (func (param $p1 (ref $ct1)) (param $p2 (ref $ct4))
    ;; Error $ct4 and $ct1 have generally compatible types,
    ;; but have not declared $ft4 <: ft1 or $ct4 <: $ct1
    ;; Thus, $ct4 </: $ct1.
    (local.set $p1
      (local.get $p2)))

  (func (param $p1 (ref $ct1))
    ;;(param $p2 (ref $ct2))
    (param $p3 (ref $ct3))
    ;; Error $ct3 and $ct1 have generally compatible types,
    ;; (in particular: declared $ft3 <: ft1,
    ;; but have not declared $ct3 <: $ct1
    ;; $ct3 </: $ct1
    (local.set $p1
      (local.get $p3)))

  (func (param $p_any (ref any)) (param $p_cont (ref cont))
    ;; Error: cont </: any
    (local.set $p_any
      (local.get $p_cont))))
";
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

  (type $ft2 (func (param i32 i32) (result i32)))

  ;; simple smoke test
  (func (param $x (ref $ft1)) (result (ref $ct1))
    (local.get $x)
    (cont.new $ct1)
  )

  ;; cont.new takes a nullable function
  (func (param $x (ref null $ft1)) (result (ref $ct1))
    (local.get $x)
    (cont.new $ct1)
  )

  (func (param $x (ref null $ct1)) (result (ref $ct1))
    (local.get $x)
    (cont.new $ct1)
  )

  (func (param $x (ref null $ft2)) (result (ref $ct1))
    (local.get $x)
    (cont.new $ct1)
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

  (func
    (param $p_ct1 (ref null $ct1))
    (result (ref $ct0))

    ;; cont.bind takes nullable ref, returns non-nullable one
    (i32.const 123)
    (local.get $p_ct1)
    (cont.bind $ct1 $ct0)
  )

  (func
    (param $p_ct2 (ref $ct2))
    (result (ref $ct1))

    ;; cont.bind does not have to apply continuation fully
    (i64.const 123)
    (local.get $p_ct2)
    (cont.bind $ct2 $ct1)
  )

  (func
    (param $p_ct1 (ref $ct1))
    (result (ref $ct1))

    ;; cont.bind can take same type twice, not actually apply anything
    (local.get $p_ct1)
    (cont.bind $ct1 $ct1)
  )

  (func
    (param $p_ct1 (ref $ct1))
    ;; cont.bind type mismatch: passing wrong kind of continuation.
    ;; This is actually just checking type-matching, not cont.bind specific.
    (local.get $p_ct1)
    (cont.bind $ct0 $ct0)
    (drop)
  )

  (func
    (param $p_ct1 (ref $ct1))
    ;; error: Insufficient arguments::
    ;; This is really testing the general application of arguments to instructions,
    ;; but trying to trick parsers of the folded form
    (cont.bind $ct1 $ct0 (local.get $p_ct1))
    (drop)
  )

  (func
    (param $p_ct1 (ref $ct1))
    ;; error: Too many arguments::
    ;; This is really testing the general application of arguments to instructions,
    ;; but trying to trick parsers of the folded form
    (cont.bind $ct1 $ct0 (i32.const 123) (i32.const 123) (local.get $p_ct1))
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

  (type $ft5 (func (param i64)))
  (type $ct5 (cont $ft5))

  (tag $t0)
  (tag $t1)
  (tag $t2 (param i32) (result i64))
  (tag $t3 (param i64) (result i32))
  (tag $t4 (param i32))

  ;; resume takes a nullable reference
  (func (param $x (ref null $ct0))
    (local.get $x)
    (resume $ct0)
  )

  ;; handler blocks take the continuation as nullable ref
  (func (param $x (ref $ct0))
    (block $handler (result (ref null $ct0))
      (local.get $x)
      (resume $ct0 (on $t0 $handler))
      (return)
    )
    (unreachable)
  )

  ;; handler block can have params
  (func (param $x (ref $ct0))
    (local.get $x)
    (block $handler (param (ref $ct0)) (result (ref $ct0))
      (resume $ct0 (on $t0 $handler))
      (return)
    )
    (unreachable)
  )

  (func
    (param $p (ref $ct4))
    ;; error: Too many arguments::
    ;; This is really testing the general application of arguments to instructions,
    ;; but trying to trick parsers of the folded form
    (resume $ct4 (i32.const 123) (i32.const 123) (local.get $p))
  )

  (func
    (param $p (ref $ct4))
    ;; error: Too few arguments::
    ;; This is really testing the general application of arguments to instructions,
    ;; but trying to trick parsers of the folded form
    (resume $ct4 (local.get $p))
  )

  (func
    (param $p (ref $ct5))
    ;; error: Mismatch between annotation on resume and actual argument
    ;; This is really testing the general application of arguments to instructions.
    (resume $ct4 (i32.const 123) (local.get $p))
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
fn suspend() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (tag $t1 (param i64 i32) (result i32 i64))
  (tag $t2 (param i32) (result i32 i64))

  (func (result i32 i64)
    (i64.const 123)
    (i32.const 123)
    (suspend $t1)
  )

  (func (result i32 i64)
    ;; error: Insufficient arguments::
    ;; This is really testing the general application of arguments to instructions,
    ;; but trying to trick parsers of the folded form
    (suspend $t1 (i64.const 123))
  )

  (func (result i32 i64)
    ;; error: Too many arguments:
    ;; This is really testing the general application of arguments to instructions,
    ;; but trying to trick parsers of the folded form
    (suspend $t2 (i64.const 123) (i32.const 123))
  )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
