use super::*;
use insta::assert_json_snapshot;
use lspt::Position;
use wat_service::LanguageService;

#[test]
fn global_type_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global )
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
fn global_type_mut_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global ())
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 2,
            character: 13,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn global_type_mut_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global (mut ))
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
fn globals() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (global.get ))
    (global i32)
    (global $global i32)
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
fn globals_following_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (global.get 0))
    (global i32)
    (global $global i32)
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
fn globals_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (global.get $))
    (global i32)
    (global $global i32)
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
fn globals_following_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (global.get $g))
    (global i32)
    (global $global i32)
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
fn in_sequence() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func global.get )
    (global i32)
    (global $global i32)
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
fn export() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global i32)
    (global $global i32)
    (export \"\" (global ))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 4,
            character: 23,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn export_following_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global i32)
    (global $global i32)
    (export \"\" (global 0))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 4,
            character: 24,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn export_following_dollar() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global i32)
    (global $global i32)
    (export \"\" (global $))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 4,
            character: 24,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn export_following_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global i32)
    (global $global i32)
    (export \"\" (global $g))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 4,
            character: 25,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn preferred_type_by_instr() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global $a i32)
  (global $b i64)
  (global $c f32)
  (global $d f64)
  (func (param f64))
  (func
    (f32.add
      (global.get $))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 9,
            character: 18,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn preferred_type_by_call() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global $a i32)
  (global $b i64)
  (global $c f32)
  (global $d f64)
  (func (param f64))
  (func
    (call 0
      (global.get $))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(
        uri,
        Position {
            line: 9,
            character: 18,
        },
    ));
    assert_json_snapshot!(response);
}
