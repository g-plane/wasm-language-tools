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
