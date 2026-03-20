use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn align_power_of_two() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 0)
  (func (drop (i32.load8_s align=0 (i32.const 0))))
  (func (drop (i32.load align=3 (i32.const 0))))
  (func (f64.store align=7 (i32.const 0) (f64.const 0)))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn align_range() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 0)
  (func (drop (i32.load8_s align=2 (i32.const 0))))
  (func (drop (i32.load8_u align=2 (i32.const 0))))
  (func (drop (i32.load16_s align=4 (i32.const 0))))
  (func (drop (i32.load16_u align=4 (i32.const 0))))
  (func (drop (i32.load align=8 (i32.const 0))))
  (func (drop (i64.load8_s align=2 (i32.const 0))))
  (func (drop (i64.load8_u align=2 (i32.const 0))))
  (func (drop (i64.load16_s align=4 (i32.const 0))))
  (func (drop (i64.load16_u align=4 (i32.const 0))))
  (func (drop (i64.load32_s align=8 (i32.const 0))))
  (func (drop (i64.load32_u align=8 (i32.const 0))))
  (func (drop (i64.load align=16 (i32.const 0))))
  (func (drop (f32.load align=8 (i32.const 0))))
  (func (drop (f64.load align=16 (i32.const 0))))
  (func (drop (i32.load8_s align=2 (i32.const 0))))
  (func (drop (i32.load8_u align=2 (i32.const 0))))
  (func (drop (i32.load16_s align=4 (i32.const 0))))
  (func (drop (i32.load16_u align=4 (i32.const 0))))
  (func (drop (i32.load align=8 (i32.const 0))))
  (func (drop (i64.load8_s align=2 (i32.const 0))))
  (func (drop (i64.load8_u align=2 (i32.const 0))))
  (func (drop (i64.load16_s align=4 (i32.const 0))))
  (func (drop (i64.load16_u align=4 (i32.const 0))))
  (func (drop (i64.load32_s align=8 (i32.const 0))))
  (func (drop (i64.load32_u align=8 (i32.const 0))))
  (func (drop (i64.load align=16 (i32.const 0))))
  (func (drop (f32.load align=8 (i32.const 0))))
  (func (drop (f64.load align=16 (i32.const 0))))
  (func (i32.store8 align=2 (i32.const 0) (i32.const 0)))
  (func (i32.store16 align=4 (i32.const 0) (i32.const 0)))
  (func (i32.store align=8 (i32.const 0) (i32.const 0)))
  (func (i64.store8 align=2 (i32.const 0) (i64.const 0)))
  (func (i64.store16 align=4 (i32.const 0) (i64.const 0)))
  (func (i64.store32 align=8 (i32.const 0) (i64.const 0)))
  (func (i64.store align=16 (i32.const 0) (i64.const 0)))
  (func (f32.store align=8 (i32.const 0) (f32.const 0)))
  (func (f64.store align=16 (i32.const 0) (f64.const 0)))

  (memory i64 0)
  (func (drop (i32.load8_s 1 align=2 (i64.const 0))))
  (func (drop (i32.load8_u 1 align=2 (i64.const 0))))
  (func (drop (i32.load16_s 1 align=4 (i64.const 0))))
  (func (drop (i32.load16_u 1 align=4 (i64.const 0))))
  (func (drop (i32.load 1 align=8 (i64.const 0))))
  (func (drop (i64.load8_s 1 align=2 (i64.const 0))))
  (func (drop (i64.load8_u 1 align=2 (i64.const 0))))
  (func (drop (i64.load16_s 1 align=4 (i64.const 0))))
  (func (drop (i64.load16_u 1 align=4 (i64.const 0))))
  (func (drop (i64.load32_s 1 align=8 (i64.const 0))))
  (func (drop (i64.load32_u 1 align=8 (i64.const 0))))
  (func (drop (i64.load 1 align=16 (i64.const 0))))
  (func (drop (f32.load 1 align=8 (i64.const 0))))
  (func (drop (f64.load 1 align=16 (i64.const 0))))
  (func (drop (i32.load8_s 1 align=2 (i64.const 0))))
  (func (drop (i32.load8_u 1 align=2 (i64.const 0))))
  (func (drop (i32.load16_s 1 align=4 (i64.const 0))))
  (func (drop (i32.load16_u 1 align=4 (i64.const 0))))
  (func (drop (i32.load 1 align=8 (i64.const 0))))
  (func (drop (i64.load8_s 1 align=2 (i64.const 0))))
  (func (drop (i64.load8_u 1 align=2 (i64.const 0))))
  (func (drop (i64.load16_s 1 align=4 (i64.const 0))))
  (func (drop (i64.load16_u 1 align=4 (i64.const 0))))
  (func (drop (i64.load32_s 1 align=8 (i64.const 0))))
  (func (drop (i64.load32_u 1 align=8 (i64.const 0))))
  (func (drop (i64.load 1 align=16 (i64.const 0))))
  (func (drop (f32.load 1 align=8 (i64.const 0))))
  (func (drop (f64.load 1 align=16 (i64.const 0))))
  (func (i32.store8 1 align=2 (i64.const 0) (i32.const 0)))
  (func (i32.store16 1 align=4 (i64.const 0) (i32.const 0)))
  (func (i32.store 1 align=8 (i64.const 0) (i32.const 0)))
  (func (i64.store8 1 align=2 (i64.const 0) (i64.const 0)))
  (func (i64.store16 1 align=4 (i64.const 0) (i64.const 0)))
  (func (i64.store32 1 align=8 (i64.const 0) (i64.const 0)))
  (func (i64.store 1 align=16 (i64.const 0) (i64.const 0)))
  (func (f32.store 1 align=8 (i64.const 0) (f32.const 0)))
  (func (f64.store 1 align=16 (i64.const 0) (f64.const 0)))

  (func (param $x v128) (result v128)
    (v128.load8_lane align=2 0
      (i32.const 0)
      (local.get $x)))
  (func (param $x v128) (result v128)
    (v128.load16_lane align=4 0
      (i32.const 0)
      (local.get $x)))
  (func (param $x v128) (result v128)
    (v128.load32_lane align=8 0
      (i32.const 0)
      (local.get $x)))
  (func (param $x v128) (result v128)
    (v128.load64_lane align=16 0
      (i32.const 0)
      (local.get $x)))
  (func (param $x v128)
    (v128.store8_lane align=2 0
      (i32.const 0)
      (local.get $x)))
  (func (param $x v128)
    (v128.store16_lane align=4 0
      (i32.const 0)
      (local.get $x)))
  (func (param $x v128)
    (v128.store32_lane align=8 0
      (i32.const 0)
      (local.get $x)))
  (func (param $x v128)
    (v128.store64_lane align=16 0
      (i32.const 0)
      (local.get $x)))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn offset() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 1)
  (func (drop (i32.load offset=4294967296 (i32.const 0))))
  (func
    i32.const 0
    i32.load offset=0xFFFF_FFFF_FFFF_FFFF
    drop
  )
  (func (drop (v128.load offset=4294967296 (i32.const 0))))
  (func (v128.store offset=4294967296 (i32.const 0) (v128.const i32x4 0 0 0 0)))

  (memory i64 1)
  (func (drop (i32.load 1 offset=4294967296 (i32.const 0))))

  ;; ignore undefined memory
  (func (drop (i32.load 2 offset=4294967296 (i32.const 0))))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
