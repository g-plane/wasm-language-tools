# `wrapBeforeLocals`

> default: `"overflow"`

Control whether to insert line break before function locals.

Available option values:

- `"never"`
- `"overflow"`
- `"multi-only"`
- `"always"`

All the examples below assume the print width is `40`.

## `"never"`

Line wrap will never be happened before locals.

```wasm
(module
  (func (param)(local i32 (ref any)))
  (func (param) (local i32) (local f64))
  (func (param) (param i32) (result i32)
    (local i32)))
```

<CenteredArrowDown />

```wasm
(module
  (func (param) (local i32 (ref any)))
  (func (param) (local i32) (local f64))
  (func (param) (param i32) (result i32) (local i32)))
```

## `"overflow"`

Line wrap will be happened before locals only when previous code exceeds the print width.

```wasm
(module
  (func (param) (local i32) (local f64))
  (func (param) (param i32) (result i32) (local i32)))
```

<CenteredArrowDown />

```wasm
(module
  (func (param) (local i32) (local f64))
  (func (param) (param i32) (result i32)
    (local i32)))
```

## `"multi-only"`

Line wrap will be happened before locals only when there are more than one local.
Note that `(local i32 i64)` is considered as one local syntax.

```wasm
(module
  (func (param) (local i32 (ref any)))
  (func (param) (local i32) (local f64)))
```

<CenteredArrowDown />

```wasm
(module
  (func (param) (local i32 (ref any)))
  (func (param)
    (local i32) (local f64)))
```

## `"always"`

Line wrap will always be happened before locals.

```wasm
(module
  (func (param) (local i32 (ref any)))
  (func (param) (local i32) (local f64))
  (func (param) (param i32) (result i32) (local i32)))
```

<CenteredArrowDown />

```wasm
(module
  (func (param)
    (local i32 (ref any)))
  (func (param)
    (local i32) (local f64))
  (func (param) (param i32) (result i32)
    (local i32)))
```
