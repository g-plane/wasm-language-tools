use criterion::{Criterion, criterion_group, criterion_main};

static CODE: &str = r#"(module
    (func $f1 (param $p1 i32) (param $p2 i32) (result i32)
        (i32.add (local.get $p1) (local.get $p2))
    )
    (global $g1 f64 (f64.const 0))
    (func $f2 (result f64)
        (global.get $g1)
    )
    (type $t (func (result f64)))
    (func $f3 (type $t)
        (call $f2)
    )
    (func (export "f32.min_positive") (result i32) (i32.reinterpret_f32 (f32.const 0x1p-149)))
    (func (export "f32.min_normal") (result i32) (i32.reinterpret_f32 (f32.const 0x1p-126)))

    (rec (type $r (sub $t (struct (field (ref $r))))))
    (global (;7;) (mut f32) (f32.const -13))
    (rec
        (type $t1 (sub (func (param i32 (ref $t3)))))
        (type $t2 (sub $t1 (func (param i32 (ref $t2)))))
    )
    (global (;8;) (mut f64) (f64.const -14))

    (func (export "f32.max_finite") (result i32) (i32.reinterpret_f32 (f32.const 0x1.fffffep+127)))
    (func (export "f32.max_subnormal") (result i32) (i32.reinterpret_f32 (f32.const 0x1.fffffcp-127)))
)
"#;

fn bench_parser(c: &mut Criterion) {
    c.bench_function("parser", |b| {
        b.iter(|| {
            let _ = wat_parser::parse_to_green(CODE);
        });
    });
}

criterion_group!(benches, bench_parser);
criterion_main!(benches);
