use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn single() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    block (result i32)
      block (result f32)
        block (result i64)
          i32.const 0
          i32.const 0
          br_table 2
          unreachable
        end
        unreachable
      end
      unreachable
    end
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn multi_mismatch() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    block (result i32)
      block (result f32)
        block (result i64)
          i64.const 0
          i32.const 0
          br_table 0 1 2
          unreachable
        end
        unreachable
      end
      unreachable
    end
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn multi_match() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    block (result i64)
      block (result i64)
        block (result i64)
          i64.const 0
          i32.const 0
          br_table 0 1 2
          unreachable
        end
        unreachable
      end
      unreachable
    end
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
