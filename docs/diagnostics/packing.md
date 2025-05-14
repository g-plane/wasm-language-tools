# Packed Type

WasmGC introduces packed types, which are `i8` and `i16`.
They can only exist in array and struct field types.
When taking them out of fields, it's required that using special `*.get_s` or `*.get_u` instructions.

## Misuse of `*.get`

It's invalid to use `*.get` instructions to take out packed types.

```wasm error-5-18-5-19
(module
  (type (struct (field i8)))
  (func (param (ref 0)) (result i32)
    local.get 0
    struct.get 0 0))
```

Quick fixes are available for replacing `*.get` with `*.get_s` or `*.get_u`.

## Misuse of `*.get_s` or `*.get_u`

It's invalid to use `*.get_s` or `*.get_u` instructions to take out normal value types.

```wasm error-5-20-5-21 error-8-20-8-21
(module
  (type (struct (field i32)))
  (func (param (ref 0)) (result i32)
    local.get 0
    struct.get_s 0 0)
  (func (param (ref 0)) (result i32)
    local.get 0
    struct.get_u 0 0))
```

Quick fixes are available for replacing `*.get_s` or `*.get_u` with `*.get`.
