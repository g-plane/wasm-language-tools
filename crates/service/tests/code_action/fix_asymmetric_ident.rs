use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn folded() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
  (block $b)
  (loop $l)
  (if $i (i32.const 0) (then))
  (try_table $t)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    assert!(service.code_action(create_params(uri.clone(), 3, 12, 3, 12)).is_none());
    assert!(service.code_action(create_params(uri.clone(), 4, 11, 4, 11)).is_none());
    assert!(service.code_action(create_params(uri.clone(), 5, 30, 5, 30)).is_none());
    assert!(service.code_action(create_params(uri.clone(), 6, 16, 6, 16)).is_none());
}

#[test]
fn no_end_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    block end
    block $b end
    loop end
    loop $l end
    i32.const 0
    if end
    i32.const 0
    if $i end
    try_table end
    try_table $t end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    assert!(service.code_action(create_params(uri.clone(), 3, 12, 3, 12)).is_none());
    assert!(service.code_action(create_params(uri.clone(), 4, 14, 4, 14)).is_none());
    assert!(service.code_action(create_params(uri.clone(), 5, 11, 5, 11)).is_none());
    assert!(service.code_action(create_params(uri.clone(), 6, 14, 6, 14)).is_none());
    assert!(service.code_action(create_params(uri.clone(), 8, 9, 8, 9)).is_none());
    assert!(
        service
            .code_action(create_params(uri.clone(), 10, 11, 10, 11))
            .is_none(),
    );
    assert!(service.code_action(create_params(uri.clone(), 11, 9, 11, 9)).is_none());
    assert!(
        service
            .code_action(create_params(uri.clone(), 12, 19, 12, 19))
            .is_none(),
    );
}

#[test]
fn matched() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    block $b end $b
    loop $l end $l
    if $i end $i
    try_table $t end $t))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    assert!(service.code_action(create_params(uri.clone(), 3, 15, 3, 15)).is_none());
    assert!(service.code_action(create_params(uri.clone(), 4, 15, 4, 15)).is_none());
    assert!(service.code_action(create_params(uri.clone(), 5, 16, 5, 16)).is_none());
    assert!(service.code_action(create_params(uri.clone(), 6, 19, 6, 19)).is_none());
}

#[test]
fn mismatch_in_block() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    block $b1 end $b2))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(uri.clone(), 3, 20, 3, 20));
    assert_json_snapshot!(response);
}

#[test]
fn mismatch_in_loop() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    loop $l1 end $l2))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(uri.clone(), 3, 18, 3, 18));
    assert_json_snapshot!(response);
}

#[test]
fn mismatch_in_if() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    if $i1 end $i2))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(uri.clone(), 3, 14, 3, 14));
    assert_json_snapshot!(response);
}

#[test]
fn mismatch_in_try_table() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    try_table $t1 end $t2))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(uri.clone(), 3, 23, 3, 23));
    assert_json_snapshot!(response);
}

#[test]
fn diagnostic_mismatch() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    loop $l1 end $l2))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let mut params = create_params(uri.clone(), 3, 14, 3, 14);
    params.context.diagnostics = service.publish_diagnostics(uri.clone()).diagnostics;
    let response = service.code_action(params);
    assert_json_snapshot!(response);
}

#[test]
fn excess_in_block() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    block end $b))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(uri.clone(), 3, 12, 3, 12));
    assert_json_snapshot!(response);
}

#[test]
fn excess_in_loop() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    loop end $l))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(uri.clone(), 3, 14, 3, 14));
    assert_json_snapshot!(response);
}

#[test]
fn excess_in_if() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    if end $i))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(uri.clone(), 3, 9, 3, 9));
    assert_json_snapshot!(response);
}

#[test]
fn excess_in_try_table() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    try_table end $t))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(uri.clone(), 3, 20, 3, 20));
    assert_json_snapshot!(response);
}

#[test]
fn diagnostic_excess() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    try_table end $t))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let mut params = create_params(uri.clone(), 3, 16, 3, 16);
    params.context.diagnostics = service.publish_diagnostics(uri.clone()).diagnostics;
    let response = service.code_action(params);
    assert_json_snapshot!(response);
}

#[test]
fn comments() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    block end (;;) $b))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.code_action(create_params(uri.clone(), 3, 12, 3, 12));
    assert_json_snapshot!(response);
}
