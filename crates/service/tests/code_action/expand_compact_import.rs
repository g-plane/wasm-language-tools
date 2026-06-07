use super::*;
use insta::assert_json_snapshot;
use lspt::CodeActionKind;
use wat_service::LanguageService;

fn create_params(
    uri: String,
    start_line: u32,
    start_character: u32,
    end_line: u32,
    end_character: u32,
) -> CodeActionParams {
    CodeActionParams {
        text_document: TextDocumentIdentifier { uri },
        range: Range {
            start: Position {
                line: start_line,
                character: start_character,
            },
            end: Position {
                line: end_line,
                character: end_character,
            },
        },
        context: CodeActionContext {
            diagnostics: vec![],
            only: Some(vec![CodeActionKind::RefactorRewrite]),
            trigger_kind: None,
        },
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    }
}

#[test]
fn non_compact() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (import "" "" (func (param i32)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 19, 2, 19));
    assert!(response.is_none());
}

#[test]
fn no_extern_type() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (import "env" (item "a") (item "b"))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 19, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn missing_module_name() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (import (item "a" (global i32)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 7, 2, 7));
    assert!(response.is_none());
}

#[test]
fn missing_name() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (import "env" (item "a") (item) (global i32))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 7, 2, 7));
    assert_json_snapshot!(response);
}

#[test]
fn encoding1() {
    let uri = "untitled:test".to_string();
    let source = r#"
(import "env"
    (item "f" (func (result i32)))
    (item "t" (table 3 funcref))
    (item "m" (memory $m 1))
    (item "g" (global i32))
    (item "e" (tag (param i32)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 5, 7, 5, 7));
    assert_json_snapshot!(response);
}

#[test]
fn encoding2() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (import "env" (item "a") (item "b") (memory))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 2, 42, 2, 42));
    assert_json_snapshot!(response);
}
