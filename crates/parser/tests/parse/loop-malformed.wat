(module
  (func (i32.const 0) (loop (type $sig) (result i32) (param i32)))
  (func (i32.const 0) (loop (param i32) (type $sig) (result i32)))
  (func (i32.const 0) (loop (param i32) (result i32) (type $sig)))
  (func (i32.const 0) (loop (result i32) (type $sig) (param i32)))
  (func (i32.const 0) (loop (result i32) (param i32) (type $sig)))
  (func (i32.const 0) (loop (result i32) (param i32)))
  (func (i32.const 0) (loop (param $x i32) (drop)))
)