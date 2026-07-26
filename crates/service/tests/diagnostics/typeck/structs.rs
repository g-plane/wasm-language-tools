use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn new() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct (field i32 f32)))
  (func (result (ref $s))
    f32.const 0
    i32.const 0
    struct.new 0)
  (func (result (ref $s))
    i32.const 0
    f32.const 0
    struct.new 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn new_default() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct (field i32 f32)))
  (func (result arrayref)
    struct.new_default $s)
  (func (result structref)
    struct.new_default $s))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn get() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field $x i32)))
  (type $t1 (struct (field i32) (field $x f32)))
  (type $t2 (struct (field i32 i32) (field $x (mut i64))))

  (func (param (ref 0)) (result i32)
    (struct.get 0 $x
      (local.get 0)))
  (func (param (ref $t1)) (result f32)
    (struct.get 1 $x
      (local.get 0)))
  (func (param (ref $t2)) (result i64)
    (struct.get $t2 $x
      (local.get 0)))

  (func (param (ref 0)) (result i64)
    (struct.get 0 $x
      (local.get 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn get_s() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field $x i8)))
  (func (param (ref 0)) (result i32)
    (struct.get_s 0 $x
      (local.get 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn get_u() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field $x i8)))
  (func (param (ref 0)) (result i32)
    (struct.get_u 0 $x
      (local.get 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn set() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $vec (struct (field f32) (field $y (mut f32)) (field $z f32)))
  (func (param $v (ref $vec)) (param $y f32)
    (struct.set $vec $y
      (local.get $v)
      (local.get $y)))

  (type $i8 (struct (field (mut i8))))
  (func (param (ref $i8))
    local.get 0
    i32.const 0
    struct.set $i8 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
