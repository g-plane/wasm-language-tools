use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn valid() {
    let uri = "untitled:test".to_string();
    let source = r"
(module
  (type $t (func))
  (func $dummy)
  (elem declare func $dummy)

  (tag $e (param (ref $t)))
  (func $throw
    (throw $e
      (ref.func $dummy)))

  (func (result (ref null $t))
    (block $l (result (ref null $t))
      (try_table (catch $e $l)
        (call $throw))
      (unreachable)))

  (func (result (ref null $t))
    (block $l (result (ref null $t) (ref exn))
      (try_table (catch_ref $e $l)
        (call $throw))
      (unreachable))
    (drop))

  (func (result (ref null $t))
    (block $l (result (ref null $t) (ref null exn))
      (try_table (catch_ref $e $l)
        (call $throw))
      (unreachable))
    (drop))

  (func
    (block $l (result (ref exn))
      (try_table (catch_all_ref $l)
        (call $throw))
      (unreachable))
    (drop))

  (func
    (block $l (result (ref null exn))
      (try_table (catch_all_ref $l)
        (call $throw))
      (unreachable))
    (drop)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn invalid() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (tag)
  (tag (param i64))
  (func
    (try_table (catch_ref 0 0)))
  (func (result exnref)
    (try_table (catch 0 0))
    (unreachable))
  (func
    (try_table (catch_all_ref 0)))
  (func (result exnref)
    (try_table (catch_all 0))
    (unreachable))
  (func (result i32 exnref)
    (try_table (result i32) (catch_ref 1 0)
      (i32.const 42)))

  (type $t (func))
  (tag $e (param (ref null $t)))
  (func (result (ref $t))
    (block $l (result (ref $t))
      (try_table (catch $e $l))
      (unreachable)))
  (func (result (ref $t))
    (block $l (result (ref $t) (ref exn))
      (try_table (catch_ref $e $l))
      (unreachable))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
