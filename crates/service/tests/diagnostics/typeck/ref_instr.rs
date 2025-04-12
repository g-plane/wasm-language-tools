use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn null() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct))
  (func (result (ref struct))
    ref.null struct)
  (func (result (ref 0))
    ref.null 0)
  (func (result (ref $s))
    ref.null $s))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn is_null() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (func))
  (func (param $x funcref) (result i32)
    (ref.is_null
      (local.get $x)))
  (func (param $x externref) (result i32)
    (ref.is_null
      (local.get $x)))
  (func (param $x (ref null $t)) (result i32)
    (ref.is_null
      (local.get $x)))
  (func
    (ref.is_null))
  (func (param i32)
    (ref.is_null
      (local.get 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn as_non_null() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct))
  (func (result (ref 0))
    ref.as_non_null)
  (func (param (ref null $s)) (result (ref $s))
    local.get 0
    ref.as_non_null))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn test() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param (ref null any)) (result i32)
    local.get 0
    ref.test funcref)
  (func (param anyref) (result i32)
    local.get 0
    ref.test (ref null func))
  (func (param (ref null func))
    local.get 0
    ref.test funcref)
  (func (param (ref struct)) (result i32)
    local.get 0
    ref.test anyref)
  (func (param (ref struct)) (result i32)
    local.get 0
    ref.test (ref null any))
  (func (param (ref null struct)) (result i32)
    local.get 0
    ref.test (ref any))
  (func (param anyref) (result i32)
    local.get 0
    ref.test (ref struct))
  (func (param nullfuncref) (result i32)
    local.get 0
    ref.test funcref)
  (func (param funcref) (result i32)
    local.get 0
    ref.test (ref nofunc)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn cast() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param (ref null any)) (result (ref null func))
    local.get 0
    ref.cast funcref)
  (func (param anyref) (result funcref)
    local.get 0
    ref.cast (ref null func))
  (func (param (ref null func))
    local.get 0
    ref.cast funcref)
  (func (param (ref struct)) (result (ref any))
    local.get 0
    ref.cast anyref)
  (func (param (ref struct)) (result (ref null any))
    local.get 0
    ref.cast (ref any))
  (func (param (ref struct)) (result anyref)
    local.get 0
    ref.cast (ref null any))
  (func (param (ref null struct)) (result (ref any))
    local.get 0
    ref.cast (ref any))
  (func (param anyref) (result (ref struct))
    local.get 0
    ref.cast (ref struct))
  (func (param nullfuncref) (result funcref)
    local.get 0
    ref.cast funcref)
  (func (param funcref) (result (ref nofunc))
    local.get 0
    ref.cast (ref nofunc)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn func() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s0 (func (param i32)))
  (type $s1 (func (param i32 (ref $s0))))
  (type $s2 (func (param i32 (ref $s0))))
  (type $t1 (func (param (ref $s1))))
  (type $t2 (func (param (ref $s2))))

  (type $t3 (func))

  (func $s1 (type $s1))
  (func $s2 (type $s2))
  (func $f1 (type $t1))
  (func $f2 (type $t2))
  (table funcref (elem $f1 $f2 $s1 $s2))

  (func
    (call_indirect (type $t1) (ref.func $s1) (i32.const 0))
    (call_indirect (type $t1) (ref.func $s1) (i32.const 1))
    (call_indirect (type $t1) (ref.func $s2) (i32.const 0))
    (call_indirect (type $t1) (ref.func $s2) (i32.const 1))
    (call_indirect (type $t2) (ref.func $s1) (i32.const 0))
    (call_indirect (type $t2) (ref.func $s1) (i32.const 1))
    (call_indirect (type $t2) (ref.func $s2) (i32.const 0))
    (call_indirect (type $t2) (ref.func $s2) (i32.const 1)))

  (type $t (func (param i32) (result i32)))
  (elem func $f)
  (func $f (param i32) (result i32) (i32.mul (local.get 0) (local.get 0)))
  (func $a (param $n i32) (param $r (ref null $t)) (result i32)
    (block $l (result i32)
      (return (call_ref $t (br_on_null $l (local.get $n) (local.get $r))))))
  (func (param $n i32) (result i32)
    (call $a (local.get $n) (ref.func $f))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
