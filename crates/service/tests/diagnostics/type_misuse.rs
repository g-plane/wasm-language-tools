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
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
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
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
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
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
