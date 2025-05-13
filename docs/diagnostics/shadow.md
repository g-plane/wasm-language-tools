# Shadowing

> [!TIP]
> This can be configured. Read more about [configuration](../config/lint.md#shadow).

WebAssembly allows shadowing identifiers in block labels. For example:

```wasm :line-numbers warning-3-12-3-18
(module
  (func
    (block $label
      (block $label
        br $label))))
```

However, it's confusing that you may want to jump to the label at line 3 or line 4.
(Actually it jumps to the inner block label, which is line 4 in this case.)
This checks such cases when you accidentally give the same name to different block labels.

Same name but not being as inner block won't be reported, such as:

```wasm
(module
  (func
    (block $label
      br $label)
    (block $label
      br $label)))
```
