use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn not_matched_instr() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory $m)
  (func
    memory.get))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 4, 10, 4, 10));
    assert!(response.is_none());
}

#[test]
fn no_memory() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    memory.init))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 3, 10, 3, 10));
    assert!(response.is_none());
}

#[test]
fn no_table() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    table.set))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 3, 10, 3, 10));
    assert!(response.is_none());
}

#[test]
fn load_store() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory $m)
  (func
    i64.load32_s))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 4, 10, 4, 10));
    assert_json_snapshot!(response);
}

#[test]
fn load_store_with_mem_arg() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory $m)
  (func
    v128.load64_splat offset=0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 4, 10, 4, 10));
    assert_json_snapshot!(response);
}

#[test]
fn load_store_with_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory)
  (func
    f64.load $m))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 4, 10, 4, 10));
    assert!(response.is_none());
}

#[test]
fn memory_grow_with_immediate() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory $m)
  (func
    memory.grow 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 4, 10, 4, 10));
    assert!(response.is_none());
}

#[test]
fn memory_fill() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory)
  (func
    memory.fill))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 4, 10, 4, 10));
    assert_json_snapshot!(response);
}

#[test]
fn memory_init_0_immediates() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory $m)
  (func
    memory.init))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 4, 10, 4, 10));
    assert_json_snapshot!(response);
}

#[test]
fn memory_init_1_immediate() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory $m)
  (func
    memory.init $e))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 4, 10, 4, 10));
    assert_json_snapshot!(response);
}

#[test]
fn memory_init_2_immediates() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory)
  (func
    memory.init $m 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 4, 10, 4, 10));
    assert!(response.is_none());
}

#[test]
fn memory_copy_0_immediates() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory $m)
  (func
    memory.copy))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 4, 10, 4, 10));
    assert_json_snapshot!(response);
}

#[test]
fn memory_copy_1_immediate() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory $m)
  (func
    memory.copy 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 4, 10, 4, 10));
    assert_json_snapshot!(response);
}

#[test]
fn memory_copy_2_immediates() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory $m)
  (func
    memory.copy $m 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 4, 10, 4, 10));
    assert!(response.is_none());
}

#[test]
fn table_get() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $t)
  (func
    table.get))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 4, 10, 4, 10));
    assert_json_snapshot!(response);
}

#[test]
fn table_init_0_immediates() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table)
  (func
    table.init))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 4, 10, 4, 10));
    assert_json_snapshot!(response);
}

#[test]
fn table_init_1_immediate() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table)
  (func
    table.init $e))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 4, 10, 4, 10));
    assert_json_snapshot!(response);
}

#[test]
fn table_init_2_immediates() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table)
  (func
    table.init $t 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 4, 10, 4, 10));
    assert!(response.is_none());
}

#[test]
fn table_copy_0_immediates() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table)
  (func
    table.copy))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 4, 10, 4, 10));
    assert_json_snapshot!(response);
}

#[test]
fn table_copy_1_immediate() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $t)
  (func
    table.copy $t))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 4, 10, 4, 10));
    assert_json_snapshot!(response);
}

#[test]
fn table_copy_2_immediates() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table)
  (func
    table.copy 0 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.code_action(create_params(uri, 4, 10, 4, 10));
    assert!(response.is_none());
}
