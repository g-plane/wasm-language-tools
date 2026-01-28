use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn throw() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (import "" "" (tag $e-i32-i32 (param i32 i32)))
  (tag $e0)
  (tag $e-i32 (param i32))

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
    (throw 2))
  (func
    (i64.const 5)
    (throw 2)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
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
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn try_table() {
    let uri = "untitled:test".to_string();
    let source = r"
(module
  (tag $e0)
  (tag $e1)
  (tag $e2)
  (tag $e-f32 (param f32))

  (func (param i32) (result i32)
    (block $h1
      (try_table (result i32) (catch $e1 $h1)
        (block $h0
          (try_table (result i32) (catch $e0 $h0)
            (if
              (i32.eqz
                (local.get 0))
              (then
                (throw $e0))
              (else
                (if
                  (i32.eq
                    (local.get 0)
                    (i32.const 1))
                  (then
                    (throw $e1))
                  (else
                    (throw $e2)))))
            (i32.const 2))
          (br 1))
        (i32.const 3))
      (return))
    (i32.const 4))

  (func (param f32) (result f32)
    (block $h (result f32)
      (try_table (result f32) (catch $e-f32 $h)
        (throw $e-f32
          (local.get 0))
        (f32.const 0))
      (return))
    (return))

  (func (param f32) (result f32)
    (block $h (result f32 exnref)
      (try_table (result f32) (catch_ref $e-f32 $h)
        (throw $e-f32
          (local.get 0))
        (f32.const 0))
      (return))
    (drop)
    (return))

  (func $throw-if (param i32) (result i32)
    (local.get 0)
    (i32.const 0)
    (if
      (i32.ne)
      (then
        (throw $e0)))
    (i32.const 0))
  (func (param i32) (result i32)
    (block $h
      (try_table (result i32) (catch $e0 $h)
        (try_table (result i32)
          (call $throw-if
            (local.get 0))))
      (return))
    (i32.const 1))

  (func
    (i32.const 0)
    (try_table (param i32)
      (drop))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
