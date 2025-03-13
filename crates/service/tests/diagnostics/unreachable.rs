use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

fn disable_other_lints(service: &mut LanguageService, uri: String) {
    service.set_config(
        uri,
        ServiceConfig {
            lint: Lints {
                unused: LintLevel::Allow,
                unreachable: LintLevel::Warn,
                ..Default::default()
            },
            ..Default::default()
        },
    );
}

#[test]
fn simple_reachable() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    nop
    nop)
  (func
    nop
    return)
  (func
    block
      block
        nop
      end
    end)
  (func (param i32)
    block
      local.get 0
      if
        br 0
      end
    end
    nop)
  (func (param i32)
    local.get 0
    if
      return
    end
    nop)
  (func (param i32)
    local.get 0
    if
    else
      return
    end
    nop)

  (type $s (struct))
  (func (param (ref $s))
    block $l
      local.get 0
      br_on_null $l
      drop
    end
    nop)
  (func (param (ref null $s))
    block $l
      local.get 0
      br_on_non_null $l
    end
    nop))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn simple_unreachable() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    return
    nop)
  (func
    nop
    return
    nop)
  (func
    block
      return
    end)
  (func
    unreachable
    nop)
  (func
    loop
      br 0
      nop
    end)
  (func
    block
      i32.const 0
      br_table 0
    end)

  (func
    i32.const 1
    if
      return
    else
      unreachable
    end
    nop)
  (func
    i32.const 1
    if
      block
        return
      end
    else
      block
        unreachable
      end
    end
    nop)
  (func
    nop
    block
      return
    end
    nop)
  (func
    loop
      block
        i32.const 0
        if
          br 2
        else
          br 1
        end
        nop
      end
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn nested_if() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param i32)
    local.get 0
    if
      nop
      unreachable
      nop
    else
      local.get 0
      if
        unreachable
      end
      nop
    end
    nop)

  (func (param i32)
    local.get 0
    if
      nop
      unreachable
      nop
    else
      local.get 0
      if
        unreachable
      else
        unreachable
      end
      nop
    end
    nop))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn merge_range() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    return
    nop
    unreachable
    nop)
  (func
    return
    i32.const 1
    if
      nop
    else
      nop
    end)

  (func (result i32)
    (i32.add
      (i32.add
        (i32.add
          (i32.add
            (i32.add
              (i32.const 0)
              (unreachable))
            (unreachable))
          (unreachable))
        (unreachable))
      (drop
        (i32.add
          (i32.const 0)
          (i32.const 0))))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn br_if_a() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $log (param i32))
  (func (param i32)
    block $a
      block $b
        block $c
          local.get 0
          br_if $a
          return
        end
        i32.const 2
        call $log
      end
      i32.const 3
      call $log
    end
    i32.const 4
    call $log))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn br_if_b() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $log (param i32))
  (func (param i32)
    block $a
      block $b
        block $c
          local.get 0
          br_if $b
          return
        end
        i32.const 2
        call $log
      end
      i32.const 3
      call $log
    end
    i32.const 4
    call $log))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn br_if_c() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $log (param i32))
  (func (param i32)
    block $a
      block $b
        block $c
          local.get 0
          br_if $c
          return
        end
        i32.const 2
        call $log
      end
      i32.const 3
      call $log
    end
    i32.const 4
    call $log))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn folded_plain_instr() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (nop)
    (i32.add
      (nop)
      (unreachable
        (i32.const 1))
      (i32.const 0))
    (drop)
    (nop)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn folded_block_if() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param i32)
    (if
      (local.get 0)
      (unreachable
        (nop))
      (nop)
      (then)
      (else))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn infinite_loop() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    loop
      br 0
    end
    nop)
  (func
    loop
      block
        br 1
      end
    end
    nop)
  (func (param i32)
    loop
      nop
      block
        local.get 0
        if
          br 0
        else
          br 2
        end
      end
    end
    nop)
  (func (param i32)
    loop
      nop
      block
        local.get 0
        if
          br 2
        else
          br 2
        end
      end
    end
    nop)
  (func
    (loop
      (br 0))
    (nop))
  (func (local $x i32)
    (block $incr_loop_break
      (loop $incr_loop
        (br $incr_loop)
        (br_if $incr_loop_break
          (local.get $x))))
    (nop)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn finite_loop() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    loop
    end
    nop)
  (func
    loop
      block
        br 0
      end
    end
    nop)
  (func (param i32)
    loop
      nop
      block
        local.get 0
        if
          br 0
        else
          br 1
        end
      end
    end
    nop)
  (func (param i32)
    loop $loop
      local.get 0
      if
        br $loop
      end
    end
    nop)
  (func (param i32)
    loop $loop
      nop
      block
        nop
        block
          nop
          block
            local.get 0
            if
              br $loop
            end
          end
        end
      end
    end
    nop)
  (func (param i32)
    loop $loop
      nop
      block
        nop
        block
          nop
          block
            local.get 0
            br_if $loop
          end
        end
      end
    end
    nop)
  (func
    (loop)
    (nop))
  (func (local $x i32)
    (block $incr_loop_break
      (loop $incr_loop
        (if
          (local.get $x)
          (then
            (br $incr_loop))
          (else
            (br $incr_loop_break)))))
    (nop))
  (func (local $x i32)
    (block $incr_loop_break
      (loop $incr_loop
        (br_if $incr_loop_break
          (local.get $x))
        (br $incr_loop)))
    (nop)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn global() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global i32
    unreachable
    i32.const 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn folded_instr_with_loop() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (nop
      (nop)
      (loop
        (br_if 0
          (i32.const 0))))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}
