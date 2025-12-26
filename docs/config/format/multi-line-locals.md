# `multiLineLocals`

> default: `"never"`

Control how to insert whitespace between multiple locals in a function.

Available option values:

- `"never"`
- `"overflow"`
- `"smart"`
- `"always"`

All the examples below assume the print width is `45`.

## `"never"`

All locals will be printed in the same line, regardless of the print width.

```wasm
(module
  (func (local i32) (local i64) (local f32))
  (func (local i32)
    (local i64) (local f32))
  (func (local $1 i32) (local $2 i32) (local $3 i32))
  (func (local $1 i32)
    (local $2 i32) (local $3 i32)))
```

<CenteredArrowDown />

```wasm
(module
  (func (local i32) (local i64) (local f32))
  (func (local i32) (local i64) (local f32))
  (func
    (local $1 i32) (local $2 i32) (local $3 i32))
  (func
    (local $1 i32) (local $2 i32) (local $3 i32)))
```

## `"overflow"`

If failed to print all locals in the same line within the print width, each local will be printed in its own line.

```wasm
(module
  (func (local i32) (local i64) (local f32))
  (func (local i32)
    (local i64) (local f32))
  (func (local $1 i32) (local $2 i32) (local $3 i32))
  (func (local $1 i32)
    (local $2 i32) (local $3 i32)))
```

<CenteredArrowDown />

```wasm
(module
  (func (local i32) (local i64) (local f32))
  (func (local i32) (local i64) (local f32))
  (func (local $1 i32)
    (local $2 i32)
    (local $3 i32))
  (func (local $1 i32)
    (local $2 i32)
    (local $3 i32)))
```

## `"smart"`

`"smart"` will detect if there's a line break between the first local and the second local in original code.

If there's a line break, it behaves like `"always"`, though it doesn't exceed the print width:

```wasm
(module
  (func (local i32)
    (local i64) (local f32)))
```

<CenteredArrowDown />

```wasm
(module
  (func (local i32)
    (local i64)
    (local f32)))
```

If there's no line break, it behaves like `"overflow"`, even if there're line breaks between the rest locals:

```wasm
(module
  (func (local i32) (local i64)
    (local f32))
  (func (local $1 i32) (local $2 i32) (local $3 i32)))
```

<CenteredArrowDown />

```wasm
(module
  (func (local i32) (local i64) (local f32))
  (func (local $1 i32)
    (local $2 i32)
    (local $3 i32)))
```

## `"always"`

All locals will be printed in their own lines.

```wasm
(module
  (func (local i32) (local i64) (local f32))
  (func (local i32)
    (local i64) (local f32))
  (func (local $1 i32) (local $2 i32) (local $3 i32))
  (func (local $1 i32)
    (local $2 i32) (local $3 i32)))
```

<CenteredArrowDown />

```wasm
(module
  (func (local i32)
    (local i64)
    (local f32))
  (func (local i32)
    (local i64)
    (local f32))
  (func (local $1 i32)
    (local $2 i32)
    (local $3 i32))
  (func (local $1 i32)
    (local $2 i32)
    (local $3 i32)))
```
