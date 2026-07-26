use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn field_def_int_idx() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (type (struct (field i32)))
  (type (struct (field i32)))
  (func
    struct.get 0 0
    struct.set 0 0))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 2, 24, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 24, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn field_def_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (type (struct (field $x i32)))
  (type (struct (field $x i32)))
  (func
    struct.get 0 0
    struct.set 0 $x))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 2, 24, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 24, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn field_ref_int_idx() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (type (struct (field i32)))
  (type (struct (field i32)))
  (func
    struct.get 0 0
    struct.set 0 0))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 5, 18, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 5, 18, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn field_ref_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (type (struct (field $x i32)))
  (type (struct (field $x i32)))
  (func
    struct.get 0 0
    struct.set 0 $x))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 6, 18, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 6, 18, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn undef() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (type (struct (field i32)))
  (func
    struct.get 0 $x
    struct.set 0 $x))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 4, 18, true));
    assert!(include_decl.unwrap().is_empty());
    let exclude_decl = service.find_references(create_params(uri, 4, 18, false));
    assert!(exclude_decl.unwrap().is_empty());
}
