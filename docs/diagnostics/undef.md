# Undefined

This checks items that are used but haven't been defined. It checks for:

- function calls
- parameter usage
- function local usage
- type use
- global usage
- memory usage
- table usage
- block label usage
- WasmGC struct field usage

Examples:

```wasm error-3-8-3-9 error-5-15-5-16 error-6-10-6-22 error-9-16-9-28
(module
  (func
    br 1)
  (func
    local.get 0
    call $not-defined)
  (func
    i32.const 0
    global.set $not-defined))
```
