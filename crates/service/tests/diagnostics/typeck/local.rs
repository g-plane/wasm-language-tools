use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn get() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (result i32) (local i64)
        (i32.add (local.get 0) (i32.const 1))
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn set() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (local $l i32)
    f32.const 0
    local.set $l))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn tee() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (local $l i32)
    f32.const 0
    local.tee $l))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
