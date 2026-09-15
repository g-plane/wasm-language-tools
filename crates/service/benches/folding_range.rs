use criterion::{Criterion, criterion_group, criterion_main};
use lspt::{FoldingRangeParams, TextDocumentIdentifier};
use std::hint::black_box;
use wat_service::LanguageService;

pub fn folding_range_bench(c: &mut Criterion) {
    let source = r#"
(module
  (global i32
    i32.const 0)
  (func (param (ref any)) (result (ref any))
    (local (ref any))
    (block $b
      (loop $loop
        (if
          (then
            (br $b)
            (local.set 1
              (local.get 0)))
          (else
            (local.set 1
              (local.get 0))
            (br $loop)))))
    (local.get 1))

  (func
    i32.load
    i32.const 0
    i32.load 0)

  (func (param v128)
    v128.load8_lane
    struct.new
    drop)

  (func (param $a i32)
    (local $a f32))

  (func
    (global.set 0
      (i32.const 0)))

  (type (array (mut i32)))
  (func (param (ref 0)) (result i32)
    local.get 0
    i32.const 0
    array.get 0)

  (func
    loocalget
    local.get 6)

  (func
    (local i32)
    (local.set 0
      (i32.const 0))
    (if
      (i32.const 0)
      (then
        (local.set 0
          (i32.const 1))
        (drop
          (nop
            (local.get 0))))
      (else
        (local.set 0
          (i32.const 2)))))

  (type $a (sub (struct (field (mut (ref null any))))))
  (type $b (sub $a (struct (field (mut (ref null none))))))

  (type $dst_array (array (mut i32)))
  (type $src_array (array i64))
  (func (param (ref $dst_array) (ref $src_array))
    local.get 0
    i32.const 0
    local.get 1
    i32.const 0
    i32.const 0
    array.copy $struct $struct)

  (func
    memory.init
    array.new_fixed
    i8x16.shuffle
    f32.sub)

  a

  (func (export "func")))
"#;
    let uri = "untitled:test".to_string();
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());

    c.bench_function("folding range", |b| {
        b.iter(|| {
            let ranges = service.folding_range(FoldingRangeParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                work_done_token: None,
                partial_result_token: None,
            });
            black_box(ranges);
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = folding_range_bench
}
criterion_main!(benches);
