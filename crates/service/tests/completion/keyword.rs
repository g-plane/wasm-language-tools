use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn module_keyword() {
    let uri = "untitled:test".to_string();
    let source = "(";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 0, 1));
    assert_json_snapshot!(response);
}

#[test]
fn module_keyword_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "(mo";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 0, 3));
    assert_json_snapshot!(response);
}

#[test]
fn module_keyword_in_empty() {
    let uri = "untitled:test".to_string();
    let source = " ";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 0, 1));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_keyword() {
    let uri = "untitled:test".to_string();
    let source = "(module ())";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 0, 9));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_keyword_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "(module (f)";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 0, 10));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_keyword_without_paren() {
    let uri = "untitled:test".to_string();
    let source = "(module )";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 0, 8));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_func_keyword() {
    let uri = "untitled:test".to_string();
    let source = "(module (func ( (i32.const 0)))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 0, 15));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_func_keyword_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "(module (func (l (i32.const 0)))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 0, 16));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_func_keyword_after_other_syntax() {
    let uri = "untitled:test".to_string();
    let source = "(module (func (param) ( (i32.const 0)))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 0, 23));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_func_keyword_in_middle() {
    let uri = "untitled:test".to_string();
    let source = "(module (func (param) ( (param) (i32.const 0)))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 0, 23));
    assert_json_snapshot!(response);
}

#[test]
fn no_module_field_func_keyword_without_paren() {
    let uri = "untitled:test".to_string();
    let source = "(module (func p))"; // shouldn't provide "param"
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 0, 15));
    assert_json_snapshot!(response);
}

#[test]
fn export_desc_keyword() {
    let uri = "untitled:test".to_string();
    let source = "(module (export \"\" ())";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 0, 20));
    assert_json_snapshot!(response);
}

#[test]
fn export_desc_keyword_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "(module (export \"\" (f))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 0, 21));
    assert_json_snapshot!(response);
}

#[test]
fn export_desc_keyword_without_paren() {
    let uri = "untitled:test".to_string();
    let source = "(module (export \"\" ))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 0, 19));
    assert!(response.is_none());
}

#[test]
fn module_field_memory_keyword() {
    let uri = "untitled:test".to_string();
    let source = "(module (memory ( ))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 0, 17));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_memory_keyword_without_paren() {
    let uri = "untitled:test".to_string();
    let source = "(module (memory ))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 0, 16));
    assert!(response.is_none());
}

#[test]
fn block_type_result_keyword_after_paren() {
    let uri = "untitled:test".to_string();
    let source = "(module (func (block ())))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 0, 22));
    assert_json_snapshot!(response);
}

#[test]
fn block_type_result_keyword_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "(module (func (block (r))))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 0, 23));
    assert_json_snapshot!(response);
}

#[test]
fn call_indirect() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (call_indirect ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 3, 20));
    assert_json_snapshot!(response);
}

#[test]
fn call_indirect_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (call_indirect (p))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 3, 21));
    assert_json_snapshot!(response);
}

#[test]
fn return_call_indirect() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (return_call_indirect ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 3, 27));
    assert_json_snapshot!(response);
}

#[test]
fn return_call_indirect_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (return_call_indirect (p))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 3, 28));
    assert_json_snapshot!(response);
}

#[test]
fn ref_test() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (ref.test ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 3, 15));
    assert_json_snapshot!(response);
}

#[test]
fn ref_cast() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (ref.cast ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 3, 15));
    assert_json_snapshot!(response);
}
