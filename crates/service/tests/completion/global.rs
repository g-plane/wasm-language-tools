use super::*;
use insta::assert_json_snapshot;
use lsp_types::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn global_type_type() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (global )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 12)));
    assert_json_snapshot!(response);
}

#[test]
fn global_type_mut_keyword() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (global ())
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 13)));
    assert_json_snapshot!(response);
}

#[test]
fn global_type_mut_type() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (global (mut ))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 17)));
    assert_json_snapshot!(response);
}

#[test]
fn globals() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (global.get ))
    (global i32)
    (global $global i32)
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 22)));
    assert_json_snapshot!(response);
}

#[test]
fn globals_following_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (global.get 1))
    (global i32)
    (global $global i32)
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 23)));
    assert_json_snapshot!(response);
}

#[test]
fn globals_following_dollar() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (global.get $))
    (global i32)
    (global $global i32)
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 23)));
    assert_json_snapshot!(response);
}

#[test]
fn globals_following_ident() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (global.get $g))
    (global i32)
    (global $global i32)
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 24)));
    assert_json_snapshot!(response);
}
