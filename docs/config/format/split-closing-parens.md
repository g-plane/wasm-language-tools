# `splitClosingParens`

> default: `false`

Control whether closing parentheses should be splitted into different lines.

## `false`

```wasm
(module
  (func
    (block)))
```

## `true`

```wasm
(module
  (func
    (block)
  )
)
```
