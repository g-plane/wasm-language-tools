use super::*;
use insta::assert_json_snapshot;
use lspt::Position;
use wat_service::LanguageService;

#[test]
fn func_types() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (type ))
    (type (func))
    (type $type (func))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 16,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn func_types_following_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (type 0))
    (type (func))
    (type $type (func))
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
fn func_types_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (type $))
    (type (func))
    (type $type (func))
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
fn func_types_following_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (type $t))
    (type (func))
    (type $type (func))
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
