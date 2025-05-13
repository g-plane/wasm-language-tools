# Unreachable

> [!TIP]
> This can be configured. Read more about [configuration](../config/lint.md#unreachable).

This detects unreachable code, for example those code after `br`, `br_table`, `return` and `unreachable` instructions or infinite loops:

```wasm faded-5-5-5-8 faded-8-5-8-8
(module
  (func
    (loop ;; infinite loop
      br 0)
    nop)
  (func
    return
    nop))
```

```wasm faded-4-6-4-13 faded-8-7-10-10
(module
  (func
    (nop)
    (i32.add
      (nop)
      (unreachable
        (i32.const 1))
      (i32.const 0))
    (drop)
    (nop)))
```
