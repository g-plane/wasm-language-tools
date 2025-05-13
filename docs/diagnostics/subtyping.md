# Subtyping

This checks subtyping relationships when defining types.

## Super Type's Typeidx

Super type's typeidx must be smaller than the sub type's typeidx:

```wasm error-2-14-2-15 error-5-17-5-19
(module
  (type (sub 1 (func)))
  (type (sub (func)))

  (type $z (sub $a (func)))
  (type $a (sub (func))))
```

## Super Type Can't Be Final

Super type can't be implicitly or explicitly final:

```wasm error-3-17-3-19
(module
  (type $t (func)) ;; implicitly final
  (type $s (sub $t (func))))
```

```wasm error-3-17-3-19
(module
  (type $t (sub final (func))) ;; explicitly final
  (type $s (sub $t (func))))
```

## Type Matching

The sub type must match the super type:

```wasm error-3-18-3-21
(module
  (type $a0 (sub (array i32)))
  (type $s0 (sub $a0 (struct))))
```
