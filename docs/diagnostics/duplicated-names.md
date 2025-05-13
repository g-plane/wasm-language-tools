# Duplicated Names

This checks duplicated identifiers in their same scope:

```wasm error-2-9-2-11 error-3-9-3-11
(module
  (func $f)
  (func $f))
```

Same identifier in different scopes is allowed:

```wasm
(module
  (func (param $p i32))
  (func (param $p i32)))
```

Though in the same scope, if two identifiers have same name but different kinds, it is allowed:

```wasm
(module
  (func $f)
  (type $f (func)))
```

In the example above, the first `$f` is funcidx while the second `$f` is typeidx.
