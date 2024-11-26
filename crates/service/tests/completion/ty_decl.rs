use super::*;
use insta::assert_json_snapshot;
use lsp_types::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn func_keyword() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (type ())
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 11)));
    assert_json_snapshot!(response);
}

#[test]
fn param_and_result_after_paren() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (type (func ()))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 17)));
    assert_json_snapshot!(response);
}

#[test]
fn param_and_result_incomplete() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (type (func (p)))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 18)));
    assert_json_snapshot!(response);
}
