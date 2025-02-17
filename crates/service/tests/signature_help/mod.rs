use insta::assert_json_snapshot;
use lspt::{Position, SignatureHelpParams, TextDocumentIdentifier};
use wat_service::LanguageService;

fn create_params(uri: String, position: Position) -> SignatureHelpParams {
    SignatureHelpParams {
        context: None,
        text_document: TextDocumentIdentifier { uri },
        position,
        work_done_token: Default::default(),
    }
}

#[test]
fn first_param() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $func (param i32) (param i32) (result i32)
    (call $func ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(
        uri,
        Position {
            line: 3,
            character: 17,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn first_param_end() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $func (param i32) (param i32) (result i32)
    (call $func (local.get 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(
        uri,
        Position {
            line: 3,
            character: 29,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn first_param_before_others() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $func (param i32) (param i32) (result i32)
    (call $func () (local.get 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(
        uri,
        Position {
            line: 3,
            character: 17,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn non_first_param() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $func (param i32) (param i32) (result i32)
    (call $func (local.get 0) ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(
        uri,
        Position {
            line: 3,
            character: 31,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn middle() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $func (param i32) (param i32) (param i32)
    (call $func (local.get 0) ( (local.get 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(
        uri,
        Position {
            line: 3,
            character: 31,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn no_params_no_results() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $func
    (call $func ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(
        uri,
        Position {
            line: 3,
            character: 17,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn no_params() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $func (result i32) (result f32 f64)
    (call $func ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(
        uri,
        Position {
            line: 3,
            character: 17,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn no_results() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $func (param $p i32) (param f32 f64)
    (call $func ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(
        uri,
        Position {
            line: 3,
            character: 17,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn doc_comment() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  ;;; doc comment
  (func $func (param i32) (param i32) (result i32)
    (call $func ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(
        uri,
        Position {
            line: 4,
            character: 17,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn call_indirect_type_use() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table 0 funcref)
  (type (func (param f32) (result f64)))
  (func (result f64)
    (call_indirect 0 (type 0) ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(
        uri,
        Position {
            line: 5,
            character: 31,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn call_indirect_inline_func_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table 0 funcref)
  (func (result f64)
    (call_indirect 0 (param f32) (result f64) ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(
        uri,
        Position {
            line: 4,
            character: 47,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn return_call() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $func (param i32) (param i32) (result i32)
    (return_call $func ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(
        uri,
        Position {
            line: 3,
            character: 24,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn return_call_indirect() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table 0 funcref)
  (type (func (param f32) (result f64)))
  (func (result f64)
    (return_call_indirect 0 (type 0) ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(
        uri,
        Position {
            line: 5,
            character: 38,
        },
    ));
    assert_json_snapshot!(response);
}
