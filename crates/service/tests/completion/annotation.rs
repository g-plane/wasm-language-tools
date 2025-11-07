use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn after_at() {
    let uri = "untitled:test".to_string();
    let source = "(@";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 0, 2));
    assert_json_snapshot!(response);
}

#[test]
fn incomplete() {
    let uri = "untitled:test".to_string();
    let source = "(@d";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 0, 3));
    assert_json_snapshot!(response);
}
