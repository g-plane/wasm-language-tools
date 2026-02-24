# Unread Locals

> [!TIP]
> This can be configured. Read more about [configuration](../config/lint.md#unread).

This checks function locals that are set by `local.set` or `local.tee` but never read with `local.get`.

```wasm warning-5-15-5-16 warning-8-16-8-17
(module
  (func
    (local i32)
    i32.const 1
    local.set 0)
  (func (result i32)
    (local i32)
    (local.set 0
      (i32.const 0))
    (local.set 0
      (i32.const 1))
    (local.get 0)))
```

If there's `local.set` or `local.tee` after `local.get` but no more `local.get` after that, it will also be considered as unread.

```wasm warning-8-16-8-17
(module
  (func
    (local i32)
    (local.set 0
      (i32.const 1))
    (drop
      (local.get 0))
    (local.set 0
      (i32.const 0))))
```
