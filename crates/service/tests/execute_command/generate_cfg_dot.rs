use insta::assert_snapshot;
use lspt::{ExecuteCommandParams, Position};
use wat_service::LanguageService;

const COMMAND: &str = "wasmLanguageTools.__generateControlFlowGraphDot";

#[test]
fn non_func() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (global i32
    i32.const 0)
  (func (param (ref any)) (result (ref any)) (local (ref any))
    (block $b)
    (local.get 1)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.execute_command(ExecuteCommandParams {
        command: COMMAND.into(),
        arguments: Some(vec![
            serde_json::Value::String(uri),
            serde_json::to_value(Position { line: 3, character: 8 }).unwrap(),
        ]),
        work_done_token: Default::default(),
    });
    assert!(response.is_none());
}

#[test]
fn func() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func (param (ref any)) (result (ref any)) (local (ref any))
    (block $b
      (loop $loop
        (if
          (global.get 0)
          (then
            (br $b)
            (local.set 1
              (local.get 0)))
          (else
            (local.set 1
              (local.get 0))
            (br $loop)))))
    (local.get 1)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.execute_command(ExecuteCommandParams {
        command: COMMAND.into(),
        arguments: Some(vec![
            serde_json::Value::String(uri),
            serde_json::to_value(Position { line: 8, character: 13 }).unwrap(),
        ]),
        work_done_token: Default::default(),
    });
    assert_snapshot!(response.unwrap().as_str().unwrap());
}
