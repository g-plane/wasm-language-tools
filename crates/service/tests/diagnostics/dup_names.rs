use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn same_kind() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f (param i32))
  (func $f (result f32) (f32.const 0)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn different_kinds() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global $f i32 i32.const 0)
  (func $f))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn param_and_local() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param $a i32) (local $a f32)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn different_scopes() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param $p i32))
  (func (param $p i32)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn ref_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $fn
    (call $fn)
    (call $fn)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn type_def() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (type $_ (func))
  (type $_ (func)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn global() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (global $_ i32
    i32.const 0)
  (global $_ i32
    i32.const 0))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn memory() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (memory $_ 0)
  (memory $_ 0))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn table() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (table $_ 0 funcref)
  (table $_ 0 funcref))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn fields() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field $x i32) (field $x i32)))
  (type (struct (field $x i32))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn tag() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (tag $_)
  (tag $_))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn exports() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func (export "func"))
  (export "func" (func 0))
  (func (export "f1") (export "f1")))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
