use insta::assert_json_snapshot;
use lspt::{
    CodeActionContext, CodeActionKind, CodeActionParams, Diagnostic, Position, Range,
    TextDocumentIdentifier, Union2,
};
use wat_service::LanguageService;

fn create_params(uri: String, line: u32, col: u32, diagnostic: Diagnostic) -> CodeActionParams {
    CodeActionParams {
        text_document: TextDocumentIdentifier { uri },
        range: Range {
            start: Position {
                line,
                character: col,
            },
            end: Position {
                line,
                character: col,
            },
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

fn create_data(start: u32, end: u32, types: Vec<String>) -> Option<serde_json::Value> {
    serde_json::to_value((start, end, types)).ok()
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
            data: create_data(10, 35, vec![]),
            ..Default::default()
        },
    ));
    assert!(response.is_none());
}

#[test]
fn unrelated_range() {
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
            code: Some(Union2::B("type-check".into())),
            data: create_data(0, 0, vec![]),
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
            data: create_data(11, 33, vec!["i32".into()]),
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
            data: create_data(11, 36, vec!["i32".into()]),
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
            data: create_data(11, 45, vec!["i32".into()]),
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
            data: create_data(11, 54, vec!["i32".into()]),
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
            data: create_data(11, 63, vec!["i32".into()]),
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
            data: create_data(11, 72, vec!["i32".into()]),
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
            data: create_data(11, 69, vec!["i32".into()]),
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
            data: create_data(21, 46, vec!["i32".into()]),
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
            data: create_data(21, 49, vec!["i32".into()]),
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
            data: create_data(21, 58, vec!["i32".into()]),
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
            data: create_data(21, 45, vec!["i32".into()]),
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
            data: create_data(21, 60, vec!["i32".into()]),
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
            data: create_data(21, 50, vec!["i32".into()]),
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
            data: create_data(11, 36, vec!["i32".into(), "(ref null any)".into()]),
            ..Default::default()
        },
    ));
    assert_json_snapshot!(response);
}
