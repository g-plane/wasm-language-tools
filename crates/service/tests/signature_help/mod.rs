use insta::assert_json_snapshot;
use lsp_types::{
    Position, SignatureHelpParams, TextDocumentIdentifier, TextDocumentPositionParams, Uri,
};
use wat_service::LanguageService;

fn create_params(uri: Uri, position: Position) -> SignatureHelpParams {
    SignatureHelpParams {
        context: None,
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position,
        },
        work_done_progress_params: Default::default(),
    }
}

#[test]
fn first_param() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func $func (param i32) (param i32) (result i32)
    (call $func ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(uri, Position::new(3, 17)));
    assert_json_snapshot!(response);
}

#[test]
fn first_param_end() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func $func (param i32) (param i32) (result i32)
    (call $func (local.get 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(uri, Position::new(3, 29)));
    assert_json_snapshot!(response);
}

#[test]
fn first_param_before_others() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func $func (param i32) (param i32) (result i32)
    (call $func () (local.get 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(uri, Position::new(3, 17)));
    assert_json_snapshot!(response);
}

#[test]
fn non_first_param() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func $func (param i32) (param i32) (result i32)
    (call $func (local.get 0) ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(uri, Position::new(3, 31)));
    assert_json_snapshot!(response);
}

#[test]
fn middle() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func $func (param i32) (param i32) (param i32)
    (call $func (local.get 0) ( (local.get 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(uri, Position::new(3, 31)));
    assert_json_snapshot!(response);
}

#[test]
fn no_params_no_results() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func $func
    (call $func ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(uri, Position::new(3, 17)));
    assert_json_snapshot!(response);
}

#[test]
fn no_params() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func $func (result i32) (result f32 f64)
    (call $func ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(uri, Position::new(3, 17)));
    assert_json_snapshot!(response);
}

#[test]
fn no_results() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func $func (param $p i32) (param f32 f64)
    (call $func ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(uri, Position::new(3, 17)));
    assert_json_snapshot!(response);
}

#[test]
fn doc_comment() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  ;;; doc comment
  (func $func (param i32) (param i32) (result i32)
    (call $func ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(uri, Position::new(4, 17)));
    assert_json_snapshot!(response);
}

#[test]
fn call_indirect_type_use() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (table 0 funcref)
  (type (func (param f32) (result f64)))
  (func (result f64)
    (call_indirect 0 (type 0) ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(uri, Position::new(5, 31)));
    assert_json_snapshot!(response);
}

#[test]
fn call_indirect_inline_func_type() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (table 0 funcref)
  (func (result f64)
    (call_indirect 0 (param f32) (result f64) ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.signature_help(create_params(uri, Position::new(4, 47)));
    assert_json_snapshot!(response);
}
