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
    service.commit(&uri, source.into());
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
    service.commit(&uri, source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 2, 23, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 23, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn param_int_idx_in_type_def() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (func (param i32 i32) (result i32 i32)))
  (func (type 0)
    (local.get 1)
    (local.get 1))
  (func (type 0)
    (local.get 1)
    (local.get 1)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 2, 27, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 27, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn param_ident_idx_in_type_def() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (func (param $a i32) (param $b i32) (result i32 i32)))
  (func (type 0)
    (local.get 1)
    (local.get 1))
  (func (type 0)
    (local.get 1)
    (local.get 1)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 2, 37, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 37, false));
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
    service.commit(&uri, source.into());
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
    service.commit(&uri, source.into());
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
    service.commit(&uri, source.into());
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
    service.commit(&uri, source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 3, 38, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 3, 38, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn param_access_via_int_idx_type_def() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (func (param i32 i32) (result i32 i32)))
  (func (type 0)
    (local.get 1)
    (local.get 1))
  (func (type 0)
    (local.get 1)
    (local.get 1)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 5, 15, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 5, 15, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn param_access_via_ident_idx_type_def() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (func (param $a i32) (param $b i32) (result i32 i32)))
  (func (type 0)
    (local.get 1)
    (local.get 1))
  (func (type 0)
    (local.get 1)
    (local.get 1)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 5, 15, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 5, 15, false));
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
    service.commit(&uri, source.into());
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
    service.commit(&uri, source.into());
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
    service.commit(&uri, source.into());
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
    service.commit(&uri, source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 3, 24, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 3, 24, false));
    assert_json_snapshot!(exclude_decl);
}
