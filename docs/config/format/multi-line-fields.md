# `multiLineFields`

> default: `"smart"`

Control how to insert whitespace between multiple fields in a struct.

Available option values:

- `"never"`
- `"overflow"`
- `"smart"`
- `"always"`

All the examples below assume the print width is `60`.

## `"never"`

All fields will be printed in the same line, regardless of the print width.

```wasm
(module
  (type (struct (field i32) (field i64)
    (field f32)))
  (type (struct (field i32)
    (field i64) (field f32)))
  (type (struct (field $1 i32) (field $2 i32) (field $3 i32) (field $4 i32)))
  (type (struct (field $1 i32)
    (field $2 i32) (field $3 i32) (field $4 i32))))
```

<CenteredArrowDown />

```wasm
(module
  (type
    (struct
      (field i32) (field i64) (field f32)))
  (type
    (struct
      (field i32) (field i64) (field f32)))
  (type
    (struct
      (field $1 i32) (field $2 i32) (field $3 i32) (field $4 i32)))
  (type
    (struct
      (field $1 i32) (field $2 i32) (field $3 i32) (field $4 i32))))
```

## `"overflow"`

If failed to print all fields in the same line within the print width, each field will be printed in its own line.

```wasm
(module
  (type (struct (field i32) (field i64)
    (field f32)))
  (type (struct (field i32)
    (field i64) (field f32)))
  (type (struct (field $1 i32) (field $2 i32) (field $3 i32) (field $4 i32)))
  (type (struct (field $1 i32)
    (field $2 i32) (field $3 i32) (field $4 i32))))
```

<CenteredArrowDown />

```wasm
(module
  (type
    (struct
      (field i32) (field i64) (field f32)))
  (type
    (struct
      (field i32) (field i64) (field f32)))
  (type
    (struct
      (field $1 i32)
      (field $2 i32)
      (field $3 i32)
      (field $4 i32)))
  (type
    (struct
      (field $1 i32)
      (field $2 i32)
      (field $3 i32)
      (field $4 i32))))
```

## `"smart"`

`"smart"` will detect if there's a line break between the first field and the second field in original code.

If there's a line break, it behaves like `"always"`, though it doesn't exceed the print width:

```wasm
(module
  (type (struct (field i32)
    (field i64) (field f32))))
```

<CenteredArrowDown />

```wasm
(module
  (type
    (struct
      (field i32)
      (field i64)
      (field f32))))
```

If there's no line break, it behaves like `"overflow"`, even if there're line breaks between the rest fields:

```wasm
(module
  (type (struct (field i32) (field i64)
    (field f32)))
  (type (struct (field $1 i32) (field $2 i32) (field $3 i32) (field $4 i32))))
```

<CenteredArrowDown />

```wasm
(module
  (type
    (struct
      (field i32) (field i64) (field f32)))
  (type
    (struct
      (field $1 i32)
      (field $2 i32)
      (field $3 i32)
      (field $4 i32))))
```

## `"always"`

All fields will be printed in their own lines.

```wasm
(module
  (type (struct (field i32) (field i64)
    (field f32)))
  (type (struct (field i32)
    (field i64) (field f32)))
  (type (struct (field $1 i32) (field $2 i32) (field $3 i32) (field $4 i32)))
  (type (struct (field $1 i32)
    (field $2 i32) (field $3 i32) (field $4 i32))))
```

<CenteredArrowDown />

```wasm
(module
  (type
    (struct
      (field i32)
      (field i64)
      (field f32)))
  (type
    (struct
      (field i32)
      (field i64)
      (field f32)))
  (type
    (struct
      (field $1 i32)
      (field $2 i32)
      (field $3 i32)
      (field $4 i32)))
  (type
    (struct
      (field $1 i32)
      (field $2 i32)
      (field $3 i32)
      (field $4 i32))))
```
