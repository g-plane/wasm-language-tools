use super::*;
use insta::assert_json_snapshot;
use lspt::Position;
use wat_service::LanguageService;

#[test]
fn type_def_int_idx() {
    let uri = "untitled:test".to_string();
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
    let include_decl = service.find_references(create_params(
        uri.clone(),
        Position {
            line: 2,
            character: 9,
        },
        true,
    ));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(
        uri,
        Position {
            line: 2,
            character: 9,
        },
        false,
    ));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn type_def_ident_idx() {
    let uri = "untitled:test".to_string();
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
    let include_decl = service.find_references(create_params(
        uri.clone(),
        Position {
            line: 2,
            character: 15,
        },
        true,
    ));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(
        uri,
        Position {
            line: 2,
            character: 15,
        },
        false,
    ));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn type_use_int_idx() {
    let uri = "untitled:test".to_string();
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
    let include_decl = service.find_references(create_params(
        uri.clone(),
        Position {
            line: 3,
            character: 17,
        },
        true,
    ));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(
        uri,
        Position {
            line: 3,
            character: 17,
        },
        false,
    ));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn type_use_ident_idx() {
    let uri = "untitled:test".to_string();
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
    let include_decl = service.find_references(create_params(
        uri.clone(),
        Position {
            line: 4,
            character: 21,
        },
        true,
    ));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(
        uri,
        Position {
            line: 4,
            character: 21,
        },
        false,
    ));
    assert_json_snapshot!(exclude_decl);
}
