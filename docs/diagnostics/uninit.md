# Uninitialized

This checks when accessing function locals, if they are initialized or not.

Number types, vector types and nullable reference types are considered "defaultable", so they don't require manual initialization.
For non-nullable reference types, they must be initialized by `local.set` or `local.tee` instructions.

```wasm error-5-15-5-16
(module
  (func (result i32) (local i32)
    local.get 0)
  (func (result (ref any)) (local (ref any))
    local.get 0))
```
