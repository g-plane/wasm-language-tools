use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

mod array;
mod block;
mod block_if;
mod block_loop;
mod br;
mod call_indirect;
mod drop;
mod returning;
mod select;
mod structs;
mod subtyping;

#[test]
fn less_operands() {
    let uri = "untitled:test".to_string();
    let source = "(module (func (result i32) (i32.add (i32.const 0))))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn more_operands() {
    let uri = "untitled:test".to_string();
    let source = "(module (func (i32.add (i32.const 0) (i32.const 0) (i32.const 0))))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn operand_count_pluralization() {
    let uri = "untitled:test".to_string();
    let source = "(module (func (result i32) (i32.eqz)))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn builtin_instr_type_mismatch() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result i32)
    (i32.add
      (i64.const 1)
      (i32.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn type_mismatch_from_func_results() {
    let uri = "untitled:test".to_string();
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
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn param_type_mismatch() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param i64) (result i32)
        (i32.add (local.get 0) (i32.const 1))
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn local_type_mismatch() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (result i32) (local i64)
        (i32.add (local.get 0) (i32.const 1))
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn global_type_mismatch() {
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
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn call_type_mismatch() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $f1 (param f32))
    (func (call $f1 (i32.const 0)))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn less_operands_on_stack() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param i32 i32) (result i32)
        local.get 0
        i32.sub))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn stacked_type_mismatch_from_func_params() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $swap (param i32 i32) (result i32 i32)
        local.get 1
        local.get 0)
    (func (param f32 i32) (result i32)
        local.get 0
        local.get 1
        call $swap
        i32.sub)

    (func (param f32 f64))
    (func
        i32.const 0
        call 2))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn stacked_type_mismatch_from_func_results() {
    let uri = "untitled:test".to_string();
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
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn stacked_type_mismatch_from_instr_sig() {
    let uri = "untitled:test".to_string();
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
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn mixed_type_mismatch_from_instr_sig() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (result i32)
        (i32.const 1) (f32.const 2.0) i32.add
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn mixed_matches_from_call() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $fib$naive (param i32) (result i32)
    (call $fib$naive
      (i32.sub
        (local.get 0)
        (i32.const 1)))
    (call $fib$naive
      (i32.sub
        (local.get 0)
        (i32.const 2)))
    i32.add))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn undefined_local_and_global() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result i32)
    (i32.add
      ;; undefined locals and globals shouldn't be reported as 'missing operands'
      (global.get 1)
      (local.get 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn incomplete_folded() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result i32)
    (i32.const 1)
    (i32.const 2)
    (i32.add)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn unreachable_with_matched_count() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param i32 f32) (result i32)
    (unreachable)
    (local.get 1)
    i32.add))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn unreachable_with_mismatched_count() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param i32 f32) (result i32)
    (unreachable)
    i32.add))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn func_results_incorrect() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result i32 i32)
    block (result i32)
      unreachable
    end)
  (func (result i32 i32)
    block (result i32 f32)
      unreachable
    end)
  (func (result i32 i32)
    block (result i32 i32 i32)
      unreachable
    end)
  (func (result i32 i32)
    i32.const 0
    i32.const 0
    f32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn func_results_correct() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result i32 i32)
    unreachable)
  (func (result i32 i32)
    (f32.const 0)
    (unreachable))
  (func (result i32 i32)
    (f32.const 0)
    (f32.const 0)
    (unreachable))
  (func (result i32 i32)
    block (result i32 i32)
      unreachable
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn global_results_incorrect() {
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
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn global_results_correct() {
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
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn imported_global() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (global (import "" "") i32))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn excessive_at_end() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    block (result i32 i32)
      i32.const 0
      i32.const 0
      i32.const 0
      unreachable
      i32.const 0
      i32.const 0
      i32.const 0
    end
    drop
    drop)
  (func
    block (result i32 i32)
      i32.const 0
      i32.const 0
      i32.const 0
      i32.const 0
      br_table 0
      i32.const 0
      i32.const 0
      i32.const 0
    end
    drop
    drop))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
