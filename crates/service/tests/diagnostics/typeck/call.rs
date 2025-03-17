use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn call_type_mismatch() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $f1 (param f32))
    (func (call $f1 (i32.const 0)))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn return_call_mismatch() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $f1 (param f32))
    (func (return_call $f1 (i32.const 0)))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
