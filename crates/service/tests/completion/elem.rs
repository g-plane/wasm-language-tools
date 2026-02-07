use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn module_field() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (elem )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 10));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_with_parens() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (elem ())
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 11));
    assert_json_snapshot!(response);
}

#[test]
fn declare_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (elem dec)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 13));
    assert_json_snapshot!(response);
}

#[test]
fn ref_type_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (elem fun)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 13));
    assert_json_snapshot!(response);
}

#[test]
fn ref_type_incomplete_after_declare() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (elem declare fun)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 13));
    assert_json_snapshot!(response);
}

#[test]
fn elem_expr() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (elem funcref ())
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn elem_expr_with_item_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (elem funcref (item ))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 24));
    assert_json_snapshot!(response);
}

#[test]
fn table_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (elem (tab))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 14));
    assert_json_snapshot!(response);
}

#[test]
fn table_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (elem (table ))
    (table $table)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 17));
    assert_json_snapshot!(response);
}

#[test]
fn parens_after_table_use() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (elem (table) ())
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn offset_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (elem (off))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 14));
    assert_json_snapshot!(response);
}

#[test]
fn offset_incomplete_after_table_use() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (elem (table) (off))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 22));
    assert_json_snapshot!(response);
}

#[test]
fn offset() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (elem (offset ))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 18));
    assert_json_snapshot!(response);
}

#[test]
fn offset_after_table_use() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (elem (table) (offset ))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 26));
    assert_json_snapshot!(response);
}

#[test]
fn elem_list_after_offset() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (elem (offset) ())
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 20));
    assert_json_snapshot!(response);
}

#[test]
fn func_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (elem func )
    (func $f)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 15));
    assert_json_snapshot!(response);
}

#[test]
fn func_idxes() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (elem func $f )
    (func $f)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 18));
    assert_json_snapshot!(response);
}

#[test]
fn elem_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (elem $e)
  (elem)
  (func
    (elem.drop )))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 15));
    assert_json_snapshot!(response);
}

#[test]
fn elem_idx_following_int() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (elem $e)
  (elem)
  (func
    (elem.drop 1)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 16));
    assert_json_snapshot!(response);
}

#[test]
fn elem_idx_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (elem $e)
  (elem)
  (func
    (elem.drop $)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 16));
    assert_json_snapshot!(response);
}

#[test]
fn elem_idx_following_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (elem $e)
  (elem)
  (func
    (elem.drop $e)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 17));
    assert_json_snapshot!(response);
}

#[test]
fn table_init() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $t)
  (elem $e)
  (func
    (table.init )))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 16));
    assert_json_snapshot!(response);
}

#[test]
fn table_init_after_first_immediate() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $t)
  (elem $e)
  (func
    (table.init 0 )))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 18));
    assert_json_snapshot!(response);
}

#[test]
fn array_init_elem() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (elem $e)
  (elem)
  (func
    (array.init_elem 0 )))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 23));
    assert_json_snapshot!(response);
}

#[test]
fn array_new_elem() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (elem $e)
  (elem)
  (func
    (array.new_elem 0 )))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 22));
    assert_json_snapshot!(response);
}

#[test]
fn deprecated() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (@deprecated)
  (elem $e)
  (func
    (elem.drop )))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 15));
    assert_json_snapshot!(response);
}
