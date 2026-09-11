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
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
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
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn missing() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    block end $b
    loop end $l
    i32.const 0
    if end $i
    try_table end $t))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}

#[test]
fn matched() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    block $b end $b
    loop $l end $l
    i32.const 0
    if $i end $i
    try_table $t end $t))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn mismatch() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    block $b1 end $b2
    loop $l1 end $l2
    i32.const 0
    if $i1 end $i2
    try_table $t1 end $t2))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    calm(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
