(module
  (type $e0 (sub (array i32)))
  (type $e1 (sub $e0 (array i32)))

  (type $e0 (sub (struct)))
  (type $e1 (sub $e0 (struct)))
  (type $e2 (sub $e1 (struct (field i32))))

  (type $t2 (sub final (func)))
)
