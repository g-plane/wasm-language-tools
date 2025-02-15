(module
  (rec)
  (rec
    (type $s0 (struct (field (ref 0))))
    (type $s1 (struct (field (ref 0))))
  )
  (rec (type $r (sub $t (struct (field (ref $r))))))
  (rec
    (type $t1 (sub (func (param i32 (ref $t3)))))
    (type $t2 (sub $t1 (func (param i32 (ref $t2)))))
  )
)
