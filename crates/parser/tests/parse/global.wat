(module
  (global (import "spectest" "global_i32") i32)
  (global (import "spectest" "global_i64") i64)

  (global $a i32 (i32.const -2))
  (global (;3;) f32 (f32.const -3))
  (global (;4;) f64 (f64.const -4))
  (global $b i64 (i64.const -5))

  (global $x (mut i32) (i32.const -12))
  (global (;7;) (mut f32) (f32.const -13))
  (global (;8;) (mut f64) (f64.const -14))
  (global $y (mut i64) (i64.const -15))

  (global $z1 i32 (global.get 0))
  (global $z2 i64 (global.get 1))

  (global $r externref (ref.null extern))
  (global $mr (mut externref) (ref.null extern))
  (global funcref (ref.null func))
)