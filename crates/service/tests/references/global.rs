use super::*;
use insta::assert_json_snapshot;
use lsp_types::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn global_def_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
    (global i32)
    (func (global.get 0))
    (export "" (global 0))
)
(module (global i32))
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
fn global_def_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
    (global $global i32)
    (func (global.get 0) (global.get $global))
    (export "" (global $global))
)
(module (global $global i32))
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
fn global_ref_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
    (global i32)
    (func (global.get 0))
    (export "" (global 0))
)
(module (global i32))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 23), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 23), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn global_ref_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
    (global $global i32)
    (func (global.get 0) (global.get $global))
    (export "" (global $global))
)
(module (global $global i32))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 44), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 44), false));
    assert_json_snapshot!(exclude_decl);
}
