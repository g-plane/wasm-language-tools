use super::*;
use insta::assert_json_snapshot;
use lsp_types::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn in_func() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 10)));
    assert_json_snapshot!(response);
}

#[test]
fn in_func_with_paren() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func ()
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 11)));
    assert_json_snapshot!(response);
}

#[test]
fn in_func_before_plain_instr() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (i32.const 0))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 10)));
    assert_json_snapshot!(response);
}

#[test]
fn in_func_with_paren_before_plain_instr() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func ((i32.const 0))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 11)));
    assert_json_snapshot!(response);
}

#[test]
fn in_func_before_block_block() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (block))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 10)));
    assert_json_snapshot!(response);
}

#[test]
fn in_func_with_paren_before_block_block() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func ((block))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 11)));
    assert_json_snapshot!(response);
}

#[test]
fn in_func_before_block_if() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (if))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 10)));
    assert_json_snapshot!(response);
}

#[test]
fn in_func_with_paren_before_block_if() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func ((if))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 11)));
    assert_json_snapshot!(response);
}

#[test]
fn in_func_before_block_loop() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (loop))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 10)));
    assert_json_snapshot!(response);
}

#[test]
fn in_func_with_paren_before_block_loop() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func ((loop))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 11)));
    assert_json_snapshot!(response);
}

#[test]
fn following_instr_name() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (i32))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 14)));
    assert_json_snapshot!(response);
}
