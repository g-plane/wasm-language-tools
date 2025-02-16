use super::*;
use insta::assert_json_snapshot;
use lsp_types::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn type_def_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (type (func))
    (func (type 0))
    (type (sub 0) (func (param (ref 0))))
)
(module (type (func)))
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
fn type_def_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (type $type (func))
    (func (type 0))
    (func (type $type))
    (type (sub $type) (func (param (ref $type))))
)
(module (type $type))
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
fn type_use_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (type (func))
    (func (type 0))
    (type (sub 0) (func (param (ref 0))))
)
(module (type (func)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 17), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 17), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn type_use_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (type $type (func))
    (func (type 0))
    (func (type $type))
    (type (sub $type) (func (param (ref $type))))
)
(module (type $type))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(4, 21), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(4, 21), false));
    assert_json_snapshot!(exclude_decl);
}
