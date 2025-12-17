use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn unrelated_instr() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (nop))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 11, 2, 11));
    assert!(response.is_none());
}

#[test]
fn no_call() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (return))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 11, 2, 11));
    assert!(response.is_none());
}

#[test]
fn no_call_inside() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (return (nop)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 11, 2, 11));
    assert!(response.is_none());
}

#[test]
fn no_call_before() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (nop) (return))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 17, 2, 17));
    assert!(response.is_none());
}

#[test]
fn sth_else_after_call() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (call) (nop))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 12, 2, 12));
    assert!(response.is_none());
}

#[test]
fn sth_else_outside_call() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (nop (call)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 17, 2, 17));
    assert!(response.is_none());
}

#[test]
fn missing_callee() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (call)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 11, 2, 11));
    assert!(response.is_none());
}

#[test]
fn not_recursive() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (call 1))
  (func))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 12, 2, 12));
    assert!(response.is_none());
}

#[test]
fn not_tail() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (call 0) (block)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 12, 2, 12));
    assert!(response.is_none());
}

#[test]
fn call_only() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (call 0)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 12, 2, 12));
    assert_json_snapshot!(response);
}

#[test]
fn call_before_return() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func $f call $f return)
  (export "f" (func $f)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 14, 2, 14));
    assert_json_snapshot!(response);
}

#[test]
fn call_inside_return() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (return (call 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 19, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn return_after_call() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func $f call $f return)
  (export "f" (func $f)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 23, 2, 23));
    assert_json_snapshot!(response);
}

#[test]
fn return_outside_call() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func $f (return (call $f)))
  (export "f" (func $f)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 12, 2, 12));
    assert_json_snapshot!(response);
}

#[test]
fn nested_blocks() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    if
      if
        loop
          block
            if
              call 0
            end
          end
        end
      end
    end))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 8, 18, 8, 18));
    assert_json_snapshot!(response);
}
