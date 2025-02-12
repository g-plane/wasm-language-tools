use super::*;
use insta::assert_json_snapshot;
use lsp_types::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn func_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func
        (call 0)
    )
)
(module (func))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(2, 9), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(2, 9), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn func_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $func
        (call 0) (call $func)
    )
)
(module (func $func))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(2, 15), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(2, 15), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn func_in_implicit_module() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(func $func)
(func (call $func))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(1, 8), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(1, 8), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn call_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func
        (call 0)
    )
    (export \"\" (func 0))
)
(module (func))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 15), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 15), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn call_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $func
        (call 0) (call $func)
    )
    (export \"\" (func $func))
)
(module (func $func))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 28), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 28), false));
    assert_json_snapshot!(exclude_decl);
}
