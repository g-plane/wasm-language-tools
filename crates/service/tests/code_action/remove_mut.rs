use super::*;
use insta::assert_json_snapshot;
use lsp_types::{Diagnostic, NumberOrString, Position, Range, Uri};
use wat_service::LanguageService;

fn create_params(uri: Uri, range: Range, token_range: Range) -> CodeActionParams {
    CodeActionParams {
        text_document: TextDocumentIdentifier { uri },
        range,
        context: CodeActionContext {
            diagnostics: vec![Diagnostic {
                range: token_range,
                code: Some(NumberOrString::String("needless-mut".into())),
                message: "".into(),
                ..Default::default()
            }],
            only: None,
            trigger_kind: None,
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
    }
}

#[test]
fn no_mut() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (global $a i32))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(super::create_params(
        uri,
        Range::new(Position::new(2, 15), Position::new(2, 15)),
    ));
    assert!(response.is_none());
}

#[test]
fn no_diagnostics() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (global $a (mut i32)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(super::create_params(
        uri,
        Range::new(Position::new(2, 15), Position::new(2, 15)),
    ));
    assert!(response.is_none());
}

#[test]
fn unrelated_range() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (global $a (mut i32))
  (global $b (mut i32)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range::new(Position::new(2, 15), Position::new(2, 15)),
        Range::new(Position::new(3, 14), Position::new(3, 17)),
    ));
    assert!(response.is_none());
}

#[test]
fn unrelated_diagnostic() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (global $a (mut i32))
  (global $b (mut i32)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(CodeActionParams {
        text_document: TextDocumentIdentifier { uri },
        range: Range::new(Position::new(2, 15), Position::new(2, 15)),
        context: CodeActionContext {
            diagnostics: vec![Diagnostic {
                range: Range::new(Position::new(2, 15), Position::new(2, 15)),
                code: Some(NumberOrString::String("global-mut".into())),
                message: "".into(),
                ..Default::default()
            }],
            only: None,
            trigger_kind: None,
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
    });
    assert!(response.is_none());
}

#[test]
fn simple() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (global $a (mut i32)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range::new(Position::new(2, 15), Position::new(2, 15)),
        Range::new(Position::new(2, 14), Position::new(2, 17)),
    ));
    assert_json_snapshot!(response);
}

#[test]
fn missing_r_paren() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (global $a (mut i32
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range::new(Position::new(2, 15), Position::new(2, 15)),
        Range::new(Position::new(2, 14), Position::new(2, 17)),
    ));
    assert_json_snapshot!(response);
}

#[test]
fn with_comments() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (global $a ((;a;) mut(;b;) i32)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range::new(Position::new(2, 20), Position::new(2, 20)),
        Range::new(Position::new(2, 20), Position::new(2, 23)),
    ));
    assert_json_snapshot!(response);
}
