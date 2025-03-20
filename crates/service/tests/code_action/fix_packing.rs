use super::*;
use insta::assert_json_snapshot;
use lspt::{Diagnostic, Position, Range, Union2};
use wat_service::LanguageService;

fn create_params(uri: String, range: Range, diagnostic_range: Range) -> CodeActionParams {
    CodeActionParams {
        text_document: TextDocumentIdentifier { uri },
        range,
        context: CodeActionContext {
            diagnostics: vec![Diagnostic {
                range: diagnostic_range,
                code: Some(Union2::B("packing".into())),
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
fn no_diagnostics() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct (field i8)))
  (func (param (ref $s)) (result i32)
    local.get 0
    struct.get $s 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(super::create_params(uri, 5, 19, 5, 19));
    assert!(response.is_none());
}

#[test]
fn unrelated_range() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct (field i8)))
  (func (param (ref $s)) (result i32)
    local.get 0
    struct.get $s 0)
  (func (param (ref $s)) (result i32)
    local.get 0
    struct.get $s 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 5,
                character: 19,
            },
            end: Position {
                line: 5,
                character: 19,
            },
        },
        Range {
            start: Position {
                line: 8,
                character: 18,
            },
            end: Position {
                line: 8,
                character: 19,
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
  (type $s (struct (field i8)))
  (func (param (ref $s)) (result i32)
    local.get 0
    struct.get $s 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(CodeActionParams {
        text_document: TextDocumentIdentifier { uri },
        range: Range {
            start: Position {
                line: 5,
                character: 19,
            },
            end: Position {
                line: 5,
                character: 19,
            },
        },
        context: CodeActionContext {
            diagnostics: vec![Diagnostic {
                range: Range {
                    start: Position {
                        line: 5,
                        character: 18,
                    },
                    end: Position {
                        line: 5,
                        character: 19,
                    },
                },
                code: Some(Union2::B("undef".into())),
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
fn struct_get() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct (field i8)))
  (func (param (ref $s)) (result i32)
    local.get 0
    struct.get $s 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 5,
                character: 19,
            },
            end: Position {
                line: 5,
                character: 19,
            },
        },
        Range {
            start: Position {
                line: 5,
                character: 18,
            },
            end: Position {
                line: 5,
                character: 19,
            },
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn struct_get_s() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct (field i32)))
  (func (param (ref $s)) (result i32)
    local.get 0
    struct.get_s $s 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 5,
                character: 21,
            },
            end: Position {
                line: 5,
                character: 21,
            },
        },
        Range {
            start: Position {
                line: 5,
                character: 20,
            },
            end: Position {
                line: 5,
                character: 21,
            },
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn struct_get_u() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct (field i32)))
  (func (param (ref $s)) (result i32)
    local.get 0
    struct.get_u $s 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 5,
                character: 21,
            },
            end: Position {
                line: 5,
                character: 21,
            },
        },
        Range {
            start: Position {
                line: 5,
                character: 20,
            },
            end: Position {
                line: 5,
                character: 21,
            },
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn array_get() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (array i8))
  (func (param (ref 0)) (result i32)
    local.get 0
    array.get 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 5,
                character: 15,
            },
            end: Position {
                line: 5,
                character: 15,
            },
        },
        Range {
            start: Position {
                line: 5,
                character: 14,
            },
            end: Position {
                line: 5,
                character: 15,
            },
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn array_get_s() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (array i32))
  (func (param (ref 0)) (result i32)
    local.get 0
    array.get_s 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 5,
                character: 17,
            },
            end: Position {
                line: 5,
                character: 17,
            },
        },
        Range {
            start: Position {
                line: 5,
                character: 16,
            },
            end: Position {
                line: 5,
                character: 17,
            },
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn array_get_u() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (array i32))
  (func (param (ref 0)) (result i32)
    local.get 0
    array.get_u 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range {
            start: Position {
                line: 5,
                character: 17,
            },
            end: Position {
                line: 5,
                character: 17,
            },
        },
        Range {
            start: Position {
                line: 5,
                character: 16,
            },
            end: Position {
                line: 5,
                character: 17,
            },
        },
    ));
    assert_json_snapshot!(response);
}
