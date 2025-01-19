use super::*;
use insta::assert_json_snapshot;
use lsp_types::Uri;
use wat_service::LanguageService;

#[test]
fn less_operands() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (result i32) (i32.add (i32.const 0))))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn more_operands() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (i32.add (i32.const 0) (i32.const 0) (i32.const 0))))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn operand_count_pluralization() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (func (result i32) (i32.eqz)))";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn builtin_instr_type_mismatch() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
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
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
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
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn global_type_mismatch() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn stacked_type_mismatch_from_instr_sig() {
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
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn mixed_type_mismatch_from_instr_sig() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn undefined_local_and_global() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
fn block_type_in_stack() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (param i32 i32) (result i32)
    (block (result i32 i32)
      (local.get 0)
      (local.get 1))
    i32.add))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn block_type_folded() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (param i32 i32) (result i32)
    (i32.add
      (block (result i32 i32)
        (local.get 0)
        (local.get 1)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn drop_correct() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    i64.const 0
    drop))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn drop_incorrect() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    drop))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn incomplete_folded() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn unreachable_with_matched_count() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn block_folded() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (param i32 f32) (result i32)
    (block (result i32)
      (i32.add
        (local.get 0)
        (local.get 1)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn block_sequence() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (param i32 f32) (result i32)
    block (result i32)
      local.get 0
      local.get 1
      i32.add
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn loop_folded() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (param i32 f32) (result i32)
    (loop (result i32)
      (i32.add
        (local.get 0)
        (local.get 1)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn loop_sequence() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (param i32 f32) (result i32)
    loop (result i32)
      local.get 0
      local.get 1
      i32.add
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn then_folded() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (param i32 f32) (result i32)
    (if (result i32)
      (i32.const 0)
      (then
        (i32.add
          (local.get 0)
          (local.get 1)))
      (else
        (i32.const 0)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn else_folded() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (param i32 f32) (result i32)
    (if (result i32)
      (i32.const 0)
      (then
        (i32.const 0))
      (else
        (i32.add
          (local.get 0)
          (local.get 1))))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn then_sequence() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (param i32 f32) (result i32)
    i32.const 0
    if (result i32)
      local.get 0
      local.get 1
      i32.add
    else
      i32.const 0
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn else_sequence() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (param i32 f32) (result i32)
    i32.const 0
    if (result i32)
      i32.const 0
    else
      local.get 0
      local.get 1
      i32.add
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn new_stack_for_new_block() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (result i32)
    i32.const 0
    i32.const 1
    block (result i32)
      i32.add
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn block_params_boundary_missing() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    i32.const 0
    block (param i32 i32)
      i32.add
      drop
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn block_params_boundary_mismatched() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    i32.const 0
    f32.const 1
    block (param i32 i32)
      i32.add
      drop
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn block_params_mismatched() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    i32.const 0
    f32.const 1
    block (param i32 f32)
      i32.add
      drop
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn block_params_correct() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    i32.const 0
    i32.const 1
    block (param i32 i32)
      i32.add
      drop
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn block_results_folded() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (result i32 i32)
    (block (result i32 i32)
      (i32.const 0)))
  (func (result i32 i32)
    (block (result i32 i32)
      (i32.const 0)
      (f32.const 0)))
  (func (result i32 i32)
    (block (result i32 i32)
      (i32.const 0)
      (i32.const 0)
      (i32.const 0)))
  (func (result i32 i32)
    (block (result i32 i32)
      (i32.const 0)
      (i32.const 0)
      (f32.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn block_results_sequence() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (result i32 i32)
    block (result i32 i32)
      i32.const 0
    end)
  (func (result i32 i32)
    block (result i32 i32)
      i32.const 0
      f32.const 0
    end)
  (func (result i32 i32)
    block (result i32 i32)
      i32.const 0
      i32.const 0
      i32.const 0
    end)
  (func (result i32 i32)
    block (result i32 i32)
      i32.const 0
      i32.const 0
      f32.const 0
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn block_results_correct() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (type $t (func (result i32 i32)))
  (func (result i32 i32)
    (block (result i32 i32)
      unreachable))
  (func (result i32 i32)
    (block (result i32 i32)
      (i32.const 0)
      (f32.const 0)
      (unreachable)))
  (func (result i32 i32)
    (block (result i32 i32)
      (f32.const 0)
      (i32.const 0)
      (unreachable)))
  (func (result i32 i32)
    (block (result i32 i32)
      (i32.const 0)
      (i32.const 0)))
  (func (result i32 i32)
    (block (type $t)
      (i32.const 0)
      (i32.const 0)))
  (func (result i32 i32)
    block (result i32 i32)
      unreachable
    end)
  (func (result i32 i32)
    block (result i32 i32)
      i32.const 0
      f32.const 0
      unreachable
    end)
  (func (result i32 i32)
    block (result i32 i32)
      f32.const 0
      i32.const 0
      unreachable
    end)
  (func (result i32 i32)
    block (result i32 i32)
      i32.const 0
      i32.const 0
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn loop_results_folded() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (result i32 i32)
    (loop (result i32 i32)
      (i32.const 0)))
  (func (result i32 i32)
    (loop (result i32 i32)
      (i32.const 0)
      (f32.const 0)))
  (func (result i32 i32)
    (loop (result i32 i32)
      (i32.const 0)
      (i32.const 0)
      (i32.const 0)))
  (func (result i32 i32)
    (loop (result i32 i32)
      (i32.const 0)
      (i32.const 0)
      (f32.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn loop_results_sequence() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (result i32 i32)
    loop (result i32 i32)
      i32.const 0
    end)
  (func (result i32 i32)
    loop (result i32 i32)
      i32.const 0
      f32.const 0
    end)
  (func (result i32 i32)
    loop (result i32 i32)
      i32.const 0
      i32.const 0
      i32.const 0
    end)
  (func (result i32 i32)
    loop (result i32 i32)
      i32.const 0
      i32.const 0
      f32.const 0
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn loop_results_correct() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (result i32 i32)
    (loop (result i32 i32)
      unreachable))
  (func (result i32 i32)
    (loop (result i32 i32)
      (i32.const 0)
      (f32.const 0)
      (unreachable)))
  (func (result i32 i32)
    (loop (result i32 i32)
      (f32.const 0)
      (i32.const 0)
      (unreachable)))
  (func (result i32 i32)
    (loop (result i32 i32)
      (i32.const 0)
      (i32.const 0)))
  (func (result i32 i32)
    loop (result i32 i32)
      unreachable
    end)
  (func (result i32 i32)
    loop (result i32 i32)
      i32.const 0
      f32.const 0
      unreachable
    end)
  (func (result i32 i32)
    loop (result i32 i32)
      f32.const 0
      i32.const 0
      unreachable
    end)
  (func (result i32 i32)
    loop (result i32 i32)
      i32.const 0
      i32.const 0
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn then_results_folded() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (i32.const 0))
      (else
        (unreachable))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (i32.const 0)
        (f32.const 0))
      (else
        (unreachable))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (i32.const 0)
        (i32.const 0)
        (i32.const 0))
      (else
        (unreachable))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (i32.const 0)
        (i32.const 0)
        (f32.const 0))
      (else
        (unreachable)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn then_results_sequence() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      i32.const 0
    else
      unreachable
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      i32.const 0
      f32.const 0
    else
      unreachable
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      i32.const 0
      i32.const 0
      i32.const 0
    else
      unreachable
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      i32.const 0
      i32.const 0
      f32.const 0
    else
      unreachable
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn then_results_correct() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (unreachable))
      (else
        (unreachable))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (i32.const 0)
        (f32.const 0)
        (unreachable))
      (else
        (unreachable))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (f32.const 0)
        (i32.const 0)
        (unreachable))
      (else
        (unreachable))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (i32.const 0)
        (i32.const 0))
      (else
        (unreachable))))
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      unreachable
    else
      unreachable
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      f32.const 0
      i32.const 0
      unreachable
    else
      unreachable
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      i32.const 0
      f32.const 0
      unreachable
    else
      unreachable
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      i32.const 0
      i32.const 0
    else
      unreachable
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn else_results_folded() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then (i32.const 0) (i32.const 0))
      (else
        (i32.const 0))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then (i32.const 0) (i32.const 0))
      (else
        (i32.const 0)
        (f32.const 0))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then (i32.const 0) (i32.const 0))
      (else
        (i32.const 0)
        (i32.const 0)
        (i32.const 0))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then (i32.const 0) (i32.const 0))
      (else
        (i32.const 0)
        (i32.const 0)
        (f32.const 0)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn else_results_sequence() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      i32.const 0
      i32.const 0
    else
      i32.const 0
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      i32.const 0
      i32.const 0
    else
      i32.const 0
      f32.const 0
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      i32.const 0
      i32.const 0
    else
      i32.const 0
      i32.const 0
      i32.const 0
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      i32.const 0
      i32.const 0
    else
      i32.const 0
      i32.const 0
      f32.const 0
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn else_results_correct() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (unreachable))
      (else
        (unreachable))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (unreachable))
      (else
        (i32.const 0)
        (i32.const 0))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (unreachable))
      (else
        (f32.const 0)
        (i32.const 0)
        (unreachable))))
  (func (result i32 i32)
    (if (result i32 i32)
      (i32.const 1)
      (then
        (unreachable))
      (else
        (i32.const 0)
        (f32.const 0)
        (unreachable))))
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      unreachable
    else
      unreachable
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      unreachable
    else
      f32.const 0
      i32.const 0
      unreachable
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      unreachable
    else
      i32.const 0
      f32.const 0
      unreachable
    end)
  (func (result i32 i32)
    i32.const 1
    if (result i32 i32)
      unreachable
    else
      i32.const 0
      i32.const 0
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn func_results_incorrect() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn global_results_incorrect() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
fn missing_then() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (if
      (i32.const 0))
    (if
      (i32.const 0)
      (else))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn missing_else() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (drop
      (if (result i32)
        (i32.const 1)
        (then
          (i32.const 0)))))
  (func
    i32.const 1
    if (result i32)
      i32.const 0
    end)
  (func
    i32.const 1
    if
     ;; missing else is allowed since both branches return nothing
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn if_cond_incorrect() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (if
      (then)))
  (func
    (if
      (f32.const 1)
      (then)))
  (func
    (if
      (i32.add
        (i32.const 0)
        (f32.const 0))
      (then)))
  (func
    (if
      (i32.const 1)
      (i32.const 1)
      (then)))
  (func
    if
    end)
  (func
    f32.const 1
    if
    end)
  (func
    i32.const 1
    i32.const 1
    if
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn if_cond_correct() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (if
      (i32.const 1)
      (then)))
  (func (result f32)
    (if
      (f32.const 1)
      (i32.const 1)
      (then)))
  (func
    i32.const 1
    if
    end)
  (func (result f32)
    f32.const 1
    i32.const 1
    if
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn return_incorrect() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (result i32)
    (block
      (return))
    (unreachable))
  (func (result i32)
    (block
      (f32.const 0)
      (return))
    (unreachable))
  (func (result i32)
    block
      return
    end
    unreachable)
  (func (result i32)
    block
      f32.const 0
      return
    end
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn return_correct() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (result i32)
    (block
      (i32.const 0)
      (return))
    (unreachable))
  (func (result i32)
    (block
      (i32.const 0)
      (i32.const 0)
      (return))
    (unreachable))
  (func (result i32)
    block
      i32.const 0
      return
    end
    unreachable)
  (func (result i32)
    block
      i32.const 0
      i32.const 0
      return
    end
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn br_incorrect() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (block (result f32)
      (br 0))
    (unreachable))
  (func
    (block (result f32)
      (f64.const 0)
      (br 0))
    (unreachable))
  (func
    (block (result f32 f32)
      (block (result f64 f64)
        (br 1
          (f64.const 0)
          (f64.const 0)))
      (unreachable))
    (unreachable))
  (func
    block (result f32)
      br 0
    end
    unreachable)
  (func
    block (result f32)
      f64.const 0
      br 0
    end
    unreachable)
  (func
    block (result f32 f32)
      block (result f64 f64)
        f64.const 0
        f64.const 0
        br 1
      end
      unreachable
    end
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn br_correct() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (block (result i32)
      (i32.const 0)
      (br 0))
    (unreachable))
  (func
    (block (result i32)
      (i32.const 0)
      (i32.const 0)
      (br 0))
    (unreachable))
  (func
    (block (result f32 f32)
      (block (result f64 f64)
        (br 1
          (f32.const 0)
          (f32.const 0)))
      (unreachable))
    (unreachable))
  (func
    block (result i32)
      i32.const 0
      br 0
    end
    unreachable)
  (func
    block (result i32)
      i32.const 0
      i32.const 0
      br 0
    end
    unreachable)
  (func
    block (result f32 f32)
      block (result f64 f64)
        f32.const 0
        f32.const 0
        br 1
      end
      unreachable
    end
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn br_if_incorrect() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    // copied from https://github.com/WebAssembly/spec/blob/f3a0e06235d2d84bb0f3b5014da4370613886965/test/core/br_if.wast
    let source = "
(module
  (func $type-false-f64
    (block
      (f64.neg
        (br_if 0
          (i32.const 0)))))

  (func $type-true-f64
    (block
      (f64.neg
        (br_if 0
          (i64.const 1)))))

  (func $type-false-arg-vs-num (result i32)
    (block (result i32)
      (br_if 0
        (i32.const 0))
      (i32.const 1)))
  (func $type-true-arg-vs-num (result i32)
    (block (result i32)
      (br_if 0
        (i32.const 1))
      (i32.const 1)))

  (func $type-false-arg-num-vs-void
    (block
      (br_if 0
        (i32.const 0)
        (i32.const 0))))
  (func $type-true-arg-num-vs-void
    (block
      (br_if 0
        (i32.const 0)
        (i32.const 1))))

  (func $type-false-arg-void-vs-num (result i32)
    (block (result i32)
      (br_if 0
        (nop)
        (i32.const 0))
      (i32.const 1)))
  (func $type-true-arg-void-vs-num (result i32)
    (block (result i32)
      (br_if 0
        (nop)
        (i32.const 1))
      (i32.const 1)))

  (func $type-false-arg-num-vs-num (result i32)
    (block (result i32)
      (drop
        (br_if 0
          (i64.const 1)
          (i32.const 0)))
      (i32.const 1)))
  (func $type-true-arg-num-vs-num (result i32)
    (block (result i32)
      (drop
        (br_if 0
          (i64.const 1)
          (i32.const 0)))
      (i32.const 1)))

  (func $type-cond-empty-vs-i32
    (block
      (br_if 0)))
  (func $type-cond-void-vs-i32
    (block
      (br_if 0
        (nop))))
  (func $type-cond-num-vs-i32
    (block
      (br_if 0
        (i64.const 0))))
  (func $type-arg-cond-void-vs-i32 (result i32)
    (block (result i32)
      (br_if 0
        (i32.const 0)
        (nop))
      (i32.const 1)))
  (func $type-arg-void-vs-num-nested (result i32)
    (block (result i32)
      (i32.const 0)
      (block
        (br_if 1
          (i32.const 1)))))
  (func $type-arg-cond-num-vs-i32 (result i32)
    (block (result i32)
      (br_if 0
        (i32.const 0)
        (i64.const 0))
      (i32.const 1)))

  (func $type-1st-cond-empty-in-then
    (block
      (i32.const 0)
      (i32.const 0)
      (if (result i32)
        (then
          (br_if 0))))
    (i32.eqz)
    (drop))
  (func $type-2nd-cond-empty-in-then
    (block
      (i32.const 0)
      (i32.const 0)
      (if (result i32)
        (then
          (br_if 0
            (i32.const 1)))))
    (i32.eqz)
    (drop))
  (func $type-1st-cond-empty-in-return
    (block (result i32)
      (return
        (br_if 0)))
    (i32.eqz)
    (drop))
  (func $type-2nd-cond-empty-in-return
    (block (result i32)
      (return
        (br_if 0
          (i32.const 1))))
    (i32.eqz)
    (drop))
  (func
    block (result f32 f32)
      block (result f64 f64)
        f64.const 0
        f64.const 0
        i32.const 0
        br_if 1
      end
      unreachable
    end
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn br_if_correct() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    // copied from https://github.com/WebAssembly/spec/blob/f3a0e06235d2d84bb0f3b5014da4370613886965/test/core/br_if.wast
    let source = r#"
(module
  (func $dummy)
  (func (export "type-f32")
    (block
      (drop
        (f32.neg
          (br_if 0
            (f32.const 0)
            (i32.const 1))))))
  (func (export "type-f32-value") (result f32)
    (block (result f32)
      (f32.neg
        (br_if 0
          (f32.const 3)
          (i32.const 1)))))

  (func (export "as-block-first") (param i32) (result i32)
    (block
      (br_if 0
        (local.get 0))
      (return
        (i32.const 2)))
    (i32.const 3))
  (func (export "as-block-first-value") (param i32) (result i32)
    (block (result i32)
      (drop
        (br_if 0
          (i32.const 10)
          (local.get 0)))
      (return
        (i32.const 11))))
  (func (export "as-loop-last") (param i32)
    (block
      (loop
        (call $dummy)
        (br_if 1
          (local.get 0)))))

  (func (export "as-br-value") (result i32)
    (block (result i32)
      (br 0
        (br_if 0
          (i32.const 1)
          (i32.const 2)))))
  (func (export "as-br_if-cond")
    (block
      (br_if 0
        (br_if 0
          (i32.const 1)
          (i32.const 1)))))
  (func (export "as-br_if-value") (result i32)
    (block (result i32)
      (drop
        (br_if 0
          (br_if 0
            (i32.const 1)
            (i32.const 2))
          (i32.const 3)))
      (i32.const 4)))
  (func (export "as-br_if-value-cond") (param i32) (result i32)
    (block (result i32)
      (drop
        (br_if 0
          (i32.const 2)
          (br_if 0
            (i32.const 1)
            (local.get 0))))
      (i32.const 4))))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn br_table_incorrect() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    block
      br_table 0
    end)
  (func
    block (result f32)
      br_table 0
    end
    unreachable)
  (func
    block (result f32)
      f64.const 0
      br_table 0
    end
    unreachable)
  (func
    block (result f32)
      i32.const 0
      br_table 0
    end
    unreachable)
  (func
    block (result f32)
      f32.const 0
      br_table 0
    end
    unreachable)
  (func
    block (result f32)
      f64.const 0
      i32.const 0
      br_table 0
    end
    unreachable)
  (func
    block (result f32 f32)
      block (result f64 f64)
        f64.const 0
        f64.const 0
        i32.const 0
        br_table 1
      end
      unreachable
    end
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn br_table_correct() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    block (result f32)
      f32.const 0
      i32.const 0
      br_table 0
    end
    unreachable)
  (func
    block (result f32)
      i32.const 0
      f32.const 0
      i32.const 0
      br_table 0
    end
    unreachable)
  (func
    block (result f32 f32)
      block (result f64 f64)
        f32.const 0
        f32.const 0
        i32.const 0
        br_table 1
      end
      unreachable
    end
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn excessive_at_end() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
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

#[test]
fn select_incorrect() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    select
    drop)
  (func
    i32.const 0
    select
    drop)
  (func
    f32.const 0
    select
    drop)
  (func
    f32.const 0
    f32.const 0
    select
    drop)
  (func
    f32.const 0
    i32.const 0
    select
    drop)
  (func
    f32.const 0
    f64.const 0
    i32.const 0
    select
    drop)
  (func (result f64)
    f64.const 0
    f32.const 0
    i32.const 0
    select)
  (func (result f64)
    f32.const 0
    f32.const 0
    i32.const 0
    select)

  (func
    select (result f64))
  (func
    i32.const 0
    select (result f64)
    drop)
  (func
    f32.const 0
    select (result f64)
    drop)
  (func
    f32.const 0
    f32.const 0
    select (result f64)
    drop)
  (func
    f32.const 0
    f32.const 0
    select (result f32)
    drop)
  (func
    f32.const 0
    i32.const 0
    select (result f32)
    drop)
  (func
    f32.const 0
    f64.const 0
    i32.const 0
    select (result f32)
    drop)
  (func (result f32)
    f64.const 0
    f32.const 0
    i32.const 0
    select (result f32))
  (func (result f64)
    f32.const 0
    f32.const 0
    i32.const 0
    select (result f64)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn select_correct() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    f32.const 0
    f32.const 0
    i32.const 0
    select
    drop)
  (func (result f32)
    f32.const 0
    f32.const 0
    i32.const 0
    select)

  (func
    f32.const 0
    f32.const 0
    i32.const 0
    select (result f32)
    drop)
  (func (result f32)
    f32.const 0
    f32.const 0
    i32.const 0
    select (result f32)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn call_indirect_incorrect() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (table 0 funcref)
  (type (func (param f32) (result f64)))
  (func
    call_indirect (type 0))
  (func
    i32.const 0
    call_indirect (type 0))
  (func
    f32.const 0
    call_indirect (type 0))
  (func
    i32.const 0
    f32.const 0
    call_indirect 0 (type 0))
  (func
    call_indirect (param f32) (result f64))
  (func
    i32.const 0
    call_indirect (param f32) (result f64))
  (func
    f32.const 0
    call_indirect (param f32) (result f64))
  (func
    i32.const 0
    f32.const 0
    call_indirect 0 (param f32) (result f64)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn call_indirect_correct() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (table 0 funcref)
  (type (func (param f32) (result f64)))
  (func (result f64)
    f32.const 0
    i32.const 0
    call_indirect (type 0))
  (func (result f64)
    f32.const 0
    i32.const 0
    call_indirect (param f32) (result f64)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}
