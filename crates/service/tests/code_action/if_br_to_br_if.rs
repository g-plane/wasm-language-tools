use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn not_if() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(uri, 3, 9, 3, 9));
    assert!(response.is_none());
}

#[test]
fn no_then_instrs() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    if
    end
    (if
      (then))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(uri.clone(), 3, 6, 3, 6));
    assert!(response.is_none());
    let response = service.code_action(create_params(uri, 5, 6, 5, 6));
    assert!(response.is_none());
}

#[test]
fn no_br() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    if
      br_table 0
    end
    (if
      (then
        (br_table 0)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(uri.clone(), 3, 6, 3, 6));
    assert!(response.is_none());
    let response = service.code_action(create_params(uri, 6, 6, 6, 6));
    assert!(response.is_none());
}

#[test]
fn more_than_br() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    if
      br 0
      i32.const 0
    end
    (if
      (then
        (br 0)
        (i32.const 0)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(uri.clone(), 3, 6, 3, 6));
    assert!(response.is_none());
    let response = service.code_action(create_params(uri, 7, 6, 7, 6));
    assert!(response.is_none());
}

#[test]
fn has_else() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    if
      br 0
    else
    end
    (if
      (then
        (br 0))
      (else))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(uri.clone(), 3, 6, 3, 6));
    assert!(response.is_none());
    let response = service.code_action(create_params(uri, 7, 6, 7, 6));
    assert!(response.is_none());
}

#[test]
fn sequence_without_condition() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    if
      br 0
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(uri, 3, 9, 3, 9));
    assert_json_snapshot!(response);
}

#[test]
fn sequence_with_condition() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    i32.const 0
    if
      br 0
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(uri, 4, 9, 4, 9));
    assert_json_snapshot!(response);
}

#[test]
fn folded_without_condition() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (if
      (then
        (br 0)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(uri, 3, 9, 3, 9));
    assert_json_snapshot!(response);
}

#[test]
fn folded_with_single_condition() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (if
      (i32.const 0)
      (then
        (br 0)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(uri, 3, 9, 3, 9));
    assert_json_snapshot!(response);
}

#[test]
fn folded_with_multi_conditions() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (if
      (i32.const 0)
      (i32.const 1)
      (then
        (br 0)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(uri, 3, 9, 3, 9));
    assert_json_snapshot!(response);
}
