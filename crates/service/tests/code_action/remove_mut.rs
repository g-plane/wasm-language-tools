use super::*;
use insta::assert_json_snapshot;
use lspt::{Diagnostic, Position, Range, Union2};
use wat_service::LanguageService;

fn create_params(uri: String, range: Range, token_range: Range) -> CodeActionParams {
    CodeActionParams {
        text_document: TextDocumentIdentifier { uri },
        range,
        context: CodeActionContext {
            diagnostics: vec![Diagnostic {
                range: token_range,
                code: Some(Union2::B("needless-mut".into())),
                ..Default::default()
            }],
            only: None,
            trigger_kind: None,
        },
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    }
}

#[test]
fn no_mut() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global $a i32))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(super::create_params(uri, 2, 15, 2, 15));
    assert!(response.is_none());
}

#[test]
fn no_diagnostics() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global $a (mut i32)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(super::create_params(uri, 2, 15, 2, 15));
    assert!(response.is_none());
}

#[test]
fn unrelated_range() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global $a (mut i32))
  (global $b (mut i32)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 2,
                character: 15,
            },
            end: Position {
                line: 2,
                character: 15,
            },
        },
        Range {
            start: Position {
                line: 3,
                character: 14,
            },
            end: Position {
                line: 3,
                character: 17,
            },
        },
    ));
    assert!(response.is_none());
}

#[test]
fn unrelated_diagnostic() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global $a (mut i32))
  (global $b (mut i32)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(CodeActionParams {
        text_document: TextDocumentIdentifier { uri },
        range: Range {
            start: Position {
                line: 2,
                character: 15,
            },
            end: Position {
                line: 2,
                character: 15,
            },
        },
        context: CodeActionContext {
            diagnostics: vec![Diagnostic {
                range: Range {
                    start: Position {
                        line: 2,
                        character: 15,
                    },
                    end: Position {
                        line: 2,
                        character: 15,
                    },
                },
                code: Some(Union2::B("global-mut".into())),
                ..Default::default()
            }],
            only: None,
            trigger_kind: None,
        },
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    });
    assert!(response.is_none());
}

#[test]
fn simple() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global $a (mut i32)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 2,
                character: 15,
            },
            end: Position {
                line: 2,
                character: 15,
            },
        },
        Range {
            start: Position {
                line: 2,
                character: 14,
            },
            end: Position {
                line: 2,
                character: 17,
            },
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn missing_r_paren() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global $a (mut i32
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 2,
                character: 15,
            },
            end: Position {
                line: 2,
                character: 15,
            },
        },
        Range {
            start: Position {
                line: 2,
                character: 14,
            },
            end: Position {
                line: 2,
                character: 17,
            },
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn with_comments() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global $a ((;a;) mut(;b;) i32)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 2,
                character: 20,
            },
            end: Position {
                line: 2,
                character: 20,
            },
        },
        Range {
            start: Position {
                line: 2,
                character: 20,
            },
            end: Position {
                line: 2,
                character: 23,
            },
        },
    ));
    assert_json_snapshot!(response);
}
