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

#[test]
fn field_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (struct ())))
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
fn field_keyword_after_fields() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (struct (field) ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 28,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn field_keyword_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (struct (f))))
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
fn storage_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (struct (field ))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 26,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn storage_type_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (struct (field i))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 27,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn field_type_with_paren() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (struct (field ()))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 27,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn field_type_mut_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (struct (field (m)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 28,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn storage_type_after_mut() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (struct (field (mut )))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 31,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn ref_type_in_field_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (struct (field (r)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 28,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn ref_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (struct (field (ref )))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 31,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn ref_type_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (struct (field (ref n)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 32,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn heap_type_after_null() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (struct (field (ref null )))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 36,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn heap_type_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (struct (field (ref null i)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 37,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn mut_keyword_then_paren() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (struct (field (mut ())))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 32,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn ref_type_after_mut() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (struct (field (mut (ref ))))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 36,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn ref_type_after_mut_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (struct (field (mut (ref n))))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 37,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn array_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (array )))
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
fn storage_type_in_array_type_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (array i)))
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
fn array_type_with_paren() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (array ())))
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
fn field_type_mut_in_array_type_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (struct (field (m)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 28,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn ref_type_in_array_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (array (ref ))))
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
fn super_type_candidates() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $a (struct))
  (type $b (array))
  (type $c (func))
  (type (sub )))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 5,
            character: 13,
        },
    ));
    assert_json_snapshot!(response);
}
