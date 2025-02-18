use super::*;
use insta::assert_json_snapshot;
use lspt::Position;
use wat_service::LanguageService;

#[test]
fn module_field_start() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
    (import "" "" (func))
    (start )
    (func $func)
)
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 3,
            character: 11,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_start_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (start $)
    (func $func)
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
fn module_field_start_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (start $f)
    (func $func)
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 12,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn call() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (call ))
    (func $func)
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
fn call_named() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $func (call ))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 22,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn call_named_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $func (call $))
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
fn call_named_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $func (call $f))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 24,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn call_in_sequence() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func call )
    (func $func)
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
fn ref_func() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (ref.func ))
    (func $func)
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
fn export_desc_func() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (export (func ))
    (func $func)
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
fn export_desc_func_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (export (func $))
    (func $func)
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
fn export_desc_func_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (export (func $f))
    (func $func)
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
fn doc_comment() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (call ))
    ;;; doc comment
    (func $func)
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
fn label_details() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (call ))
    (func $f1 (param i32) (param $p1 i64) (result i32 i64) (result f32))
    (func $f2 (param i32 i64) (result i32) (result f32))
    (func $f3 (param i32) (param i64))
    (func $f4 (result i32) (result f32))
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
