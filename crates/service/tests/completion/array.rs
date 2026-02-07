use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn array() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $a (struct))
  (type (array))
  (type $b (array))
  (func
    array.new ))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 14));
    assert_json_snapshot!(response);
}

#[test]
fn array_following_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $a (struct))
  (type (array))
  (type $b (array))
  (func
    array.new 1))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 15));
    assert_json_snapshot!(response);
}

#[test]
fn array_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $a (struct))
  (type (array))
  (type $b (array))
  (func
    array.new $))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 15));
    assert_json_snapshot!(response);
}

#[test]
fn array_following_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $a (struct))
  (type (array))
  (type $b (array))
  (func
    array.new $b))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 16));
    assert_json_snapshot!(response);
}

#[test]
fn array_init_data() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $a (struct))
  (type (array))
  (type $b (array))
  (data $d)
  (func
    (array.init_data )))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 7, 21));
    assert_json_snapshot!(response);
}

#[test]
fn array_new_data() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $a (struct))
  (type (array))
  (type $b (array))
  (data $d)
  (func
    (array.new_data )))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 7, 20));
    assert_json_snapshot!(response);
}

#[test]
fn array_init_elem() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $a (struct))
  (type (array))
  (type $b (array))
  (data $d)
  (func
    (array.init_elem )))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 7, 21));
    assert_json_snapshot!(response);
}

#[test]
fn array_new_elem() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $a (struct))
  (type (array))
  (type $b (array))
  (data $d)
  (func
    (array.new_elem )))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 7, 20));
    assert_json_snapshot!(response);
}
