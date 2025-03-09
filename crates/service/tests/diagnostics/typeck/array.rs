use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn new() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $vec (array f32))
  (global (ref $vec)
    (array.new $vec
      (f32.const 1)
      (i32.const 3)))
  (global (ref $vec)
    (array.new $vec
      (i32.const 1)
      (f32.const 3))))
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
  (type $vec (array f32))
  (global (ref $vec)
    (array.new_default $vec
      (i32.const 3))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn new_fixed() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $vec (array f32))
  (global (ref $vec)
    (array.new_fixed $vec 1
      (f32.const 1)
      (f32.const 2)))
  (global (ref $vec)
    (array.new_fixed $vec 2
      (f32.const 1)
      (f32.const 2)))
  (global (ref $vec)
    (array.new_fixed $vec 3
      (f32.const 1)
      (f32.const 2))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn new_data() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $vec (array f32))
  (func (result (ref $vec))
    (array.new_data $vec $d
      (i32.const 1)
      (i32.const 3))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn new_elem() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $vec (array f32))
  (func (result (ref $vec))
    (array.new_elem $vec $d
      (i32.const 1)
      (i32.const 3))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn get() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $vec (array f32))
  (func $get (param $i i32) (param $v (ref $vec)) (result f32)
    (array.get $vec
      (local.get $v)
      (local.get $i)))
  (func (param $i i32) (result f32)
    (call $get
      (local.get $i)
      (array.new_default $vec
        (i32.const 3)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn get_s() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $vec (array i8))
  (func (param $i i32) (param $v (ref $vec)) (result i32)
    (array.get_s $vec
      (local.get $v)
      (local.get $i))))
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
  (type $vec (array i8))
  (func (param $i i32) (param $v (ref $vec)) (result i32)
    (array.get_u $vec
      (local.get $v)
      (local.get $i))))
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
  (type $mvec (array (mut f32)))
  (func $set (param $i i32) (param $v (ref $mvec)) (param $y f32)
    (array.set $mvec
      (local.get $v)
      (local.get $i)
      (local.get $y)))
  (func (param $i i32) (param $y f32)
    (call $set
      (local.get $i)
      (array.new_default $mvec
        (i32.const 3))
      (local.get $y))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn fill() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr8_mut (array (mut i8)))
  (func (param $1 (ref $arr8_mut)) (param $2 funcref)
    (array.fill $arr8_mut
      (local.get $1)
      (i32.const 0)
      (local.get $2)
      (i32.const 0)))

  (type $b (array (mut funcref)))
  (func (param $1 (ref $b)) (param $2 i32)
    (array.fill $b
      (local.get $1)
      (i32.const 0)
      (local.get $2)
      (i32.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn copy() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $arr (array f32))
  (type $arr_mut (array (mut f32)))
  (global (ref $arr)
    (array.new $arr
      (f32.const 10)
      (i32.const 12)))
  (global (mut (ref $arr_mut))
    (array.new_default $arr_mut
      (i32.const 12)))
  (func (param $1 i32) (param $2 i32) (param $3 i32)
    (array.copy $arr_mut $arr
      (global.get 1)
      (local.get $1)
      (global.get 0)
      (local.get $2)
      (local.get $3))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
