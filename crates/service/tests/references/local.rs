use super::*;
use insta::assert_json_snapshot;
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
    let include_decl = service.find_references(create_params(uri.clone(), 2, 20, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 20, false));
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
    let include_decl = service.find_references(create_params(uri.clone(), 2, 23, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 23, false));
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
    let include_decl = service.find_references(create_params(uri.clone(), 2, 20, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 20, false));
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
    let include_decl = service.find_references(create_params(uri.clone(), 2, 23, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 23, false));
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
    let include_decl = service.find_references(create_params(uri.clone(), 3, 20, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 3, 20, false));
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
    let include_decl = service.find_references(create_params(uri.clone(), 3, 38, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 3, 38, false));
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
    let include_decl = service.find_references(create_params(uri.clone(), 3, 20, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 3, 20, false));
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
    let include_decl = service.find_references(create_params(uri.clone(), 3, 38, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 3, 38, false));
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
    let include_decl = service.find_references(create_params(uri.clone(), 3, 20, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 3, 20, false));
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
    let include_decl = service.find_references(create_params(uri.clone(), 3, 24, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 3, 24, false));
    assert_json_snapshot!(exclude_decl);
}
