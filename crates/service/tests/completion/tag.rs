use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn module_field() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (tag ())
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 8));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (tag (p))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 9));
    assert_json_snapshot!(response);
}

#[test]
fn extern_type() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (import "" "" (tag ()))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 22));
    assert_json_snapshot!(response);
}

#[test]
fn extern_type_incomplete() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (import "" "" (tag (p)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 2, 23));
    assert_json_snapshot!(response);
}

#[test]
fn catch() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (tag)
  (tag $e)
  (func
    try_table (catch )
    end))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 21));
    assert_json_snapshot!(response);
}

#[test]
fn catch_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (tag)
  (tag $e)
  (func
    try_table (catch $)
    end))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 22));
    assert_json_snapshot!(response);
}

#[test]
fn catch_before_label() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (tag)
  (tag $e)
  (func
    try_table (catch  0)
    end))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 21));
    assert_json_snapshot!(response);
}

#[test]
fn catch_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (tag)
  (tag $e)
  (func
    try_table (catch_ref )
    end))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 5, 25));
    assert_json_snapshot!(response);
}

#[test]
fn throw() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (tag (param i32 f32) (result i64))
  (tag $e (param i32) (result f32))
  (func
    try_table
      throw ;;
    end))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 12));
    assert_json_snapshot!(response);
}

#[test]
fn throw_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (tag)
  (tag $e)
  (func
    try_table
      (throw $)
    end))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 6, 14));
    assert_json_snapshot!(response);
}

#[test]
fn extern_idx() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (tag)
  (tag $e)
  (export "" (tag )))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 4, 18));
    assert_json_snapshot!(response);
}

#[test]
fn extern_idx_following_ident() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (tag)
  (tag $e)
  (export "" (tag $e)))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 4, 20));
    assert_json_snapshot!(response);
}

#[test]
fn deprecated() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (@deprecated)
  (tag)
  (@deprecated "this is deprecated")
  (tag $e)
  (func
    try_table (catch )
    end))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.completion(create_params(uri, 7, 21));
    assert_json_snapshot!(response);
}
