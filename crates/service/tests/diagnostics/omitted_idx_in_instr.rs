use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

fn disable_other_lints(service: &mut LanguageService, uri: &str) {
    service.set_config(
        uri,
        Some(ServiceConfig {
            lint: Lints {
                unused: LintLevel::Allow,
                omitted_idx_in_instr: LintLevel::Deny,
                ..Default::default()
            },
            ..Default::default()
        }),
    );
}

#[test]
fn single_memory_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory $m 0)
  (func
    i32.const 0
    i64.const 0
    i64.store32 0)
  (func
    i32.const 0
    f64.const 0
    f64.store)
  (func
    i32.const 0
    v128.load16_splat $m offset=0
    drop)
  (func (param v128)
    i32.const 0
    local.get 0
    v128.load32_lane offset=0
    drop))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn memory_init() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory $m 0)
  (data)
  (func
    i32.const 0
    i32.const 0
    i32.const 0
    memory.init 0)
  (func
    i32.const 0
    i32.const 0
    i32.const 0
    memory.init 0 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn memory_copy() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory $m 0)
  (func
    i32.const 0
    i32.const 0
    i32.const 0
    memory.copy)
  (func
    i32.const 0
    i32.const 0
    i32.const 0
    memory.copy 0)
  (func
    i32.const 0
    i32.const 0
    i32.const 0
    memory.copy 0 $m))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn single_table_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $t 0 funcref)
  (func
    i32.const 0
    table.get 0
    drop)
  (func (param funcref)
    local.get 0
    i32.const 0
    table.grow $t
    drop)
  (func
    table.size
    drop))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn table_init() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $t 0 funcref)
  (elem 0)
  (func
    i32.const 0
    i32.const 0
    i32.const 0
    table.init 0)
  (func
    i32.const 0
    i32.const 0
    i32.const 0
    table.init 0 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn table_copy() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $t 0 funcref)
  (func
    i32.const 0
    i32.const 0
    i32.const 0
    table.copy)
  (func
    i32.const 0
    i32.const 0
    i32.const 0
    table.copy $t)
  (func
    i32.const 0
    i32.const 0
    i32.const 0
    table.copy $t 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
