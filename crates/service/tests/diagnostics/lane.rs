use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn simd() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result i32) (i8x16.extract_lane_s 16 (v128.const i8x16 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0)))
  (func (result i32) (i8x16.extract_lane_s 255 (v128.const i8x16 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0)))
  (func (result i32) (i8x16.extract_lane_u 16 (v128.const i8x16 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0)))
  (func (result i32) (i8x16.extract_lane_u 255 (v128.const i8x16 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0)))
  (func (result i32) (i16x8.extract_lane_s 8 (v128.const i16x8 0 0 0 0 0 0 0 0)))
  (func (result i32) (i16x8.extract_lane_s 255 (v128.const i16x8 0 0 0 0 0 0 0 0)))
  (func (result i32) (i16x8.extract_lane_u 8 (v128.const i16x8 0 0 0 0 0 0 0 0)))
  (func (result i32) (i16x8.extract_lane_u 255 (v128.const i16x8 0 0 0 0 0 0 0 0)))
  (func (result i32) (i32x4.extract_lane 4 (v128.const i32x4 0 0 0 0)))
  (func (result i32) (i32x4.extract_lane 255 (v128.const i32x4 0 0 0 0)))
  (func (result f32) (f32x4.extract_lane 4 (v128.const f32x4 0 0 0 0)))
  (func (result f32) (f32x4.extract_lane 255 (v128.const f32x4 0 0 0 0)))
  (func (result v128) (i8x16.replace_lane 16 (v128.const i8x16 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0) (i32.const 1)))
  (func (result v128) (i8x16.replace_lane 255 (v128.const i8x16 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0) (i32.const 1)))
  (func (result v128) (i16x8.replace_lane 16 (v128.const i16x8 0 0 0 0 0 0 0 0) (i32.const 1)))
  (func (result v128) (i16x8.replace_lane 255 (v128.const i16x8 0 0 0 0 0 0 0 0) (i32.const 1)))
  (func (result v128) (i32x4.replace_lane 4 (v128.const i32x4 0 0 0 0) (i32.const 1)))
  (func (result v128) (i32x4.replace_lane 255 (v128.const i32x4 0 0 0 0) (i32.const 1)))
  (func (result v128) (f32x4.replace_lane 4 (v128.const f32x4 0 0 0 0) (f32.const 1)))
  (func (result v128) (f32x4.replace_lane 255 (v128.const f32x4 0 0 0 0) (f32.const 1)))
  (func (result i64) (i64x2.extract_lane 2 (v128.const i64x2 0 0)))
  (func (result i64) (i64x2.extract_lane 255 (v128.const i64x2 0 0)))
  (func (result f64) (f64x2.extract_lane 2 (v128.const f64x2 0 0)))
  (func (result f64) (f64x2.extract_lane 255 (v128.const f64x2 0 0)))
  (func (result v128) (i64x2.replace_lane 2 (v128.const i64x2 0 0) (i64.const 1)))
  (func (result v128) (i64x2.replace_lane 255 (v128.const i64x2 0 0) (i64.const 1)))
  (func (result v128) (f64x2.replace_lane 2 (v128.const f64x2 0 0) (f64.const 1)))
  (func (result v128) (f64x2.replace_lane 255 (v128.const f64x2 0 0) (f64.const 1.0)))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn load_store() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 1)
  (func (param $x v128) (result v128)
    (v128.load8_lane 0 offset=0 16
      (i32.const 0)
      (local.get $x)))
  (func (param $x v128) (result v128)
    (v128.load16_lane offset=0 8
      (i32.const 0)
      (local.get $x)))
  (func (param $x v128) (result v128)
    (v128.load32_lane offset=0 4
      (i32.const 0)
      (local.get $x)))
  (func (param $x v128) (result v128)
    (v128.load64_lane offset=0 2
      (i32.const 0)
      (local.get $x)))
  (func (param $x v128)
    (v128.store8_lane offset=0 16
      (i32.const 0)
      (local.get $x)))
  (func (param $x v128)
    (v128.store16_lane offset=0 8
      (i32.const 0)
      (local.get $x)))
  (func (param $x v128)
    (v128.store32_lane offset=0 4
      (i32.const 0)
      (local.get $x)))
  (func (param $x v128)
    (v128.store64_lane 0 offset=0 2
      (i32.const 0)
      (local.get $x))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn uint() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory 1)
  (func (param $x v128)
    (v128.store8_lane offset=0 -1
      (i32.const 0)
      (local.get $x)))
  (func (param $x v128)
    (v128.store8_lane offset=0 999999999999999999999999999999
      (i32.const 0)
      (local.get $x)))
  (func (param $x v128)
    (v128.store8_lane offset=0 0xF
      (i32.const 0)
      (local.get $x))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn i8x16_shuffle() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (result v128)
    (i8x16.shuffle 0 1 2 3 4 5 6 7 8 9 10 11 12 13 32 255
      (v128.const i8x16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0)
      (v128.const i8x16 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
