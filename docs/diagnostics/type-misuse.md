# Type Misuse

> [!NOTE]
> Please pay attention to distinguish it from [type checking](./type-check.md).
> Type checking ensures operand types that passed to instructions are correct,
> while this checks immediates.

WasmGC introduces struct types and array types, and also introduces new instructions like `array.*` and `struct.*`.
This checks if types that used in these instructions match the instruction or not.

## `array.*` and `struct.*`

For example, a struct type used in `array.*` instructions:

```wasm error-5-23-5-30
(module
  (type $struct (struct))
  (func
    i32.const 0
    array.new_default $struct
    drop))
```

Or, an array type used in `struct.*` instructions:

```wasm error-4-24-4-30
(module
  (type $array (array i32))
  (func
    struct.new_default $array
    drop))
```

## `array.copy`

Beside the checks above, `array.copy` also checks the source and destination types.
The source type must match the destination type.

```wasm error-10-5-10-37
(module
  (type $dst_array (array (mut i32)))
  (type $src_array (array i64))
  (func (param (ref $dst_array) (ref $src_array))
    local.get 0
    i32.const 0
    local.get 1
    i32.const 0
    i32.const 0
    array.copy $dst_array $src_array))
```

## `call_ref` and `return_call_ref`

These instructions require the referenced type must be a function type.

```wasm error-12-14-12-21 error-14-21-14-27
(module
  (type $func (func))
  (type $struct (struct (field i32)))
  (type $array (array (mut i32)))
  (func (param (ref $func))
    local.get 0
    call_ref $func
    local.get 0
    return_call_ref $func)
  (func (param (ref $func))
    local.get 0
    call_ref $struct
    local.get 0
    return_call_ref $array))
```

## `br_on_cast`

This instruction requires the last type of the label's types must be a ref type:

```wasm error-3-17-3-18
(module
  (func
    (br_on_cast 0 structref structref
      (unreachable))))
```

It requires the second ref type must match the first ref type:

```wasm error-3-29-3-37
(module
  (func (result anyref)
    (br_on_cast 0 structref arrayref
      (unreachable))))
```

It also requires the second ref type must match the last type of the label's types:

```wasm error-4-36-4-49
(module
  (func (param (ref any)) (result (ref $t))
    (block (result (ref any))
      (br_on_cast 1 (ref null any) (ref null $t)
        (local.get 0)))
    (unreachable)))
```

## `br_on_cast_fail`

This instruction requires the last type of the label's types must be a ref type:

```wasm error-3-22-3-23
(module
  (func
    (br_on_cast_fail 0 structref structref
      (unreachable))))
```

It requires the second ref type must match the first ref type:

```wasm error-3-34-3-42
(module
  (func (result anyref)
    (br_on_cast_fail 0 structref arrayref
      (unreachable))))
```

It also requires the [type difference](https://webassembly.github.io/gc/core/valid/conventions.html#aux-reftypediff)
between the first ref type and the second ref type must match the last type of the label's types:

```wasm error-4-7-5-23
(module
  (func (param (ref null any)) (result (ref any))
    (block (result (ref $t))
      (br_on_cast_fail 1 (ref null any) (ref $t)
        (local.get 0)))))
```
