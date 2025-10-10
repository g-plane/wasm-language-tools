use super::*;
use insta::assert_json_snapshot;
use wat_service::LanguageService;

#[test]
fn in_func() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.const 0) )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 24));
    assert_json_snapshot!(response);
}

#[test]
fn in_func_with_paren() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.const 0) ()
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 25));
    assert_json_snapshot!(response);
}

#[test]
fn in_func_before_plain_instr() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.const 0) (i32.const 0))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 24));
    assert_json_snapshot!(response);
}

#[test]
fn in_func_with_paren_before_plain_instr() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.const 0) ((i32.const 0))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 25));
    assert_json_snapshot!(response);
}

#[test]
fn in_func_before_block_block() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.const 0) (block))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 24));
    assert_json_snapshot!(response);
}

#[test]
fn in_func_with_paren_before_block_block() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.const 0) ((block))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 25));
    assert_json_snapshot!(response);
}

#[test]
fn in_func_before_block_if() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.const 0) (if))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 24));
    assert_json_snapshot!(response);
}

#[test]
fn in_func_with_paren_before_block_if() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.const 0) ((if))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 25));
    assert_json_snapshot!(response);
}

#[test]
fn in_func_before_block_loop() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.const 0) (loop))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 24));
    assert_json_snapshot!(response);
}

#[test]
fn in_func_with_paren_before_block_loop() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.const 0) ((loop))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 25));
    assert_json_snapshot!(response);
}

#[test]
fn following_instr_name() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.const 0) (i32))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 28));
    assert_json_snapshot!(response);
}

#[test]
fn in_func_without_any_instrs() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 10));
    assert_json_snapshot!(response);
}

#[test]
fn in_func_with_paren_without_any_instrs() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func ()
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 11));
    assert_json_snapshot!(response);
}

#[test]
fn in_global() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global i32 )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 16));
    assert_json_snapshot!(response);
}

#[test]
fn in_global_with_paren() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global i32 ()
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 17));
    assert_json_snapshot!(response);
}

#[test]
fn nested() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $func
    (call $func ())))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 3, 17));
    assert_json_snapshot!(response);
}

#[test]
fn in_block() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (block (i32.const 0) ()))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 32));
    assert_json_snapshot!(response);
}

#[test]
fn after_block_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (block (result i32) ()))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 31));
    assert_json_snapshot!(response);
}

#[test]
fn following_dot() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 15));
    assert_json_snapshot!(response);
}

#[test]
fn after_dot() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.c))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 16));
    assert_json_snapshot!(response);
}

#[test]
fn shape_descriptor() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func v128.const )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 21));
    assert_json_snapshot!(response);
}

#[test]
fn shape_descriptor_incomplete() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func v128.const i)
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, 2, 22));
    assert_json_snapshot!(response);
}
