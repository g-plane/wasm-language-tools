use super::*;
use insta::assert_json_snapshot;
use lspt::Position;
use wat_service::LanguageService;

#[test]
fn types() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param ))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 17,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn types_following_incomplete_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param $))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 18,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn types_following_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param $p))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 19,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn types_after_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param $p ))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 20,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn types_multiple_types() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param i32 ))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 21,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn types_incomplete_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param i))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 18,
        },
    ));
    assert_json_snapshot!(response);
}
