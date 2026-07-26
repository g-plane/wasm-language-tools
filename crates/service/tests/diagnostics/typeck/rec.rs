use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn implicit_func_1() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (rec (type $s (struct)))
  (rec (type $t (func (param (ref $s)))))
  (func $f (param (ref $s))) ;; okay, type is equivalent to $t
  (global (ref $t)
    (ref.func $f)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn implicit_func_2() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (rec
    (type $s (struct))
    (type $t (func (param (ref $s)))))
  (func $f (param (ref $s))) ;; type is not equivalent to $t
  (global (ref $t)
    (ref.func $f)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn implicit_func_3() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (rec
    (type (struct))
    (type $t (func)))
  (func $f) ;; type is not equivalent to $t
  (global (ref $t)
    (ref.func $f)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
