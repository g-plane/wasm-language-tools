(module
  (global i32 i32.const 0)
  (global i32 (i32.add (i32.const 0) (i32.const 0)))
  (global i32 i32.const 0 i32.const 0 i32.add)
  (global (ref null func) ref.func $long-long-long-long-func-name)

  (table funcref (elem (ref.func $f)))
  (table funcref (elem (item (ref.func $f1) (ref.func $f2))))
  (table (ref null func) (elem (item (ref.func $long-long-func-name))))

  (table 0 funcref (ref.func $f))
  (table 0 funcref (ref.func $f1) (ref.func $f2))
  (table 0 (ref null func) (ref.func $long-long-long-func-name))

  (data (memory 0) (offset i32.const 0))
  (data (memory 0) (offset i32.const 0 i32.const 0))
  (data (memory $long-long-memory-name) (offset i32.const 0))
)
