use super::*;
use insta::assert_json_snapshot;
use lspt::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn export_desc_memory() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (export \"\" (memory ))
    (memory $memory (data))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position { line: 2, character: 23 }));
    assert_json_snapshot!(response);
}

#[test]
fn export_desc_memory_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (export \"\" (memory $))
    (memory $memory (data))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position { line: 2, character: 24 }));
    assert_json_snapshot!(response);
}

#[test]
fn export_desc_memory_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (export \"\" (memory $m))
    (memory $memory (data))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position { line: 2, character: 25 }));
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
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position { line: 4, character: 14 }));
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
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position { line: 4, character: 23 }));
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
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position { line: 4, character: 24 }));
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
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position { line: 4, character: 17 }));
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
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position { line: 4, character: 19 }));
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
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position { line: 4, character: 19 }));
    assert!(response.is_none());
}
