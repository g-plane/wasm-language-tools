use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn non_func() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $ct1 (cont $ct1))
  (rec
    (type $s0 (struct (field (ref 0) (ref 1) (ref $s0) (ref $s1))))
    (type $s1 (struct (field (ref 0) (ref 1) (ref $s0) (ref $s1)))))
  (type $ct2 (cont $s0)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
