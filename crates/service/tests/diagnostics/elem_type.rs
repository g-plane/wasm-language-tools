use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn invalid() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct))
  (table 0 funcref)
  (elem (table 0)
    (i32.const 0) (ref null $s)
    (item
      ref.null $s)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn valid() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $f (func))
  (table 0 funcref)
  (elem (table 0)
    (i32.const 0) (ref null $f)
    (item
      ref.null $f)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
