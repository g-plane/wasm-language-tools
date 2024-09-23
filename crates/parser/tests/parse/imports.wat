(module
  ;; Functions
  (import "spectest" "print_i32" (func (param i32)))
  (func (import "spectest" "print_i64") (param i64))
  (import "spectest" "print_i32" (func $print_i32 (param i32)))
  (func $print_i32-2 (import "spectest" "print_i32") (param i32))

  (func (export "p1") (import "spectest" "print_i32") (param i32))
  (func $p (export "p2") (import "spectest" "print_i32") (param i32))
  (func (export "p3") (export "p4") (import "spectest" "print_i32") (param i32))

  (import "spectest" "print_i32" (func (type $forward)))

  ;; Globals
  (import "spectest" "global_i32" (global i32))
  (global (import "spectest" "global_i32") i32)

  (import "spectest" "global_i32" (global $x i32))
  (global $y (import "spectest" "global_i32") i32)

  ;; Tables
  (import "spectest" "table" (table $tab 10 20 funcref))
  (table $tab (import "spectest" "table") 10 20 funcref)
  (import "test" "table-10-inf" (table 10 funcref))
  (import "spectest" "table" (table 0 funcref))

  ;; Memories
  (import "spectest" "memory" (memory 1 2))
  (memory (import "spectest" "memory") 1 2)
)
