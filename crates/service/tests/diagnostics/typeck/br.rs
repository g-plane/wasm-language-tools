use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn br_incorrect() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block (result f32)
      (br 0))
    (unreachable))
  (func
    (block (result f32)
      (f64.const 0)
      (br 0))
    (unreachable))
  (func
    (block (result f32 f32)
      (block (result f64 f64)
        (br 1
          (f64.const 0)
          (f64.const 0)))
      (unreachable))
    (unreachable))
  (func
    block (result f32)
      br 0
    end
    unreachable)
  (func
    block (result f32)
      f64.const 0
      br 0
    end
    unreachable)
  (func
    block (result f32 f32)
      block (result f64 f64)
        f64.const 0
        f64.const 0
        br 1
      end
      unreachable
    end
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn br_correct() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block (result i32)
      (i32.const 0)
      (br 0))
    (unreachable))
  (func
    (block (result i32)
      (i32.const 0)
      (i32.const 0)
      (br 0))
    (unreachable))
  (func
    (block (result f32 f32)
      (block (result f64 f64)
        (br 1
          (f32.const 0)
          (f32.const 0)))
      (unreachable))
    (unreachable))
  (func
    block (result i32)
      i32.const 0
      br 0
    end
    unreachable)
  (func
    block (result i32)
      i32.const 0
      i32.const 0
      br 0
    end
    unreachable)
  (func
    block (result f32 f32)
      block (result f64 f64)
        f32.const 0
        f32.const 0
        br 1
      end
      unreachable
    end
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn br_if_incorrect() {
    let uri = "untitled:test".to_string();
    // copied from https://github.com/WebAssembly/spec/blob/f3a0e06235d2d84bb0f3b5014da4370613886965/test/core/br_if.wast
    let source = "
(module
  (func $type-false-f64
    (block
      (f64.neg
        (br_if 0
          (i32.const 0)))))

  (func $type-true-f64
    (block
      (f64.neg
        (br_if 0
          (i64.const 1)))))

  (func $type-false-arg-vs-num (result i32)
    (block (result i32)
      (br_if 0
        (i32.const 0))
      (i32.const 1)))
  (func $type-true-arg-vs-num (result i32)
    (block (result i32)
      (br_if 0
        (i32.const 1))
      (i32.const 1)))

  (func $type-false-arg-num-vs-void
    (block
      (br_if 0
        (i32.const 0)
        (i32.const 0))))
  (func $type-true-arg-num-vs-void
    (block
      (br_if 0
        (i32.const 0)
        (i32.const 1))))

  (func $type-false-arg-void-vs-num (result i32)
    (block (result i32)
      (br_if 0
        (nop)
        (i32.const 0))
      (i32.const 1)))
  (func $type-true-arg-void-vs-num (result i32)
    (block (result i32)
      (br_if 0
        (nop)
        (i32.const 1))
      (i32.const 1)))

  (func $type-false-arg-num-vs-num (result i32)
    (block (result i32)
      (drop
        (br_if 0
          (i64.const 1)
          (i32.const 0)))
      (i32.const 1)))
  (func $type-true-arg-num-vs-num (result i32)
    (block (result i32)
      (drop
        (br_if 0
          (i64.const 1)
          (i32.const 0)))
      (i32.const 1)))

  (func $type-cond-empty-vs-i32
    (block
      (br_if 0)))
  (func $type-cond-void-vs-i32
    (block
      (br_if 0
        (nop))))
  (func $type-cond-num-vs-i32
    (block
      (br_if 0
        (i64.const 0))))
  (func $type-arg-cond-void-vs-i32 (result i32)
    (block (result i32)
      (br_if 0
        (i32.const 0)
        (nop))
      (i32.const 1)))
  (func $type-arg-void-vs-num-nested (result i32)
    (block (result i32)
      (i32.const 0)
      (block
        (br_if 1
          (i32.const 1)))))
  (func $type-arg-cond-num-vs-i32 (result i32)
    (block (result i32)
      (br_if 0
        (i32.const 0)
        (i64.const 0))
      (i32.const 1)))

  (func $type-1st-cond-empty-in-then
    (block
      (i32.const 0)
      (i32.const 0)
      (if (result i32)
        (then
          (br_if 0))))
    (i32.eqz)
    (drop))
  (func $type-2nd-cond-empty-in-then
    (block
      (i32.const 0)
      (i32.const 0)
      (if (result i32)
        (then
          (br_if 0
            (i32.const 1)))))
    (i32.eqz)
    (drop))
  (func $type-1st-cond-empty-in-return
    (block (result i32)
      (return
        (br_if 0)))
    (i32.eqz)
    (drop))
  (func $type-2nd-cond-empty-in-return
    (block (result i32)
      (return
        (br_if 0
          (i32.const 1))))
    (i32.eqz)
    (drop))
  (func
    block (result f32 f32)
      block (result f64 f64)
        f64.const 0
        f64.const 0
        i32.const 0
        br_if 1
      end
      unreachable
    end
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn br_if_correct() {
    let uri = "untitled:test".to_string();
    // copied from https://github.com/WebAssembly/spec/blob/f3a0e06235d2d84bb0f3b5014da4370613886965/test/core/br_if.wast
    let source = r#"
(module
  (func $dummy)
  (func (export "type-f32")
    (block
      (drop
        (f32.neg
          (br_if 0
            (f32.const 0)
            (i32.const 1))))))
  (func (export "type-f32-value") (result f32)
    (block (result f32)
      (f32.neg
        (br_if 0
          (f32.const 3)
          (i32.const 1)))))

  (func (export "as-block-first") (param i32) (result i32)
    (block
      (br_if 0
        (local.get 0))
      (return
        (i32.const 2)))
    (i32.const 3))
  (func (export "as-block-first-value") (param i32) (result i32)
    (block (result i32)
      (drop
        (br_if 0
          (i32.const 10)
          (local.get 0)))
      (return
        (i32.const 11))))
  (func (export "as-loop-last") (param i32)
    (block
      (loop
        (call $dummy)
        (br_if 1
          (local.get 0)))))

  (func (export "as-br-value") (result i32)
    (block (result i32)
      (br 0
        (br_if 0
          (i32.const 1)
          (i32.const 2)))))
  (func (export "as-br_if-cond")
    (block
      (br_if 0
        (br_if 0
          (i32.const 1)
          (i32.const 1)))))
  (func (export "as-br_if-value") (result i32)
    (block (result i32)
      (drop
        (br_if 0
          (br_if 0
            (i32.const 1)
            (i32.const 2))
          (i32.const 3)))
      (i32.const 4)))
  (func (export "as-br_if-value-cond") (param i32) (result i32)
    (block (result i32)
      (drop
        (br_if 0
          (i32.const 2)
          (br_if 0
            (i32.const 1)
            (local.get 0))))
      (i32.const 4))))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn br_table_incorrect() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    block
      br_table 0
    end)
  (func
    block (result f32)
      br_table 0
    end
    unreachable)
  (func
    block (result f32)
      f64.const 0
      br_table 0
    end
    unreachable)
  (func
    block (result f32)
      i32.const 0
      br_table 0
    end
    unreachable)
  (func
    block (result f32)
      f32.const 0
      br_table 0
    end
    unreachable)
  (func
    block (result f32)
      f64.const 0
      i32.const 0
      br_table 0
    end
    unreachable)
  (func
    block (result f32 f32)
      block (result f64 f64)
        f64.const 0
        f64.const 0
        i32.const 0
        br_table 1
      end
      unreachable
    end
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn br_table_correct() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    block (result f32)
      f32.const 0
      i32.const 0
      br_table 0
    end
    unreachable)
  (func
    block (result f32)
      i32.const 0
      f32.const 0
      i32.const 0
      br_table 0
    end
    unreachable)
  (func
    block (result f32 f32)
      block (result f64 f64)
        f32.const 0
        f32.const 0
        i32.const 0
        br_table 1
      end
      unreachable
    end
    unreachable))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn br_on_null() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $vec (struct))
  (func (param $v (ref $vec))
    block $l (result i32)
      local.get $v
      i32.const 0
      br_on_null $l
    end)
  (func (param $v (ref null $vec))
    block $l (result i32)
      i32.const 0
      local.get $v
      br_on_null $l
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn br_on_non_null() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $vec (struct))
  (func (param $v (ref $vec))
    block $l (result i32)
      local.get $v
      i32.const 0
      br_on_non_null $l
    end
    drop)
  (func (param $v (ref $vec))
    block $l (result i32)
      i32.const 0
      local.get $v
      br_on_non_null $l
    end
    drop))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn br_on_cast() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct))
  (func (param (ref null any)) (result (ref $s))
    (block (result (ref any))
      (br_on_cast 1 (ref null any) (ref $s)
        (local.get 0)))
    (unreachable))

  (type $t (func))
  (func $f (param (ref null $t)) (result funcref)
    (local.get 0))
  (func (param funcref) (result funcref funcref)
    (ref.null $t)
    (local.get 0)
    (br_on_cast 0 funcref (ref $t)) ;; only leaves two funcref's on the stack
    (drop)
    (call $f)
    (local.get 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
