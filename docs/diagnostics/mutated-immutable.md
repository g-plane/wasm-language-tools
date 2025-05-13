# Mutated Immutable

This checks for globals, WasmGC struct fields and arrays that are not declared mutable,
but are mutated unexpectedly.

Specifically:

- `global.set` will mutate a global
- `struct.set` will mutate a WasmGC struct field
- `array.set`, `array.fill`, `array.init_data` and `array.init_elem` will mutate a WasmGC array

Global types and field types aren't defined with a `mut` keyword are immutable.
It's invalid to mutate them.

## Globals

```wasm error-5-17-5-18
(module
  (global i32
    i32.const 0)
  (func
    (global.set 0
      (i32.const 0))))
```

## Struct Fields

```wasm error-6-18-6-19
(module
  (type (struct (field i32)))
  (func (param (ref 0))
    local.get 0
    i32.const 0
    struct.set 0 0))
```

## Arrays

```wasm error-7-15-7-16
(module
  (type (array i32))
  (func (param (ref 0))
    local.get 0
    i32.const 0
    i32.const 0
    array.set 0))
```
