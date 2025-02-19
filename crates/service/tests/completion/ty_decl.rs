use super::*;
use insta::assert_json_snapshot;
use lspt::Position;
use wat_service::LanguageService;

#[test]
fn keywords() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type ())
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 11,
        },
    ));
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
fn param_and_result_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type (func (p)))
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
fn func_type_in_sub_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type (sub (func (p))))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 23,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn sub_type_without_paren() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type (sub ))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 15,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn final_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type (sub f))
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
fn sub_type_with_paren() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type (sub ()))
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
