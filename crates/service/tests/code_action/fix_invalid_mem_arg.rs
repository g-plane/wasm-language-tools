use super::*;
use insta::assert_json_snapshot;
use lsp_types::{Position, Range, Uri};
use wat_service::LanguageService;

#[test]
fn leading_whitespace() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (i32.load align =1))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range::new(Position::new(2, 25), Position::new(2, 25)),
    ));
    assert_json_snapshot!(response);
}

#[test]
fn trailing_whitespace() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (i32.load align= 1))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range::new(Position::new(2, 26), Position::new(2, 26)),
    ));
    assert_json_snapshot!(response);
}

#[test]
fn around_whitespaces() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (i32.load align = 1))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.code_action(create_params(
        uri,
        Range::new(Position::new(2, 25), Position::new(2, 25)),
    ));
    assert_json_snapshot!(response);
}
