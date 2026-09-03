use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::{fs, hint::black_box};

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
    let mut group = c.benchmark_group("parser");
    group.throughput(Throughput::Bytes(CODE.len() as u64));
    group.bench_function("900 bytes", |b| {
        b.iter(|| {
            black_box(wat_parser::parse(CODE));
        });
    });

    fs::read_dir("crates/parser/benches")
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "wat"))
        .for_each(|entry| {
            let path = entry.path();
            let code = fs::read_to_string(&path).unwrap();
            group.throughput(Throughput::Bytes(code.len() as u64));
            group.bench_function(
                format!("{} ({} bytes)", path.file_stem().unwrap().display(), code.len()),
                |b| {
                    b.iter(|| {
                        black_box(wat_parser::parse(&code));
                    });
                },
            );
        });
}

criterion_group!(benches, bench_parser);
criterion_main!(benches);
