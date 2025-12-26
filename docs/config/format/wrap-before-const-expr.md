# `wrapBeforeConstExpr`

> default: `"always"`

Control whether to insert line break before constant expression.

Available option values:

- `"never"`
- `"overflow"`
- `"multi-only"`
- `"always"`

All the examples below assume the print width is `60`.

## `"never"`

Line wrap will never be happened before constant expression.

Though possible, using `"never"` is not recommended.

```wasm
(module
  (global i32 i32.const 0)
  (global i32 (i32.add (i32.const 0) (i32.const 0)))
  (global i32 i32.const 0 i32.const 0 i32.add)
  (global (ref null func) ref.func $long-long-long-long-func-name))
```

<CenteredArrowDown />

```wasm
(module
  (global i32 i32.const 0)
  (global i32 (i32.add
      (i32.const 0)
      (i32.const 0)))
  (global i32 i32.const 0
    i32.const 0
    i32.add)
  (global (ref null func) ref.func $long-long-long-long-func-name))
```

## `"overflow"`

Line wrap will be happened before constant expression only when previous code exceeds the print width.

```wasm
(module
  (global i32 i32.const 0)
  (global i32 (i32.add (i32.const 0) (i32.const 0)))
  (global i32 i32.const 0 i32.const 0 i32.add)
  (global (ref null func) ref.func $long-long-long-long-func-name))
```

<CenteredArrowDown />

```wasm
(module
  (global i32 i32.const 0)
  (global i32 (i32.add
      (i32.const 0)
      (i32.const 0)))
  (global i32 i32.const 0
    i32.const 0
    i32.add)
  (global (ref null func)
    ref.func $long-long-long-long-func-name))
```

## `"multi-only"`

Line wrap will be happened before constant expression only when there are more than one instruction.
Folded instruction with children instructions is considered as multiple instructions.

```wasm
(module
  (global i32 i32.const 0)
  (global i32 (i32.add (i32.const 0) (i32.const 0)))
  (global i32 i32.const 0 i32.const 0 i32.add)
  (global (ref null func) ref.func $long-long-long-long-func-name))
```

<CenteredArrowDown />

```wasm
(module
  (global i32 i32.const 0)
  (global i32
    (i32.add
      (i32.const 0)
      (i32.const 0)))
  (global i32
    i32.const 0
    i32.const 0
    i32.add)
  (global (ref null func) ref.func $long-long-long-long-func-name))
```

## `"always"`

Line wrap will always be happened before constant expression.

```wasm
(module
  (global i32 i32.const 0)
  (global i32 (i32.add (i32.const 0) (i32.const 0)))
  (global i32 i32.const 0 i32.const 0 i32.add)
  (global (ref null func) ref.func $long-long-long-long-func-name))
```

<CenteredArrowDown />

```wasm
(module
  (global i32
    i32.const 0)
  (global i32
    (i32.add
      (i32.const 0)
      (i32.const 0)))
  (global i32
    i32.const 0
    i32.const 0
    i32.add)
  (global (ref null func)
    ref.func $long-long-long-long-func-name))
```
