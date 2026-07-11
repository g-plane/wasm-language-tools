use insta::assert_json_snapshot;
use lspt::{
    CodeActionContext, CodeActionKind, CodeActionParams, Diagnostic, NumberOrString, Position, Range,
    StringOrMarkupContent, TextDocumentIdentifier,
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

fn create_diagnostic<const N: usize>(line: u32, col: u32, data: [&'static str; N]) -> Diagnostic {
    Diagnostic {
        range: Range {
            start: Position { line, character: col },
            end: Position {
                line,
                character: col + 1,
            },
        },
        severity: None,
        code: Some(NumberOrString::String("type-check".into())),
        code_description: None,
        source: None,
        message: StringOrMarkupContent::String("".into()),
        tags: None,
        related_information: None,
        data: serde_json::to_value(data.as_slice()).ok(),
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
            range: Range {
                start: Position { line: 3, character: 15 },
                end: Position { line: 3, character: 16 },
            },
            severity: None,
            code: Some(NumberOrString::String("typeck".into())),
            code_description: None,
            source: None,
            message: StringOrMarkupContent::String("".into()),
            tags: None,
            related_information: None,
            data: serde_json::to_value::<Vec<String>>(vec![]).ok(),
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
    let response = service.code_action(create_params(uri, 3, 15, create_diagnostic(3, 15, ["i32"])));
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
    let response = service.code_action(create_params(uri, 3, 15, create_diagnostic(3, 15, ["i32"])));
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
    let response = service.code_action(create_params(uri, 3, 15, create_diagnostic(3, 15, ["i32"])));
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
    let response = service.code_action(create_params(uri, 3, 15, create_diagnostic(3, 15, ["i32"])));
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
    let response = service.code_action(create_params(uri, 3, 15, create_diagnostic(3, 15, ["i32"])));
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
    let response = service.code_action(create_params(uri, 3, 15, create_diagnostic(3, 15, ["i32"])));
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
    let response = service.code_action(create_params(uri, 3, 15, create_diagnostic(3, 15, ["i32"])));
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
    let response = service.code_action(create_params(uri, 4, 17, create_diagnostic(4, 17, ["i32"])));
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
    let response = service.code_action(create_params(uri, 4, 17, create_diagnostic(4, 17, ["i32"])));
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
    let response = service.code_action(create_params(uri, 4, 17, create_diagnostic(4, 17, ["i32"])));
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
    let response = service.code_action(create_params(uri, 4, 17, create_diagnostic(4, 17, ["i32"])));
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
    let response = service.code_action(create_params(uri, 4, 17, create_diagnostic(5, 21, ["i32"])));
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
    let response = service.code_action(create_params(uri, 4, 17, create_diagnostic(5, 21, ["i32"])));
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
    let response = service.code_action(create_params(uri, 4, 17, create_diagnostic(5, 21, ["i32"])));
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
    let response = service.code_action(create_params(uri, 4, 17, create_diagnostic(4, 17, ["i32"])));
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
        create_diagnostic(3, 15, ["i32", "(ref null any)"]),
    ));
    assert_json_snapshot!(response);
}
