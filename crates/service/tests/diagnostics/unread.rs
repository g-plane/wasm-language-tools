use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

fn disable_other_lints(service: &mut LanguageService, uri: &str) {
    service.set_config(
        uri,
        Some(ServiceConfig {
            lint: Lints {
                unused: LintLevel::Allow,
                unread: LintLevel::Warn,
                unreachable: LintLevel::Allow,
                needless_try_table: LintLevel::Allow,
                ..Default::default()
            },
            ..Default::default()
        }),
    );
}

#[test]
fn param() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param i32)
    i32.const 42
    local.set 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn undef() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    i32.const 42
    local.set 0))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn valid() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (local i32)
    i32.const 0
    local.set 0
    loop
      local.get 0
      drop
    end)
  (func (local i32)
    (return
      (local.get 0
        (local.set 0
          (i32.const 1)))))

  (func (local $x i32)
    i32.const 1
    local.set $x
    (block)
    local.get 0
    drop)
  (func (local $x i32)
    i32.const 1
    local.set 0
    (loop)
    local.get $x
    drop))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn invalid() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (local $x i32)
    i32.const 0
    local.set 0)
  (func (local $x i32)
    i32.const 0
    local.set $x)
  (func (local i32)
    (local.get 0
      (local.set 0
        (i32.const 0
          (local.set 0
            (i32.const 1)))))
    (drop))
  (func (local i32)
    i32.const 0
    local.tee 0
    local.set 0
    i32.const 1
    local.set 0
    (drop
      (local.get 0))
    i32.const 2
    local.set 0)

  (func (local i32)
    i32.const 0
    local.tee 0
    local.set 0
    local.get 0
    drop)
  (func (local i32)
    i32.const 0
    local.set 0
    i32.const 0
    local.tee 0
    local.get 0
    unreachable)
  (func (local $x i32)
    i32.const 1
    local.set $x
    (block
      i32.const 2
      local.set 0)
    local.get 0
    drop)
  (func (local $x i32)
    i32.const 1
    local.set 0
    (loop)
    i32.const 2
    local.set $x
    local.get $x
    drop))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn infinite_loop() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (local i32)
    i32.const 0
    local.set 0
    loop
      br 0
      local.get 0
      drop
    end)
  (func (local i32)
    loop
      i32.const 0
      local.set 0
      br 0
    end
    local.get 0
    drop)
  (func (local i32)
    (local.set 0
      (i32.const 0))
    (loop
      (br 0))
    (drop
      (local.get 0)))
  (func (local i32)
    (block
      (loop
        (if
          (i32.const 1)
          (then
            (local.set 0
              (i32.const 1))
            (br 1))
          (else
            (br 1)))))
    (drop
      (local.get 0))))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn conditional() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  ;; invalid
  (func (local i32)
    (local.set 0
      (i32.const 0))
    (if
      (i32.const 0)
      (then
        (local.set 0
          (i32.const 1)))))
  (func (local i32)
    (local.set 0
      (i32.const 0))
    (if
      (i32.const 0)
      (then
        (local.set 0
          (i32.const 1)))
      (else
        (local.set 0
          (i32.const 2))))
    (drop
      (local.get 0)))
  (func (local i32)
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
  (func (local i32)
    i32.const 0
    local.set 0
    i32.const 0
    if
      i32.const 1
      local.set 0
      local.get 0
      drop
    else
      i32.const 2
      local.set 0
    end)
  (func (local i32)
    i32.const 0
    local.tee 0
    if
      i32.const 1
      local.set 0
    else
      local.get 0
      drop
    end)
  (func (param i32) (local i32)
    local.get 0
    local.set 1
    loop
      local.get 0
      if
        local.get 0
        local.set 1
      else
        local.get 1
        drop
      end
    end)

  ;; valid
  (func (local i32)
    i32.const 0
    local.tee 0
    if
      local.get 0
      drop
    end)
  (func (local i32)
    (local.set 0
      (i32.const 0))
    (if
      (i32.const 1)
      (then
        (local.set 0
          (i32.const 1))))
    (drop
      (local.get 0)))
  (func (param i32) (local i32)
    (if
      (local.get 0)
      (then
        (local.set 1
          (local.get 0))))
    (drop
      (local.get 1)))
  (func (local i32)
    (block
      (loop
        (if
          (i32.const 1)
          (then
            (local.set 0
              (i32.const 1))
            (br 1))
          (else
            (br 2)))))
    (drop
      (local.get 0)))
  (func (param i32) (local i32)
    local.get 0
    local.set 1
    loop
      local.get 0
      if
        local.get 0
        local.set 1
      else
        local.get 1
        drop
      end
      local.get 0
      if
        br 1
      end
    end))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    disable_other_lints(&mut service, &uri);
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
