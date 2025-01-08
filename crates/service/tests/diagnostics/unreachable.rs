use super::*;
use insta::assert_json_snapshot;
use lsp_types::Uri;
use wat_service::LanguageService;

fn disable_other_lints(service: &mut LanguageService, uri: Uri) {
    service.set_config(
        uri,
        ServiceConfig {
            lint: Lints {
                unused: LintLevel::Allow,
                shadow: LintLevel::Allow,
                implicit_module: LintLevel::Allow,
                unreachable: LintLevel::Warn,
            },
            ..Default::default()
        },
    );
}

#[test]
fn simple_reachable() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    nop
    nop)
  (func
    nop
    return)
  (func
    loop
      block
        nop
      end
    end)
  (func (param i32)
    loop
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
    nop))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(pick_diagnostics(response).is_empty());
}

#[test]
fn simple_unreachable() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
    assert_json_snapshot!(response);
}

#[test]
fn folded_plain_instr() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (nop)
    (i32.add
      (nop)
      (unreachable
        (i32.const 1))
      (i32.const 0))
    (nop)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    disable_other_lints(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
