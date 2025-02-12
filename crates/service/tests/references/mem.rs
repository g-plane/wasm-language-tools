use super::*;
use insta::assert_json_snapshot;
use lsp_types::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn memory_def_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
    (memory (data))
    (export "" (memory 0))
    (data (memory 0))
    (func
        (i32.store))
)
(module (memory))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(2, 11), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(2, 11), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn memory_def_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
    (memory $memory (data))
    (export "" (memory $memory))
    (data (memory $memory))
    (func
        (f32.store $memory))
)
(module (memory $memory))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(2, 19), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(2, 19), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn memory_ref_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
    (memory (data))
    (export "" (memory 0))
    (data (memory 0))
    (func
        (i64.store))
)
(module (memory))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 24), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 24), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn memory_ref_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
    (memory $memory (data))
    (export "" (memory $memory))
    (data (memory $memory))
    (func
        (f64.store $memory))
)
(module (memory $memory))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 30), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 30), false));
    assert_json_snapshot!(exclude_decl);
}
