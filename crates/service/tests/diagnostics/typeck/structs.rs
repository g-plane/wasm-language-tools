use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn new() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct (field i32 f32)))
  (func (result (ref $s))
    f32.const 0
    i32.const 0
    struct.new 0)
  (func (result (ref $s))
    i32.const 0
    f32.const 0
    struct.new 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn new_default() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct (field i32 f32)))
  (func (result arrayref)
    struct.new_default $s)
  (func (result structref)
    struct.new_default $s))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
