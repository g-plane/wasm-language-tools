use super::*;
use insta::assert_json_snapshot;
use lspt::Position;
use wat_service::LanguageService;

#[test]
fn top_level() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (table )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 11,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn after_top_level_paren() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (table ())
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 12,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn table_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (table 0 )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 13,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn elem_without_parens() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $f)
    (table funcref (elem ))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 3,
            character: 25,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn elem_with_parens() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (table funcref (elem ()))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 26,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn instr_after_item() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (table funcref (elem (item )))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 31,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn instr_after_item_and_paren() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (table funcref (elem (item ())))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 32,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn table_size() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $table 0 funcref)
  (table 0 funcref)
  (func
    (table.size )))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 5,
            character: 16,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn table_size_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $table 0 funcref)
  (table 0 funcref)
  (func
    (table.size $)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 5,
            character: 17,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn table_size_incomplete_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $table 0 funcref)
  (table 0 funcref)
  (func
    (table.size $t)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 5,
            character: 18,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn export() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $table 1 funcref)
  (table 0 funcref)
  (export \"\" (table )))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 4,
            character: 20,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn export_following_int_index() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $table 1 funcref)
  (table 0 funcref)
  (export \"\" (table 1)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 4,
            character: 21,
        },
    ));
    assert_json_snapshot!(response);
}
