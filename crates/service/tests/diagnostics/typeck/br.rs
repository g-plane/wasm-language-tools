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
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
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
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn br_if() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
  (func (param i32)
    block $b
      local.get 0
      br_if $b
    end)

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
    unreachable)

  (func (param i32)
    (f64.const 0)
    (block (param f64) (result f32)
      (br_if 0
        (f32.const 0)
        (local.get 0)))
    (drop))

  (func (param i32)
    (f32.const 0)
    (block (result f64)
      (block (result f32)
        (br_if 1
          (f32.const 0)
          (local.get 0))))))
"#;
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
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
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
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
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
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
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
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
    block $l (result i32 (ref $vec))
      local.get $v
      i32.const 0
      br_on_non_null $l
    end
    drop
    drop)
  (func (param $v (ref $vec))
    block $l (result i32 (ref $vec))
      i32.const 0
      local.get $v
      br_on_non_null $l
    end
    drop
    drop))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
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
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn br_on_cast_fail() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (struct))
  (func (param (ref any)) (result (ref any))
    (block (result (ref $t))
      (br_on_cast_fail 1 (ref null any) (ref null $t)
        (local.get 0))))

  (type $f (func))
  (func $f (param (ref null $f)) (result funcref)
    (local.get 0))
  (func (param funcref) (result funcref funcref)
    (ref.null $f)
    (local.get 0)
    (br_on_cast_fail 0 funcref (ref $f)) ;; only leaves two funcref's on the stack
    (drop)
    (call $f)
    (local.get 0)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    calm(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
