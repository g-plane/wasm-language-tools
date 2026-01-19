use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn after_at() {
    let uri = "untitled:test".to_string();
    let source = "(@";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 0, 2));
    assert_json_snapshot!(response);
}

#[test]
fn incomplete() {
    let uri = "untitled:test".to_string();
    let source = "(@d";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 0, 3));
    assert_json_snapshot!(response);
}

#[test]
fn compilation_priority() {
    let uri = "untitled:test".to_string();
    let source = "(@metadata.code.compilation_priority (";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 0, 38));
    assert_json_snapshot!(response);
}

#[test]
fn compilation_priority_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "(@metadata.code.compilation_priority (c";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 0, 39));
    assert_json_snapshot!(response);
}

#[test]
fn instr_freq() {
    let uri = "untitled:test".to_string();
    let source = "(@metadata.code.instr_freq (";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 0, 28));
    assert_json_snapshot!(response);
}

#[test]
fn instr_freq_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "(@metadata.code.instr_freq (f";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 0, 29));
    assert_json_snapshot!(response);
}

#[test]
fn call_targets() {
    let uri = "untitled:test".to_string();
    let source = "(@metadata.code.call_targets (";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 0, 30));
    assert_json_snapshot!(response);
}

#[test]
fn call_targets_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "(@metadata.code.call_targets (t";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 0, 31));
    assert_json_snapshot!(response);
}
