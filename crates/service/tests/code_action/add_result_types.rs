use insta::assert_json_snapshot;
use lspt::{
    CodeActionContext, CodeActionKind, CodeActionParams, Diagnostic, Position, Range, TextDocumentIdentifier, Union2,
};
use wat_service::LanguageService;

fn create_params(uri: String, line: u32, col: u32, diagnostic: Diagnostic) -> CodeActionParams {
    CodeActionParams {
        text_document: TextDocumentIdentifier { uri },
        range: Range {
            start: Position { line, character: col },
            end: Position { line, character: col },
        },
        context: CodeActionContext {
            diagnostics: vec![diagnostic],
            only: Some(vec![CodeActionKind::QuickFix]),
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
  (func $f
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(super::create_params(uri, 3, 17, 3, 17));
    assert!(response.is_none());
}

#[test]
fn unrelated_diagnostic_code() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        3,
        17,
        Diagnostic {
            code: Some(Union2::B("typeck".into())),
            range: Range {
                start: Position { line: 3, character: 15 },
                end: Position { line: 3, character: 16 },
            },
            data: serde_json::to_value::<Vec<String>>(vec![]).ok(),
            ..Default::default()
        },
    ));
    assert!(response.is_none());
}

#[test]
fn after_func_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        3,
        15,
        Diagnostic {
            code: Some(Union2::B("type-check".into())),
            range: Range {
                start: Position { line: 3, character: 15 },
                end: Position { line: 3, character: 16 },
            },
            data: serde_json::to_value(vec!["i32".to_string()]).ok(),
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn after_func_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        3,
        15,
        Diagnostic {
            code: Some(Union2::B("type-check".into())),
            range: Range {
                start: Position { line: 3, character: 15 },
                end: Position { line: 3, character: 16 },
            },
            data: serde_json::to_value(vec!["i32".to_string()]).ok(),
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn after_func_export() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f (export)
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        3,
        15,
        Diagnostic {
            code: Some(Union2::B("type-check".into())),
            range: Range {
                start: Position { line: 3, character: 15 },
                end: Position { line: 3, character: 16 },
            },
            data: serde_json::to_value(vec!["i32".to_string()]).ok(),
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn after_func_import() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f (export) (import)
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        3,
        15,
        Diagnostic {
            code: Some(Union2::B("type-check".into())),
            range: Range {
                start: Position { line: 3, character: 15 },
                end: Position { line: 3, character: 16 },
            },
            data: serde_json::to_value(vec!["i32".to_string()]).ok(),
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn after_func_type_use() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f (export) (import) (type 0)
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        3,
        15,
        Diagnostic {
            code: Some(Union2::B("type-check".into())),
            range: Range {
                start: Position { line: 3, character: 15 },
                end: Position { line: 3, character: 16 },
            },
            data: serde_json::to_value(vec!["i32".to_string()]).ok(),
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn after_func_params() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (export) (import) (type 0) (param i32)
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        3,
        15,
        Diagnostic {
            code: Some(Union2::B("type-check".into())),
            range: Range {
                start: Position { line: 3, character: 15 },
                end: Position { line: 3, character: 16 },
            },
            data: serde_json::to_value(vec!["i32".to_string()]).ok(),
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn before_func_locals() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f (type 0) (param i32) (local i32)
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        3,
        15,
        Diagnostic {
            code: Some(Union2::B("type-check".into())),
            range: Range {
                start: Position { line: 3, character: 15 },
                end: Position { line: 3, character: 16 },
            },
            data: serde_json::to_value(vec!["i32".to_string()]).ok(),
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn after_block_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block
      i32.const 0)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        4,
        17,
        Diagnostic {
            code: Some(Union2::B("type-check".into())),
            range: Range {
                start: Position { line: 4, character: 17 },
                end: Position { line: 4, character: 18 },
            },
            data: serde_json::to_value(vec!["i32".to_string()]).ok(),
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn after_block_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block $b
      i32.const 0)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        4,
        17,
        Diagnostic {
            code: Some(Union2::B("type-check".into())),
            range: Range {
                start: Position { line: 4, character: 17 },
                end: Position { line: 4, character: 18 },
            },
            data: serde_json::to_value(vec!["i32".to_string()]).ok(),
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn after_block_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block $b (type 0)
      i32.const 0)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        4,
        17,
        Diagnostic {
            code: Some(Union2::B("type-check".into())),
            range: Range {
                start: Position { line: 4, character: 17 },
                end: Position { line: 4, character: 18 },
            },
            data: serde_json::to_value(vec!["i32".to_string()]).ok(),
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn after_loop_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (loop
      i32.const 0)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        4,
        17,
        Diagnostic {
            code: Some(Union2::B("type-check".into())),
            range: Range {
                start: Position { line: 4, character: 17 },
                end: Position { line: 4, character: 18 },
            },
            data: serde_json::to_value(vec!["i32".to_string()]).ok(),
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn after_if_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (if
      (then
        (i32.const 0)))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        4,
        17,
        Diagnostic {
            code: Some(Union2::B("type-check".into())),
            range: Range {
                start: Position { line: 5, character: 21 },
                end: Position { line: 5, character: 22 },
            },
            data: serde_json::to_value(vec!["i32".to_string()]).ok(),
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn after_if_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (if $cond
      (then
        (i32.const 0)))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        4,
        17,
        Diagnostic {
            code: Some(Union2::B("type-check".into())),
            range: Range {
                start: Position { line: 5, character: 21 },
                end: Position { line: 5, character: 22 },
            },
            data: serde_json::to_value(vec!["i32".to_string()]).ok(),
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn after_if_type_use() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (if (param)
      (then
        (i32.const 0)))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        4,
        17,
        Diagnostic {
            code: Some(Union2::B("type-check".into())),
            range: Range {
                start: Position { line: 5, character: 21 },
                end: Position { line: 5, character: 22 },
            },
            data: serde_json::to_value(vec!["i32".to_string()]).ok(),
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn after_try_table_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (try_table
      i32.const 0)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        4,
        17,
        Diagnostic {
            code: Some(Union2::B("type-check".into())),
            range: Range {
                start: Position { line: 4, character: 17 },
                end: Position { line: 4, character: 18 },
            },
            data: serde_json::to_value(vec!["i32".to_string()]).ok(),
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn multi_types() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(
        uri,
        3,
        15,
        Diagnostic {
            code: Some(Union2::B("type-check".into())),
            range: Range {
                start: Position { line: 3, character: 15 },
                end: Position { line: 3, character: 16 },
            },
            data: serde_json::to_value(vec!["i32".to_string(), "(ref null any)".to_string()]).ok(),
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}
