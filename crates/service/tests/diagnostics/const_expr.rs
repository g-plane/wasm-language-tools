use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn invalid() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global (mut i32)
    (i32.add
      (i32.const 0)
      (i32.div_s
        (i32.const 1)
        (i32.const 2))))

  (table 0 funcref
    unreachable)
  (elem (table 0)
    (offset
      (block (result i32)
        (i32.const 0))))
  (func)
  (elem funcref
    (item
      ref.func 0
      loop
      end))

  (memory 1)
  (data (memory 0)
    (offset
      (i32.const 0)
      try_table
      end)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn valid() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global i32
    (i32.add
      (i32.const 1)
      (i32.const 2)))
  (global i32
    global.get 0)

  (type (func))
  (table 0 funcref)
  (table 0 funcref
    (ref.null 0))
  (elem (table 0)
    (offset
      i32.const 0))
  (elem funcref
    (ref.null 0))

  (memory 1)
  (data (memory 0)
    (i32.const 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
