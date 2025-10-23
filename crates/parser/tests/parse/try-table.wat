(module
  (func
    (try_table)
    (try_table (result i32) (catch $e0 $h))
    (try_table (catch_all $h) (unreachable))
    (try_table (result i32) (catch $e0 $h0) (catch $e1 $h1))
    (try_table (result f32) (catch_ref $e-f32 $h))
    (try_table (catch_all 0) (catch_all_ref 0))
  )
)
