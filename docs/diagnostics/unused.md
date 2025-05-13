# Unused

> [!TIP]
> This can be configured. Read more about [configuration](../config/lint.md#unused).

This checks items that are defined but never used. It checks for:

- functions
- parameters
- function locals
- type definitions
- globals
- memories
- tables
- WasmGC struct fields

Here is an example of unused functions and globals with or without identifiers:

```wasm warning-2-4-2-8 warning-3-9-3-11 warning-4-4-4-10 warning-6-11-6-13
(module
  (func)
  (func $f)
  (global i32
    i32.const 0)
  (global $g i32
    i32.const 0))
```

To intentionally ignore, you can add an underscore prefix to the identifier:

```wasm
(module
  (func $_)
  (func $_f))
```

If they're exported, they won't be reported as unused:

```wasm
(module
  (func (export "main")))
```
