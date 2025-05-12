# Lint

Each lint has these levels:

- `"allow"` - don't report problems
- `"hint"` - report problems as slight tips
- `"warning"` - report problems as warnings
- `"deny"` - report problems as errors

## `unused`

> default: `"warning"`

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

## `shadow`

> default: `"warning"`

WebAssembly allows shadowing identifiers in block labels. For example:

```wasm:line-numbers warning-3-12-3-18
(module
  (func
    (block $label
      (block $label
        br $label))))
```

However, it's confusing that you may want to jump to the label at line 3 or line 4.
This lint reports such cases when you accidentally give the same name to different block labels.

Same name but not being as inner block is not reported, such as:

```wasm
(module
  (func
    (block $label
      br $label)
    (block $label
      br $label)))
```

## `implicitModule`

> default: `"allow"`

[WABT](https://github.com/WebAssembly/wabt) allows defining module fields without the outer `(module)` syntax:

```wasm
(func)
(global i32
  i32.const 0)
```

However, you may hope it to be more explicit, or your compiler doesn't support this syntax.
In such cases, you can set this lint to `"warning"` or `"deny"`.

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

## `needlessMut`

> default: `"warning"`

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

## `multiMemories`

> default: `"allow"`

WebAssembly Language Tools supports [Multiple Memories](https://github.com/WebAssembly/multi-memory/blob/master/proposals/multi-memory/Overview.md) proposal by default:

```wasm
(module
  (memory 1)
  (memory 2))
```

If your compiler or environment doesn't support this proposal, you can set this lint to `"deny"`.
