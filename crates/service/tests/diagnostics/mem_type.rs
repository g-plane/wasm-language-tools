use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn overflow() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 1000000 1000000)
  (memory 99999999999999999999999 99999999999999999999999))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn less_than() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 2 1))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn overflow_and_less_than() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 99999999999999999999999 99999999999999999999991))
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
  (memory 1 1)
  (memory 2 3))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn invalid_page_size() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 1 2 (pagesize 2)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn page_size_1() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 99999999999999999999999 99999999999999999999991 (pagesize 1)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn page_size_65536() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 100000 110000 (pagesize 65536)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
