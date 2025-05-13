# New Non-defaultable

When using `array.new_default` or `struct.new_default`, the given type must be defaultable.
This checks if that type is defaultable or not.

```wasm error-10-24-10-46 error-13-25-13-48
(module
  (type $defaultable-array (array i32))
  (type $non-defaultable-array (array (ref any)))
  (type $defaultable-struct (struct (field i32)))
  (type $non-defaultable-struct (struct (field (ref any))))
  (func
    (result (ref $defaultable-array) (ref $non-defaultable-array) (ref $defaultable-struct) (ref $non-defaultable-struct))
    (array.new_default $defaultable-array
      (i32.const 0))
    (array.new_default $non-defaultable-array
      (i32.const 0))
    (struct.new_default $defaultable-struct)
    (struct.new_default $non-defaultable-struct)))
```

For struct type, only when all fields are defaultable, the struct is defaultable:

```wasm error-7-25-7-26
(module
  (type
    (struct
      (field $defaultable-field i32)
      (field $non-defaultable-field (ref any))))
  (func (result (ref 0))
    (struct.new_default 0)))
```

Empty struct is defaultable.
