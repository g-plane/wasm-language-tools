use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn types() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (result ))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 18));
    assert_json_snapshot!(response);
}

#[test]
fn types_multiple_types() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (result i32 ))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 22));
    assert_json_snapshot!(response);
}

#[test]
fn types_incomplete_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (result i))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn sequence_select_following_paren() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func select ())
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 18));
    assert_json_snapshot!(response);
}

#[test]
fn folded_select_following_paren() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (select ()))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn folded_select_incomplete_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (select (re)))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 21));
    assert_json_snapshot!(response);
}

#[test]
fn after_paren() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 17));
    assert_json_snapshot!(response);
}

#[test]
fn ref_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result (ref )))
  (type))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 21));
    assert_json_snapshot!(response);
}
