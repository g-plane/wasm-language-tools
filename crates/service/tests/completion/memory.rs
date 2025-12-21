use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn extern_idx_memory() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (export \"\" (memory ))
    (memory $memory (data))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 23));
    assert_json_snapshot!(response);
}

#[test]
fn extern_idx_memory_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (export \"\" (memory $))
    (memory $memory (data))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 24));
    assert_json_snapshot!(response);
}

#[test]
fn extern_idx_memory_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (export \"\" (memory $m))
    (memory $memory (data))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 25));
    assert_json_snapshot!(response);
}

#[test]
fn load_and_store_instr() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 1)
  (func
    (i32.load ))
  (memory $memory 1))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 4, 14));
    assert_json_snapshot!(response);
}

#[test]
fn load_and_store_instr_following_mem_arg() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 1)
  (func
    (i32.load offset=0 ))
  (memory $memory 1))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 4, 23));
    assert_json_snapshot!(response);
}

#[test]
fn load_and_store_instr_after_mem_arg() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 1)
  (func
    (i32.load offset=0 0))
  (memory $memory 1))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 4, 24));
    assert_json_snapshot!(response);
}

#[test]
fn memory_size() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 1)
  (func
    (memory.size ))
  (memory $memory 1))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 4, 17));
    assert_json_snapshot!(response);
}

#[test]
fn memory_fill() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 1)
  (func
    (memory.size 0 ))
  (memory $memory 1))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 4, 19));
    assert_json_snapshot!(response);
}

#[test]
fn memory_init() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 1)
  (func
    (memory.init 0 ))
  (memory $memory 1))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 4, 19));
    assert!(response.is_none());
}

#[test]
fn addr_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory  1))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 10));
    assert_json_snapshot!(response);
}

#[test]
fn addr_type_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory i 1))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 11));
    assert_json_snapshot!(response);
}

#[test]
fn deprecated() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func
    (memory.size ))
  (@deprecated)
  (memory 1)
  (@deprecated "this is deprecated")
  (memory $memory 1))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 3, 17));
    assert_json_snapshot!(response);
}

#[test]
fn pagesize_keyword_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory (p))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 12));
    assert_json_snapshot!(response);
}

#[test]
fn pagesize() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 1 (pagesize ))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 22));
    assert_json_snapshot!(response);
}

#[test]
fn pagesize_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 1 (pagesize 6))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 23));
    assert_json_snapshot!(response);
}
