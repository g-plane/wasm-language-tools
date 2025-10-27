use insta::assert_json_snapshot;
use lspt::{CodeLensParams, TextDocumentIdentifier};
use wat_service::LanguageService;

fn create_params(uri: String) -> CodeLensParams {
    CodeLensParams {
        text_document: TextDocumentIdentifier { uri },
        work_done_token: None,
        partial_result_token: None,
    }
}

#[test]
fn list() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global $g i32)
  (func $f (param $p i32) (param f32 i64) (local (ref 0))
    local.get $p)
  (memory 1 2)
  (table 1 2 funcref)
  (func
    (call $f))
  (type (struct (field (mut i32))))
  (tag))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_lens(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn zero_references() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let code_lenses = service.code_lens(create_params(uri)).unwrap();
    let response = service.code_lens_resolve(code_lenses[0].clone());
    assert_json_snapshot!(response);
}

#[test]
fn one_reference() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func)
  (func
    (call 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let code_lenses = service.code_lens(create_params(uri)).unwrap();
    let response = service.code_lens_resolve(code_lenses[0].clone());
    assert_json_snapshot!(response);
}

#[test]
fn more_references() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f)
  (func
    (call 0)
    (call $f)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let code_lenses = service.code_lens(create_params(uri)).unwrap();
    let response = service.code_lens_resolve(code_lenses[0].clone());
    assert_json_snapshot!(response);
}
