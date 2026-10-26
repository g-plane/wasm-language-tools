use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn after_instr_name() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.load ))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 20));
    assert_json_snapshot!(response);
}

#[test]
fn incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.load off))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 23));
    assert_json_snapshot!(response);
}

#[test]
fn after_mem_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (i32.load 0 )))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 20));
    assert_json_snapshot!(response);
}

#[test]
fn i32_atomic() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (i32.atomic.load8 )))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 26));
    assert_json_snapshot!(response);
}

#[test]
fn i64_atomic() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (i64.atomic.rmw.sub )))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 28));
    assert_json_snapshot!(response);
}

#[test]
fn memory_atomic() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (memory.atomic.notify )))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 30));
    assert_json_snapshot!(response);
}
