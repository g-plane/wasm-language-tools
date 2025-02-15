(module
  (func)
  (func (export "f"))
  (func $f)
  (func $h (export "g"))

  (func (local))
  (func (local) (local))
  (func (local i32))
  (func (local $x i32))
  (func (local i32 f64 i64))
  (func (local i32) (local f64))
  (func (local i32 f32) (local $x i64) (local) (local i32 f64))

  (func (param))
  (func (param) (param))
  (func (param i32))
  (func (param $x i32))
  (func (param i32 f64 i64))
  (func (param i32) (param f64))
  (func (param i32 f32) (param $x i64) (param) (param i32 f64))

  (func (result))
  (func (result) (result))
  (func (result i32) (unreachable))
  (func (result i32 f64 f32) (unreachable))
  (func (result i32) (result f64) (unreachable))
  (func (result i32 f32) (result i64) (result) (result i32 f64) (unreachable))

  (type $sig-1 (func))
  (type $sig-2 (func (result i32)))
  (type $sig-3 (func (param $x i32)))
  (type $sig-4 (func (param i32 f64 i32) (result i32)))

  (func (export "type-use-1") (type $sig-1))
  (func (export "type-use-2") (type $sig-2) (i32.const 0))
  (func (export "type-use-3") (type $sig-3))
  (func (export "type-use-4") (type $sig-4) (i32.const 0))
  (func (export "type-use-5") (type $sig-2) (result i32) (i32.const 0))
  (func (export "type-use-6") (type $sig-3) (param i32))
  (func (export "type-use-7")
    (type $sig-4) (param i32) (param f64 i32) (result i32) (i32.const 0)
  )

  (func (type $sig))
  (func (type $forward))  ;; forward reference

  (func $complex
    (param i32 f32) (param $x i64) (param) (param i32)
    (result) (result i32) (result) (result i64 i32)
    (local f32) (local $y i32) (local i64 i32) (local) (local f64 i32)
    (unreachable) (unreachable)
  )
  (func $complex-sig
    (type $sig)
    (local f32) (local $y i32) (local i64 i32) (local) (local f64 i32)
    (unreachable) (unreachable)
  )

  (type $forward (func))

  (func (param (ref 0)))
  (func (result (ref 0)))
)
