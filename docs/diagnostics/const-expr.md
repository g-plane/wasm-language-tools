# Constant Expression

For the initialization expressions of globals, tables, offsets and element segments, they must be constant.

To be a constant expression, it must only contain the following instructions:

- `*.const`
- `i32.add`
- `i32.sub`
- `i32.mul`
- `i64.add`
- `i64.sub`
- `i64.mul`
- `global.get`
- `ref.null`
- `ref.i31`
- `ref.func`
- `struct.new`
- `struct.new_default`
- `array.new`
- `array.new_default`
- `array.new_fixed`
- `any.convert_extern`
- `extern.convert_any`

Constant expression example:

```wasm
(module
  (global i32
    i32.const 2))
```

Non-constant expression example:

```wasm error-5-5-5-12
(module
  (global f32
    f32.const 1.0
    f32.const 2.0
    f32.add))
```
