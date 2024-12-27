use super::*;
use insta::assert_json_snapshot;
use lsp_types::Uri;
use wat_service::LanguageService;

#[test]
fn index() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func
        (call \"\")
        (local.get 1.0)
        (local.set 1.0 (i32.const 0))
        (global.get 1.0)
        (global.set \"\" (i32.const 0))
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    allow_unused(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn int() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (result i32 i64 v128)
        (i32.const 1.0)
        (i64.const 1.0)
        (v128.const 1.0)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    allow_unused(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn float() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (result f32 f64)
        (f32.const 1)
        (f64.const $a)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    allow_unused(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn indexes() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func
        (table.copy 1.0 1.0 (i32.const 1) (i32.const 1) (i32.const 1))
        (table.init $a \"\" (i32.const 1) (i32.const 1) (i32.const 1))
    )
    (table $a 0 funcref)
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    allow_unused(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn mem_arg() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func
        (i32.load 1 (i32.const 0))
        (f64.store 1 (i32.const 0) (f64.const 0.0))
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    allow_unused(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn mem_arg_and_index() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func
        (v128.load8_lane 1 \"\" (i32.const 0) (v128.const 0))
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    allow_unused(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn expected_instr() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (i32.add 1 (i32.const 0))))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    allow_unused(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn ignored_expecting_instr() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (br_table 0 1)))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    allow_unused(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}
