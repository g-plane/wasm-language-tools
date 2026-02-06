use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn module_field() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (data ())
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 11));
    assert_json_snapshot!(response);
}

#[test]
fn memory_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (data (me))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 13));
    assert_json_snapshot!(response);
}

#[test]
fn memory_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (data (memory ))
    (memory $m 1)
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 18));
    assert_json_snapshot!(response);
}

#[test]
fn offset_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (data (of))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 13));
    assert_json_snapshot!(response);
}

#[test]
fn instr() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (data (offset ))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 18));
    assert_json_snapshot!(response);
}

#[test]
fn instr_inside_parens() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (data (offset ()))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn data_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (data $d)
  (data)
  (func
    (data.drop )))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 15));
    assert_json_snapshot!(response);
}

#[test]
fn data_idx_following_int() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (data $d)
  (data)
  (func
    (data.drop 1)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 16));
    assert_json_snapshot!(response);
}

#[test]
fn data_idx_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (data $d)
  (data)
  (func
    (data.drop $)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 16));
    assert_json_snapshot!(response);
}

#[test]
fn data_idx_following_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (data $d)
  (data)
  (func
    (data.drop $d)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 17));
    assert_json_snapshot!(response);
}

#[test]
fn memory_init() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory $m)
  (data $d)
  (func
    (memory.init )))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 17));
    assert_json_snapshot!(response);
}

#[test]
fn memory_init_after_first_immediate() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory $m)
  (data $d)
  (func
    (memory.init 0 )))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 19));
    assert_json_snapshot!(response);
}

#[test]
fn array_init_data() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (data $d)
  (data)
  (func
    (array.init_data 0 )))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 23));
    assert_json_snapshot!(response);
}

#[test]
fn array_new_data() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (data $d)
  (data)
  (func
    (array.new_data 0 )))
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
  (data $d)
  (func
    (data.drop )))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 15));
    assert_json_snapshot!(response);
}
