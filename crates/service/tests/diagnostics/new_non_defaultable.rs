use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn structs() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field)))
  (type $s1 (struct (field i32)))
  (type $s2 (struct (field (ref $s1) i32) (field $f (ref $s1))))
  (func
    struct.new_default 0
    drop
    struct.new_default $s1
    drop
    struct.new_default $s2
    drop))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn array() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (array))
  (type $a1 (array (mut i32)))
  (type $a2 (array (ref $a1)))
  (func
    i32.const 0
    array.new_default 0
    drop
    i32.const 0
    array.new_default $a1
    drop
    i32.const 0
    array.new_default $a2
    drop))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
