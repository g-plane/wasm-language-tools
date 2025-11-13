# Deprecation

Module fields can be marked as deprecated by adding the `@deprecated` annotation above corresponding module fields:

```wasm
(module
  (@deprecated)
  (func))
```

An optional reason can be added after the `@deprecated` annotation:

```wasm
(module
  (@deprecated "This function is deprecated because...")
  (func))
```

and the reason will be showed in diagnostic message.

Functions, type definitions, globals, memories, tables and tags can be marked as deprecated.

For type definitions inside `(rec)`, `@deprecated` annotation should be added above `(type)`:

```wasm
(module
  (rec
    (@deprecated)
    (type (func))))
```

For marking imported items as deprecated, `@deprecated` annotation should be added above `(import)`:

```wasm
(module
  (@deprecated)
  (import "mod" "func" (func)))
```

When using a deprecated item, a diagnostic with strikethrough will be showed in editor where it's used.
Strikethrough will also be showed in completion list and symbol list.
