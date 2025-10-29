use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn throw() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (tag $e0)
  (tag $e-i32 (param i32))
  (tag $e-i32-i32 (param i32 i32))

  (func (param i32) (result i32)
    (local.get 0)
    (i32.const 0)
    (if
      (i32.ne)
      (then
        (throw $e0)))
    (i32.const 0))

  (func
    (throw $e0)
    (throw $e-i32))
  (func
    (block (result i32)
      (throw $e0))
    (throw $e-i32))

  (func $throw-1-2
    (i32.const 1)
    (i32.const 2)
    (throw $e-i32-i32))
  (func (export "test-throw-1-2")
    (block $h (result i32 i32)
      (try_table (catch $e-i32-i32 $h)
        (call $throw-1-2))
      (return))
    (if
      (i32.ne
        (i32.const 2))
      (then
        (unreachable)))
    (if
      (i32.ne
        (i32.const 1))
      (then
        (unreachable))))

  (func
    (throw 1))
  (func
    (i64.const 5)
    (throw 1)))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn throw_ref() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (tag $e0)
  (tag $e1)

  (func
    (throw_ref))
  (func
    (block
      (throw_ref)))

  (func (param i32) (result i32)
    (block $h (result exnref)
      (try_table (result i32) (catch_ref $e0 $h)
        (throw $e0))
      (return))
    (if (param exnref)
      (i32.eqz
        (local.get 0))
      (then
        (throw_ref))
      (else
        (drop)))
    (i32.const 23))

  (func (param i32) (result i32)
    (block $h (result exnref)
      (try_table (result i32) (catch_all_ref $h)
        (throw $e0))
      (return))
    (if (param exnref)
      (i32.eqz
        (local.get 0))
      (then
        (throw_ref))
      (else
        (drop)))
    (i32.const 23))

  (func (param i32) (result i32) (local $exn1 exnref) (local $exn2 exnref)
    (block $h1 (result exnref)
      (try_table (result i32) (catch_ref $e1 $h1)
        (throw $e1))
      (return))
    (local.set $exn1)
    (block $h2 (result exnref)
      (try_table (result i32) (catch_ref $e0 $h2)
        (throw $e0))
      (return))
    (local.set $exn2)
    (if
      (i32.eq
        (local.get 0)
        (i32.const 0))
      (then
        (throw_ref
          (local.get $exn1))))
    (if
      (i32.eq
        (local.get 0)
        (i32.const 1))
      (then
        (throw_ref
          (local.get $exn2))))
    (i32.const 23))

  (func (param i32) (result i32) (local $e exnref)
    (block $h1 (result exnref)
      (try_table (result i32) (catch_ref $e0 $h1)
        (throw $e0))
      (return))
    (local.set $e)
    (block $h2 (result exnref)
      (try_table (result i32) (catch_ref $e0 $h2)
        (if
          (i32.eqz
            (local.get 0))
          (then
            (throw_ref
              (local.get $e))))
        (i32.const 42))
      (return))
    (drop)
    (i32.const 23))

  (func (local $e exnref)
    (block $h (result exnref)
      (try_table (result f64) (catch_ref $e0 $h)
        (throw $e0))
      (unreachable))
    (local.set $e)
    (i32.const 1)
    (throw_ref
      (local.get $e))))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
