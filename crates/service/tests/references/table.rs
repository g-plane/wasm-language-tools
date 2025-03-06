use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn table_def_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table 0 funcref)
  (func
    (table.size 0)
    (call_indirect)))
(module
  (table))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 2, 8, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 8, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn table_def_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $table 0 funcref)
  (func
    (table.size $table)
    (call_indirect $table)))
(module
  (table $table))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 2, 14, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 2, 14, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn table_ref_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table 0 funcref)
  (func
    (table.size 0)
    (call_indirect)))
(module
  (table))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 4, 17, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 4, 17, false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn table_ref_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $table 0 funcref)
  (func
    (table.size $table)
    (call_indirect $table)))
(module
  (table $table))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(uri.clone(), 4, 20, true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, 4, 20, false));
    assert_json_snapshot!(exclude_decl);
}
