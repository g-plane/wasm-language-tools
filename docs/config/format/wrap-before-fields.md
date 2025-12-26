# `wrapBeforeFields`

> default: `"multi-only"`

Control whether to insert line break before struct fields.

Available option values:

- `"never"`
- `"overflow"`
- `"multi-only"`
- `"always"`

All the examples below assume the print width is `50`.

## `"never"`

Line wrap will never be happened before struct fields.

```wasm
(module
  (type (struct (field i32 (ref any))))
  (type (struct (field i32) (field f64)))
  (type (sub final $long-type-name (struct (field i32)))))
```

<CenteredArrowDown />

```wasm
(module
  (type (struct (field i32 (ref any))))
  (type (struct (field i32) (field f64)))
  (type
    (sub final $long-type-name (struct (field i32)))))
```

## `"overflow"`

Line wrap will be happened before struct fields only when previous code exceeds the print width.

```wasm
(module
  (type (struct (field i32 (ref any))))
  (type (struct (field i32) (field f64)))
  (type (sub final $long-type-name (struct (field i32)))))
```

<CenteredArrowDown />

```wasm
(module
  (type (struct (field i32 (ref any))))
  (type (struct (field i32) (field f64)))
  (type
    (sub final $long-type-name
      (struct (field i32)))))
```

## `"multi-only"`

Line wrap will be happened before struct fields only when there are more than one field.
Note that `(field i32 i64)` is considered as one field syntax.

```wasm
(module
  (type (struct (field i32 (ref any))))
  (type (struct (field i32) (field f64)))
  (type (sub final $long-type-name (struct (field i32)))))
```

<CenteredArrowDown />

```wasm
(module
  (type (struct (field i32 (ref any))))
  (type
    (struct
      (field i32)
      (field f64)))
  (type
    (sub final $long-type-name (struct (field i32)))))
```

## `"always"`

Line wrap will always be happened before struct fields.

```wasm
(module
  (type (struct (field i32 (ref any))))
  (type (struct (field i32) (field f64)))
  (type (sub final $long-type-name (struct (field i32)))))
```

<CenteredArrowDown />

```wasm
(module
  (type
    (struct
      (field i32 (ref any))))
  (type
    (struct
      (field i32)
      (field f64)))
  (type
    (sub final $long-type-name
      (struct
        (field i32)))))
```
