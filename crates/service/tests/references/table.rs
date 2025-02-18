use super::*;
use insta::assert_json_snapshot;
use lspt::Position;
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
    let include_decl = service.find_references(create_params(
        uri.clone(),
        Position {
            line: 2,
            character: 8,
        },
        true,
    ));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(
        uri,
        Position {
            line: 2,
            character: 8,
        },
        false,
    ));
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
    let include_decl = service.find_references(create_params(
        uri.clone(),
        Position {
            line: 2,
            character: 14,
        },
        true,
    ));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(
        uri,
        Position {
            line: 2,
            character: 14,
        },
        false,
    ));
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
    let include_decl = service.find_references(create_params(
        uri.clone(),
        Position {
            line: 4,
            character: 17,
        },
        true,
    ));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(
        uri,
        Position {
            line: 4,
            character: 17,
        },
        false,
    ));
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
    let include_decl = service.find_references(create_params(
        uri.clone(),
        Position {
            line: 4,
            character: 20,
        },
        true,
    ));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(
        uri,
        Position {
            line: 4,
            character: 20,
        },
        false,
    ));
    assert_json_snapshot!(exclude_decl);
}
