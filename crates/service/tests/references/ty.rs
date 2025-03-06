use super::*;
use insta::assert_json_snapshot;
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
    let include_decl = service.find_references(create_params(uri.clone(), 2, 9, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 9, false));
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
    let include_decl = service.find_references(create_params(uri.clone(), 2, 15, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 15, false));
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
    let include_decl = service.find_references(create_params(uri.clone(), 3, 17, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 3, 17, false));
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
    let include_decl = service.find_references(create_params(uri.clone(), 4, 21, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 4, 21, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn struct_def_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct))
  (func
    struct.new 0))
(module
  (type (struct)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 2, 6, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 6, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn struct_def_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct))
  (func
    struct.new 0
    struct.new_default $s))
(module
  (type $s (struct)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 2, 9, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 9, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn struct_access_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct))
  (func
    struct.new 0))
(module
  (type (struct)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 4, 15, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 4, 15, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn struct_access_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct))
  (func
    struct.new 0
    struct.new_default $s))
(module
  (type $s (struct)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 5, 24, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 5, 24, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn array_def_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (array))
  (func
    array.new 0))
(module
  (type (array)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 2, 6, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 6, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn array_def_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $a (array))
  (func
    array.new 0
    array.new_default $a))
(module
  (type $a (array)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 2, 9, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 9, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn array_access_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (array))
  (func
    array.new 0))
(module
  (type (array)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 4, 14, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 4, 14, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn array_access_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $a (array))
  (func
    array.new 0
    array.new_default $a))
(module
  (type $a (array)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 5, 23, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 5, 23, false));
    assert_json_snapshot!(exclude_decl);
}
