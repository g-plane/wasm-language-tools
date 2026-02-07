use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

fn disable_other_lints(service: &mut LanguageService, uri: &str) {
    service.set_config(
        uri,
        Some(ServiceConfig {
            lint: Lints {
                unused: LintLevel::Allow,
                needless_mut: LintLevel::Warn,
                ..Default::default()
            },
            ..Default::default()
        }),
    );
}

#[test]
fn immutable_global() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global i32
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn inline_exported_global() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (global (export "") (mut i32)
    i32.const 0))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn module_field_export() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (export "" (global 0))
  (global (mut i32)
    i32.const 0))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn global_set_with_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    i32.const 0
    global.set 0)
  (global $global (mut i32)
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn global_set_with_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    i32.const 0
    global.set $global)
  (global $global (mut i32)
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn global_get_with_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    global.get 0
    drop)
  (global $global (mut i32)
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn global_get_with_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    global.get $global
    drop)
  (global $global (mut i32)
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn unused_global() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global (mut i32)
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn imported_global() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (import "" "" (global $global (mut i32)))
  (func
    global.get $global
    drop))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn immutable_array() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (array i32)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn unused_array() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (array (mut i32))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn array_get() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (array (mut i32)))
  (func (param (ref 0)) (result i32)
    local.get 0
    i32.const 0
    array.get 0)

  (type $packed (array (mut i16)))
  (func (param (ref $packed))
    local.get 0
    i32.const 0
    array.get_s $packed
    drop
    local.get 0
    i32.const 0
    array.get_u $packed
    drop))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn array_update() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (array (mut i32)))
  (func (param (ref 0))
    local.get 0
    i32.const 0
    i32.const 0
    array.set 0

    local.get 0
    i32.const 0
    i32.const 0
    i32.const 0
    array.fill 0

    local.get 0
    i32.const 0
    i32.const 0
    i32.const 0
    array.init_data 0 0

    local.get 0
    i32.const 0
    i32.const 0
    i32.const 0
    array.init_elem 0 0)
  (data)
  (elem 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn array_copy() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $dst (array (mut i32)))
  (type $src (array (mut i32)))
  (func (param (ref $dst) (ref $src))
    local.get 0
    i32.const 0
    local.get 1
    i32.const 0
    i32.const 0
    array.copy $dst $src))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn immutable_struct() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field i32))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn unused_struct() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field (mut i32)))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn struct_get() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field (mut i32))))
  (func (param (ref 0)) (result i32)
    local.get 0
    struct.get 0 0)

  (type $packed_s (struct (field $packed_field (mut i16))))
  (func (param (ref $packed_s)) (result i32 i32)
    local.get 0
    struct.get_s 1 0
    local.get 0
    struct.get_u $packed_s $packed_field))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn struct_set() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field (mut i32))))
  (func (param (ref 0))
    local.get 0
    i32.const 0
    struct.set 0 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
