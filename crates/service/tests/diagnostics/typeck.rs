use super::*;
use insta::assert_json_snapshot;
use lsp_types::Uri;
use wat_service::LanguageService;

#[test]
fn expected_instr() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (i32.add 1 (i32.const 0))))";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn less_operands() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (i32.add (i32.const 0))))";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn more_operands() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (i32.add (i32.const 0) (i32.const 0) (i32.const 0))))";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn operand_count_pluralization() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (i32.const)))";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn builtin_instr_type_mismatch() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (i32.add (i64.const 1) (i32.const 0))))";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn type_mismatch_from_func_results() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $getTwo (result i64 i32)
        (i64.const 2) (i32.const 3)
    )
    (func $add (result i32)
        (i32.add (call $getTwo))
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn param_type_mismatch() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param i64) (result i32)
        (i32.add (local.get 0) (i32.const 1))
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn local_type_mismatch() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (result i32) (local i64)
        (i32.add (local.get 0) (i32.const 1))
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn global_type_mismatch() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (global f32)
    (func (result i32)
        (i32.add (global.get 0) (i32.const 1))
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn call_type_mismatch() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $f1 (param f32))
    (func (call $f1 (i32.const 0)))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn less_operands_on_stack() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param i32 i32) (result i32)
        local.get 0
        i32.sub))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn stacked_type_mismatch_from_func_params() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $swap (param i32 i32) (result i32 i32)
        local.get 1
        local.get 0)
    (func (param f32 i32) (result i32)
        local.get 0
        local.get 1
        call $swap
        i32.sub))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn stacked_type_mismatch_from_func_results() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $swap (param i32 i32) (result i32 f32)
        local.get 1
        local.get 0)
    (func (param i32 i32) (result i32)
        local.get 0
        local.get 1
        call $swap
        i32.sub))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn stacked_type_mismatch_from_instr_meta() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $swap (param i32 i32) (result i32 i32)
        local.get 1
        local.get 0)
    (func (param i32 i32) (result i32)
        local.get 0
        local.get 1
        call $swap
        f32.sub)
    (func (param i32 i32) (result i32)
        local.get 0
        local.get 1
        f32.sub))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
