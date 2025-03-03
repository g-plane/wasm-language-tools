(module
  (table funcref
    (elem
      $const-i32 $const-i64 $const-f32 $const-f64  ;; 0..3
      $id-i32 $id-i64 $id-f32 $id-f64              ;; 4..7
      $f32-i32 $i32-i64 $f64-f32 $i64-f64          ;; 9..11
      $fac-i64 $fib-i64 $even $odd                 ;; 12..15
      $runaway $mutual-runaway1 $mutual-runaway2   ;; 16..18
      $over-i32-duplicate $over-i64-duplicate      ;; 19..20
      $over-f32-duplicate $over-f64-duplicate      ;; 21..22
      $fac-i32 $fac-f32 $fac-f64                   ;; 23..25
      $fib-i32 $fib-f32 $fib-f64                   ;; 26..28
      $const-f64-i32 $id-i32-f64 $swap-i32-i64     ;; 29..31
    )
  )
)

(module
  (table 1 funcref)
  (func unreachable call_indirect)
  (func unreachable call_indirect nop)
  (func unreachable call_indirect call_indirect)
  (func unreachable call_indirect (call_indirect))
  (func unreachable call_indirect call_indirect call_indirect)
  (func unreachable call_indirect (result))
  (func unreachable call_indirect (result) (result))
  (func unreachable call_indirect (result) (result) call_indirect)
  (func unreachable call_indirect (result) (result) call_indirect (result))
  (func (result i32) unreachable call_indirect select)
  (func (result i32) unreachable call_indirect select call_indirect)
)
