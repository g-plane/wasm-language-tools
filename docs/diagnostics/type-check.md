# Type Checking

Type checking will happen when using instructions and at the end of a block (including functions, global expressions, etc).

For instructions:

```wasm error-5-5-5-12 error-7-5-9-21
(module
  (func (result i32)
    i32.const 0
    i64.const 0
    i32.add)
  (func (result i32)
    (i32.add
      (i32.const 0)
      (i64.const 0))))
```

For the end of a block:

```wasm error-3-16-3-17 error-6-20-6-21
(module
  (func (param i64) (result i32)
    local.get 0)
  (func (result i32)
    (block (result i32)
      (i64.const 0))))
```
