use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn new() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $vec (array f32))
  (global (ref $vec)
    (array.new $vec
      (f32.const 1)
      (i32.const 3)))
  (global (ref $vec)
    (array.new $vec
      (i32.const 1)
      (f32.const 3))))
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
  (type $vec (array f32))
  (global (ref $vec)
    (array.new_default $vec
      (i32.const 3))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn new_fixed() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $vec (array f32))
  (global (ref $vec)
    (array.new_fixed $vec 1
      (f32.const 1)
      (f32.const 2)))
  (global (ref $vec)
    (array.new_fixed $vec 2
      (f32.const 1)
      (f32.const 2)))
  (global (ref $vec)
    (array.new_fixed $vec 3
      (f32.const 1)
      (f32.const 2))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
