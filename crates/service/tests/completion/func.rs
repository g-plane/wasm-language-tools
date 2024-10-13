use super::*;
use insta::assert_json_snapshot;
use lsp_types::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn module_field_start() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (start )
    (func $func)
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 11)));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_start_following_dollar() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (start $)
    (func $func)
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 11)));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_start_incomplete() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (start $f)
    (func $func)
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 12)));
    assert_json_snapshot!(response);
}

#[test]
fn call() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (call ))
    (func $func)
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 16)));
    assert_json_snapshot!(response);
}

#[test]
fn call_named() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $func (call ))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 22)));
    assert_json_snapshot!(response);
}

#[test]
fn call_named_following_dollar() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $func (call $))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 23)));
    assert_json_snapshot!(response);
}

#[test]
fn call_named_incomplete() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $func (call $f))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 24)));
    assert_json_snapshot!(response);
}

#[test]
fn ref_func() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (ref.func ))
    (func $func)
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 20)));
    assert_json_snapshot!(response);
}
