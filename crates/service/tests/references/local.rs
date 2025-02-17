use super::*;
use insta::assert_json_snapshot;
use lspt::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn param_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param i64)
        (local.get 0)
    )
    (func (param i64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position { line: 2, character: 20 }, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position { line: 2, character: 20 }, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn param_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param $param i64)
        (local.get 0) (local.get $param)
    )
    (func (param $param i64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position { line: 2, character: 23 }, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position { line: 2, character: 23 }, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn local_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (local i64)
        (local.get 0)
    )
    (func (local i64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position { line: 2, character: 20 }, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position { line: 2, character: 20 }, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn local_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (local $local i64)
        (local.get 0) (local.get $local)
    )
    (func (local $local i64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position { line: 2, character: 23 }, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position { line: 2, character: 23 }, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn param_access_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param i64)
        (local.get 0)
    )
    (func (param i64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position { line: 3, character: 20 }, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position { line: 3, character: 20 }, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn param_access_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param $param i64) (local $local i64)
        (local.get 0) (local.get $param)
    )
    (func (param $param i64) (local $local i64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position { line: 3, character: 38 }, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position { line: 3, character: 38 }, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn local_access_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (local i64)
        (local.get 0)
    )
    (func (local i64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position { line: 3, character: 20 }, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position { line: 3, character: 20 }, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn local_access_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param $param i64) (local $local i64)
        (local.get 1) (local.get $local)
    )
    (func (param $param i64) (local $local i64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position { line: 3, character: 38 }, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position { line: 3, character: 38 }, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn local_ref_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param i64) (local i64)
        (local.get 1)
    )
    (func (param i64) (local i64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position { line: 3, character: 20 }, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position { line: 3, character: 20 }, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn local_ref_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param $name i32) (local $name i64)
        (local.get $name)
    )
    (func (param $name i32) (local $name i64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position { line: 3, character: 24 }, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position { line: 3, character: 24 }, false));
    assert_json_snapshot!(exclude_decl);
}
