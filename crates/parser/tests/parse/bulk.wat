;; segment syntax
(module
  (memory 1)
  (data "foo"))

(module
  (table 3 funcref)
  (elem funcref (ref.func 0) (ref.null func) (ref.func 1))
  (func))

;; memory.fill
(module
  (func (export "fill") (param i32 i32 i32)
    (memory.fill
      (local.get 0)
      (local.get 1)
      (local.get 2)))
)


(module
  (data $p "x")
  (data $a (memory 0) (i32.const 0) "x")
)


(module
  (elem (i32.const 0) $zero $one $two)
)
