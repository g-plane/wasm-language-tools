use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn null() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s)
  (func (ref.null )))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 3, 18));
    assert_json_snapshot!(response);
}
