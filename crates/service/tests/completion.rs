use insta::assert_json_snapshot;
use lsp_types::{
    CompletionParams, Position, TextDocumentIdentifier, TextDocumentPositionParams, Uri,
};
use wat_service::LanguageService;

fn create_params(uri: Uri, position: Position) -> CompletionParams {
    CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position,
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
        context: Default::default(),
    }
}

#[test]
fn param_types() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param ))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 17)));
    assert_json_snapshot!(response);
}

#[test]
fn param_types_following_incomplete_ident() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 18)));
    assert_json_snapshot!(response);
}

#[test]
fn param_types_following_ident() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $p))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 19)));
    assert_json_snapshot!(response);
}

#[test]
fn param_types_after_ident() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $p ))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 20)));
    assert_json_snapshot!(response);
}

#[test]
fn param_types_multiple_types() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param i32 ))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 21)));
    assert_json_snapshot!(response);
}

#[test]
fn param_types_incomplete_type() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param i))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 18)));
    assert_json_snapshot!(response);
}

#[test]
fn result_types() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (result ))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 18)));
    assert_json_snapshot!(response);
}

#[test]
fn result_types_multiple_types() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (result i32 ))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 22)));
    assert_json_snapshot!(response);
}

#[test]
fn result_types_incomplete_type() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (result i))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 19)));
    assert_json_snapshot!(response);
}

#[test]
fn local_types() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local ))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 17)));
    assert_json_snapshot!(response);
}

#[test]
fn local_types_following_incomplete_ident() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local $))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 18)));
    assert_json_snapshot!(response);
}

#[test]
fn local_types_following_ident() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local $p))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 19)));
    assert_json_snapshot!(response);
}

#[test]
fn local_types_after_ident() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local $p ))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 20)));
    assert_json_snapshot!(response);
}

#[test]
fn local_types_multiple_types() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local i32 ))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 21)));
    assert_json_snapshot!(response);
}

#[test]
fn local_types_incomplete_type() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local i))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 18)));
    assert_json_snapshot!(response);
}

#[test]
fn global_type_mut_type() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (global (mut ))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 17)));
    assert_json_snapshot!(response);
}

#[test]
fn instr_in_func() {
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
fn instr_in_func_with_paren() {
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
fn instr_in_func_before_plain_instr() {
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
fn instr_in_func_with_paren_before_plain_instr() {
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
fn instr_in_func_before_block_block() {
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
fn instr_in_func_with_paren_before_block_block() {
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
fn instr_in_func_before_block_if() {
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
fn instr_in_func_with_paren_before_block_if() {
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
fn instr_in_func_before_block_loop() {
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
fn instr_in_func_with_paren_before_block_loop() {
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
fn instr_following_instr_name() {
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

#[test]
fn locals_and_params() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $p i32) (param f32 f64) (local $l i32) (local f32 f64)
        (local.get )
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(3, 19)));
    assert_json_snapshot!(response);
}

#[test]
fn locals_and_params_following_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $p i32) (param f32 f64) (local $l i32) (local f32 f64)
        (local.get 1)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(3, 20)));
    assert_json_snapshot!(response);
}

#[test]
fn locals_and_params_following_dollar() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $p i32) (param f32 f64) (local $l i32) (local f32 f64)
        (local.get $)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(3, 20)));
    assert_json_snapshot!(response);
}

#[test]
fn locals_and_params_following_ident() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $p i32) (param f32 f64) (local $l i32) (local f32 f64)
        (local.get $x)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(3, 21)));
    assert_json_snapshot!(response);
}

#[test]
fn module_keyword() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(0, 1)));
    assert_json_snapshot!(response);
}

#[test]
fn module_keyword_incomplete() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(mo";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(0, 3)));
    assert_json_snapshot!(response);
}

#[test]
fn module_keyword_in_empty() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = " ";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(0, 1)));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_keyword() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module ())";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(0, 9)));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_keyword_incomplete() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module (f)";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(0, 10)));
    assert_json_snapshot!(response);
}

#[test]
fn module_field_keyword_without_paren() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "(module )";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(0, 8)));
    assert_json_snapshot!(response);
}
