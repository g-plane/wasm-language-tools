use super::*;
use insta::assert_json_snapshot;
use lsp_types::Uri;
use wat_service::LanguageService;

#[test]
fn call_defined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func $foo (call $foo)))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn call_undefined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func $foo (call $bar)))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn local_defined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (param $p i32) (local.get 0) (drop)))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn local_undefined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (local.get 0) (drop)))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn type_use_defined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (type $t)) (type $t (func)))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn type_use_undefined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (type $t)))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn global_defined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (global.get $foo)
    (drop))
  (global $foo i32
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn global_undefined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (global.get $bar) (drop)))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn memory_defined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (memory 1) (export \"\" (memory 0)))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn memory_undefined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (export \"\" (memory 0)))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn table_defined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (table $table 0 funcref)
  (func
    (table.size $table)
    (drop)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn table_undefined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (table.size $table)
    (drop)
    (call_indirect (param f32)
      (f32.const 0)
      (i32.const 0))
    (call_indirect 0 (param f32)
      (f32.const 0)
      (i32.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn block() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (block $a
      (block $b
        (block $c
          (br_table $a $b $c $d
           (i32.const 0))
          (br_table 0 1 2 3
            (i32.const 0)))
        (i32.const 1)
        (drop)
        (return))
      (i32.const 1)
      (drop)
      (return))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn export() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (export \"func\" (func 0))
  (func))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}
