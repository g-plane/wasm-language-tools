# `wrapBeforeLocals`

> default: `"overflow"`

Control whether to insert line break before function locals.

Available option values:

- `"never"`
- `"overflow"`
- `"multi-only"`
- `"always"`

## `"never"`

Line will never be wrapped before locals.

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

Line will be wrapped before locals only when it exceeds the print width.

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

Line will be wrapped before locals only when there are more than one local.
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

Line will always be wrapped before locals.

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
