use super::*;
use wat_service::LanguageService;

#[test]
fn i32_load() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory i32 0)
  (memory i64 0)
  (func (result i32 i32)
    (i32.load
      (i32.const 0))
    (i32.load16_s 1
      (i64.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn i64_load() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory i32 0)
  (memory i64 0)
  (func (result i64 i64)
    (i64.load
      (i32.const 0))
    (i64.load32_u 1
      (i64.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn f32_load() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory i32 0)
  (memory i64 0)
  (func (result f32 f32)
    (f32.load
      (i32.const 0))
    (f32.load 1
      (i64.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn f64_load() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory i32 0)
  (memory i64 0)
  (func (result f64 f64)
    (f64.load
      (i32.const 0))
    (f64.load 1
      (i64.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn i32_store() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory i32 0)
  (memory i64 0)
  (func
    (i32.store
      (i32.const 0)
      (i32.const 0))
    (i32.store16 1
      (i64.const 0)
      (i32.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn i64_store() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory i32 0)
  (memory i64 0)
  (func
    (i64.store
      (i32.const 0)
      (i64.const 0))
    (i64.store32 1
      (i64.const 0)
      (i64.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn f32_store() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory i32 0)
  (memory i64 0)
  (func
    (f32.store
      (i32.const 0)
      (f32.const 0))
    (f32.store 1
      (i64.const 0)
      (f32.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn f64_store() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory i32 0)
  (memory i64 0)
  (func
    (f64.store
      (i32.const 0)
      (f64.const 0))
    (f64.store 1
      (i64.const 0)
      (f64.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn memory_size() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory i32 0)
  (memory i64 0)
  (func (result i32 i64)
    (memory.size)
    (memory.size 1)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn memory_grow() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory i32 0)
  (memory i64 0)
  (func (result i32 i64)
    (memory.grow (i32.const 0))
    (memory.grow 1 (i64.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn memory_init() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory i32 0)
  (memory i64 0)
  (func
    (memory.init 0
      (i32.const 0)
      (i32.const 0)
      (i32.const 0))
    (memory.init 1 0
      (i64.const 0)
      (i32.const 0)
      (i32.const 0)))
  (data))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn memory_copy() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory i32 0)
  (memory i64 0)
  (func
    (memory.copy (i32.const 0) (i32.const 0) (i32.const 0))
    (memory.copy 0 1 (i32.const 0) (i64.const 0) (i32.const 0))
    (memory.copy 1 0 (i64.const 0) (i32.const 0) (i32.const 0))
    (memory.copy 1 1 (i64.const 0) (i64.const 0) (i64.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn memory_fill() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory i32 0)
  (memory i64 0)
  (func
    (memory.fill (i32.const 0) (i32.const 0) (i32.const 0))
    (memory.fill 1 (i64.const 0) (i32.const 0) (i64.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn v128_load() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory i32 0)
  (memory i64 0)
  (func (result v128 v128)
    (v128.load 0
      (i32.const 0))
    (v128.load64_splat 1
      (i64.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn v128_store() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory i32 0)
  (memory i64 0)
  (func (param v128)
    (v128.store 0
      (i32.const 0)
      (local.get 0))
    (v128.store16_lane 1
      (i64.const 0)
      (local.get 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn v128_load_lane() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory i32 0)
  (memory i64 0)
  (func (param v128) (result v128 v128)
    (v128.load8_lane 0
      (i32.const 0)
      (local.get 0))
    (v128.load64_lane 1
      (i64.const 0)
      (local.get 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn table_get() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table i32 0 structref)
  (table i64 0 arrayref)
  (func (result structref arrayref)
    (table.get
      (i32.const 0))
    (table.get 1
      (i64.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn table_set() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table i32 0 structref)
  (table i64 0 arrayref)
  (func (param structref arrayref)
    (table.set
      (i32.const 0)
      (local.get 0))
    (table.set 1
      (i64.const 0)
      (local.get 1))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn table_init() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table i32 0 externref)
  (table i64 0 externref)
  (func
    (table.init 0
      (i32.const 0)
      (i32.const 0)
      (i32.const 0))
    (table.init 1 0
      (i64.const 0)
      (i32.const 0)
      (i32.const 0)))
  (elem 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn table_copy() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table i32 0 anyref)
  (table i64 0 anyref)
  (func
    (table.copy (i32.const 0) (i32.const 0) (i32.const 0))
    (table.copy 0 1 (i32.const 0) (i64.const 0) (i32.const 0))
    (table.copy 1 0 (i64.const 0) (i32.const 0) (i32.const 0))
    (table.copy 1 1 (i64.const 0) (i64.const 0) (i64.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn table_grow() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table i32 0 structref)
  (table i64 0 arrayref)
  (func (param structref arrayref) (result i32 i64)
    (table.grow
      (local.get 0)
      (i32.const 0))
    (table.grow 1
      (local.get 1)
      (i64.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn table_size() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table i32 0 externref)
  (table i64 0 externref)
  (func (result i32 i64)
    (table.size)
    (table.size 1)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn table_fill() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table i32 0 structref)
  (table i64 0 arrayref)
  (func (param structref arrayref)
    (table.fill
      (i32.const 0)
      (local.get 0)
      (i32.const 0))
    (table.fill 1
      (i64.const 0)
      (local.get 1)
      (i64.const 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
