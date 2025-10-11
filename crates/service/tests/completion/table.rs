use super::*;
use insta::assert_json_snapshot;
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
    let response = service.completion(create_params(uri, 2, 11));
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
    let response = service.completion(create_params(uri, 2, 12));
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
    let response = service.completion(create_params(uri, 2, 13));
    assert_json_snapshot!(response);
}

#[test]
fn addr_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (table  0)
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 11));
    assert_json_snapshot!(response);
}

#[test]
fn addr_type_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (table i 0)
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 12));
    assert_json_snapshot!(response);
}

#[test]
fn ref_in_table_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (table 0 ()
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 14));
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
    let response = service.completion(create_params(uri, 3, 25));
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
    let response = service.completion(create_params(uri, 2, 26));
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
    let response = service.completion(create_params(uri, 2, 31));
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
    let response = service.completion(create_params(uri, 2, 32));
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
    let response = service.completion(create_params(uri, 5, 16));
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
    let response = service.completion(create_params(uri, 5, 17));
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
    let response = service.completion(create_params(uri, 5, 18));
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
    let response = service.completion(create_params(uri, 4, 20));
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
    let response = service.completion(create_params(uri, 4, 21));
    assert_json_snapshot!(response);
}

#[test]
fn call_indirect() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table 0 funcref)
  (func
    call_indirect ))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 4, 18));
    assert_json_snapshot!(response);
}

#[test]
fn call_indirect_following_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table 0 funcref)
  (func
    call_indirect 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 4, 19));
    assert_json_snapshot!(response);
}

#[test]
fn call_indirect_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $table 0 funcref)
  (func
    call_indirect $))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 4, 19));
    assert_json_snapshot!(response);
}

#[test]
fn call_indirect_incomplete_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $table 0 funcref)
  (func
    call_indirect $t))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 4, 20));
    assert_json_snapshot!(response);
}

#[test]
fn return_call_indirect() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table 0 funcref)
  (func
    return_call_indirect ))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 4, 25));
    assert_json_snapshot!(response);
}

#[test]
fn return_call_indirect_following_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table 0 funcref)
  (func
    return_call_indirect 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 4, 26));
    assert_json_snapshot!(response);
}

#[test]
fn return_call_indirect_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $table 0 funcref)
  (func
    return_call_indirect $))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 4, 26));
    assert_json_snapshot!(response);
}

#[test]
fn return_call_indirect_incomplete_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $table 0 funcref)
  (func
    return_call_indirect $t))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 4, 27));
    assert_json_snapshot!(response);
}

#[test]
fn instr() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table 0 funcref ))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn instr_following_paren() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table 0 funcref ())
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 20));
    assert_json_snapshot!(response);
}
