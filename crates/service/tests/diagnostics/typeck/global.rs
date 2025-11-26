use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn get() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global f32 f32.const 0)
    (func (result i32)
        (i32.add (global.get 0) (i32.const 1))
    )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn set() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global (mut i32) i32.const 0)
  (func
    f32.const 0
    global.set 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn results_incorrect() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global i32)
  (global i32
    f32.const 0)
  (global i32
    i32.const 0
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn results_correct() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (global i32
    i32.const 0)
  (global i32
    unreachable)
  (global (export "") (mut)
    i32.const 0))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn imported() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (global (import "" "") i32))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
