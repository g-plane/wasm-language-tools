use super::*;
use insta::assert_json_snapshot;
use lspt::{Position};
use wat_service::LanguageService;

#[test]
fn func_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type ())
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position { line: 2, character: 11 }));
    assert_json_snapshot!(response);
}

#[test]
fn param_and_result_after_paren() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type (func ()))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position { line: 2, character: 17 }));
    assert_json_snapshot!(response);
}

#[test]
fn param_and_result_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type (func (p)))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position { line: 2, character: 18 }));
    assert_json_snapshot!(response);
}
