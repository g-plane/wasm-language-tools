use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn def_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (elem)
  (func
    table.init 0
    table.init 0 0
    array.init_elem 0 0
    array.new_elem 0 0
    elem.drop 0))
(module
  (elem))
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
  (elem $elem)
  (func
    table.init $elem
    table.init 0 $elem
    array.init_elem 0 $elem
    array.new_elem 0 $elem
    elem.drop $elem))
(module
  (elem $elem))
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
  (elem)
  (func
    table.init 0
    table.init 0 0
    array.init_elem 0 0
    array.new_elem 0 0
    elem.drop 0))
(module
  (elem))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 4, 15, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 4, 15, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn ref_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (elem $elem)
  (func
    table.init $elem
    table.init 0 $elem
    array.init_elem 0 $elem
    array.new_elem 0 $elem
    elem.drop $elem))
(module
  (elem $elem))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 5, 20, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 5, 20, false));
    assert_json_snapshot!(exclude_decl);
}
