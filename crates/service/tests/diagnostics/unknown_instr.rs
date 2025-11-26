use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn with_fuzzy() {
    let uri = "untitled:test".to_string();
    let source = "(module (func (log)))";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
