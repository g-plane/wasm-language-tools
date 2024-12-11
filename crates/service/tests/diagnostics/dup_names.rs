use super::*;
use insta::assert_json_snapshot;
use lsp_types::Uri;
use wat_service::LanguageService;

#[test]
fn same_kind() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func $f (param i32))
  (func $f (result f32)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    allow_unused(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn different_kinds() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (global $f i32)
  (func $f))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    allow_unused(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn param_and_local() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (param $a i32) (local $a f32)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    allow_unused(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn different_scopes() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (param $p i32))
  (func (param $p i32)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    allow_unused(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn ref_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func $fn
    (call $fn)
    (call $fn)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    allow_unused(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn blocks() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (block $b
      (block $a
        (block $b
          (block $c
            (block $b)))))
    (block
      (block $b))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    allow_unused(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn exports() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = r#"
(module
  (func (export "func"))
  (export "func" (func 0)))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    allow_unused(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
