use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn global_get() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global i32
    i32.const 0)
  (func (result i32)
    (global.get 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn undef_global() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (global.set 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn mutable_global() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global (mut i32)
    i32.const 0)
  (func
    (global.set 0
      (i32.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn immutable_global() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global i32
    i32.const 0)
  (func
    (global.set 0
      (i32.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn imported_immutable_global() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (import "" "" (global $global i32))
  (func
    i32.const 0
    global.set $global))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn array_get() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (array i32))
  (func (param (ref 0)) (result i32)
    local.get 0
    i32.const 0
    array.get 0)

  (type $packed (array i8))
  (func (param (ref $packed)) (result i32 i32)
    local.get 0
    i32.const 0
    array.get_s $packed
    local.get 0
    i32.const 0
    array.get_u $packed))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn undef_array() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param (ref 0)) (result i32)
    local.get 0
    i32.const 0
    array.get 0)

  (func (param (ref $packed)) (result i32 i32)
    local.get 0
    i32.const 0
    array.get_s $packed
    local.get 0
    i32.const 0
    array.get_u $packed))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn mutable_array() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (array (mut i32)))
  (func (param (ref 0))
    local.get 0
    i32.const 0
    i32.const 0
    array.set 0)

  (type $packed (array (mut i8)))
  (func (param (ref $packed))
    local.get 0
    i32.const 0
    i32.const 0
    array.set $packed))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn immutable_array() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (array i32))
  (func (param (ref 0))
    local.get 0
    i32.const 0
    i32.const 0
    array.set 0)

  (type $packed (array i8))
  (func (param (ref $packed))
    local.get 0
    i32.const 0
    i32.const 0
    array.set $packed))
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
  (type $dst (array i32))
  (type $src (array i32))
  (func (param (ref $dst) (ref $src))
    local.get 0
    i32.const 0
    local.get 1
    i32.const 0
    i32.const 0
    array.copy $dst $src))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn struct_get() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field i32)))
  (func (param (ref 0)) (result i32)
    local.get 0
    struct.get 0 0)

  (type $packed_s (struct (field $packed_field i8)))
  (func (param (ref $packed_s)) (result i32 i32)
    local.get 0
    struct.get_s $packed_s $packed_field
    local.get 0
    struct.get_u $packed_s $packed_field))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn undef_struct_field() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct))
  (func (param (ref 0))
    local.get 0
    struct.get 0 0)

  (type $packed_s (struct))
  (func (param (ref $packed_s)) (result i32 i32)
    local.get 0
    struct.get_s $packed_s $packed_field
    local.get 0
    struct.get_u $packed_s $packed_field))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn mutable_struct_field() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field (mut i32))))
  (func (param (ref 0))
    local.get 0
    i32.const 0
    struct.set 0 0)

  (type $packed_s (struct (field $packed_field (mut i8))))
  (func (param (ref $packed_s))
    local.get 0
    i32.const 0
    struct.set $packed_s $packed_field))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn immutable_struct_field() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field i32)))
  (func (param (ref 0))
    local.get 0
    i32.const 0
    struct.set 0 0)

  (type $packed_s (struct (field $packed_field i8)))
  (func (param (ref $packed_s))
    local.get 0
    i32.const 0
    struct.set $packed_s $packed_field))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
