# Needless Mutable

> [!TIP]
> This can be configured. Read more about [configuration](../config/lint.md#needlessmut).

This checks for globals, WasmGC struct fields and arrays that are declared mutable but are never mutated.

Specifically:

- `global.set` can mutate a global
- `struct.set` can mutate a WasmGC struct field
- `array.set`, `array.fill`, `array.init_data` and `array.init_elem` can mutate a WasmGC array

For those global types and field types that are defined with a `mut` keyword
but are never mutated by the instructions above, "mutable" is needless.

## Globals

```wasm warning-5-20-5-23
(module
  (func
    global.get 0
    drop)
  (global $global (mut i32)
    i32.const 0))
```

It won't report exported globals, since they may be mutated by the host.

```wasm
(module
  (export "globalValue" (global 0))
  (global (mut i32)
    i32.const 0))
```

## Struct Fields

```wasm warning-2-25-2-28
(module
  (type (struct (field (mut i32))))
  (func (param (ref 0)) (result i32)
    local.get 0
    struct.get 0 0))
```

## Arrays

```wasm warning-2-17-2-20
(module
  (type (array (mut i32)))
  (func (param (ref 0)) (result i32)
    local.get 0
    i32.const 0
    array.get 0))
```
