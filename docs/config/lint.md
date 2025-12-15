# Lint

Each lint has these levels:

- `"allow"` - don't report problems
- `"hint"` - report problems as slight tips
- `"warn"` - report problems as warnings
- `"deny"` - report problems as errors

## `unused`

> default: `"warn"`

This lint reports unused items, such as functions, globals, types, and even function parameters and locals.
For WasmGC, fields in struct type are also supported.

Here is an example of unused functions and globals with or without identifiers:

```wasm warning-2-4-2-8 warning-3-9-3-11 warning-4-4-4-10 warning-6-11-6-13
(module
  (func)
  (func $f)
  (global i32
    i32.const 0)
  (global $g i32
    i32.const 0))
```

Sometimes, there may be unused items but you want to ignore them intentionally,
then you can add an underscore prefix to the identifier:

```wasm
(module
  (func $_)
  (func $_f))
```

So they won't be reported as unused.

## `unread`

> default: `"warn"`

This lint reports function locals that are set with new values but never read afterwards.

```wasm warning-3-16-3-17
(module
  (func (result i32) (local i32)
    (local.set 0 (i32.const 1))
    (local.set 0 (i32.const 2))
    (local.get 0)))
```

## `shadow`

> default: `"warn"`

WebAssembly allows shadowing identifiers in block labels. For example:

```wasm :line-numbers warning-3-12-3-18
(module
  (func
    (block $label
      (block $label
        br $label))))
```

However, it's confusing that you may want to jump to the label at line 3 or line 4.
This lint reports such cases when you accidentally give the same name to different block labels.

## `implicitModule`

> default: `"allow"`

[WABT](https://github.com/WebAssembly/wabt) allows defining module fields without the outer `(module)` syntax:

```wasm
(func)
(global i32
  i32.const 0)
```

However, you may hope it to be more explicit, or your compiler doesn't support this syntax.
In such cases, you can set this lint to `"warn"` or `"deny"`.

## `multiModules`

> default: `"deny"`

Most compilers (such as [WABT](https://github.com/WebAssembly/wabt)) only allow one module in a file.

```wasm error-2-1-2-9
(module)
(module)
```

If this doesn't match your needs, you can set this lint to `"allow"`.

## `unreachable`

> default: `"hint"`

This lint reports unreachable code, for example those code after `br`, `return` and `unreachable` instructions or infinite loops:

```wasm faded-5-5-5-8 faded-8-5-8-8
(module
  (func
    (loop
      br 0)
    nop)
  (func
    return
    nop))
```

## `deprecated`

> default: `"warn"`

This lint reports usages of deprecated items which are marked with the [`@deprecated` annotation](../guide/deprecation.md).

```wasm warning-5-10-5-15 strikethrough-5-10-5-15
(module
  (@deprecated)
  (func $func)
  (func
    call $func))
```

## `needlessMut`

> default: `"warn"`

This lint reports mutable items that are never mutated:

```wasm warning-2-12-2-15
(module
  (global (mut i32)
    i32.const 0)
  (func (result i32)
    global.get 0))
```

For WasmGC, fields in struct type and array type is also supported:

```wasm warning-2-24-2-27 warning-3-33-3-36
(module
  (type $array (array (mut i32)))
  (type $struct (struct (field (mut i32))))
  (func (param (ref $array) (ref $struct)) (result i32 i32)
    local.get 0
    i32.const 0
    array.get $array
    local.get 1
    struct.get $struct 0))
```

## `needlessTryTable`

> default: `"warn"`

A `try_table` instruction without `catch` or `catch_all` clauses behaves like a `block` instruction, which is unnecessary. Any exception thrown inside such `try_table` will be propagated.

This lint reports such cases:

```wasm warning-4-5-4-14 faded-4-5-4-14
(module
  (tag)
  (func
    try_table
      throw 0
    end))
```

## `uselessCatch`

> default: `"warn"`

This lint reports catch clauses that will never be matched.

When there're multiple handlers for the same tag, only the first one will be matched:

```wasm warning-5-29-5-41 faded-5-29-5-41
(module
  (tag $e)
  (func
    block
      try_table (catch 0 0) (catch $e 1)
      end
    end))
```

Any catch clauses after a `catch_all` clause will never be matched:

```wasm warning-5-31-5-43 faded-5-31-5-43
(module
  (tag $e)
  (func
    block
      try_table (catch_all 0) (catch $e 1)
      end
    end))
```
