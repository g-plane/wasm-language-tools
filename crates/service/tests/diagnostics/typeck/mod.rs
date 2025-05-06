use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

mod array;
mod block;
mod block_if;
mod block_loop;
mod br;
mod call;
mod call_indirect;
mod drop;
mod elem;
mod func;
mod global;
mod local;
mod offset;
mod rec;
mod ref_instr;
mod returning;
mod select;
mod structs;
mod subtyping;
mod table;

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
