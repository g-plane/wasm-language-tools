use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn def_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (data)
  (func
    memory.init 0
    memory.init 0 0
    array.init_data 0 0
    array.new_data 0 0
    data.drop 0))
(module
  (data))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 2, 6, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 6, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn def_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (data $data)
  (func
    memory.init $data
    memory.init 0 $data
    array.init_data 0 $data
    array.new_data 0 $data
    data.drop $data))
(module
  (data $data))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 2, 12, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 12, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn ref_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (data)
  (func
    memory.init 0
    memory.init 0 0
    array.init_data 0 0
    array.new_data 0 0
    data.drop 0))
(module
  (data))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 4, 16, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 4, 16, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn ref_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (data $data)
  (func
    memory.init $data
    memory.init 0 $data
    array.init_data 0 $data
    array.new_data 0 $data
    data.drop $data))
(module
  (data $data))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 5, 20, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 5, 20, false));
    assert_json_snapshot!(exclude_decl);
}
