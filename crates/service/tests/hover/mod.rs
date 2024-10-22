use insta::assert_json_snapshot;
use lsp_types::{HoverParams, Position, TextDocumentIdentifier, TextDocumentPositionParams, Uri};
use wat_service::LanguageService;

mod memo;

fn create_params(uri: Uri, position: Position) -> HoverParams {
    HoverParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position,
        },
        work_done_progress_params: Default::default(),
    }
}

#[test]
fn param_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param i32)
        (local.get 0)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(3, 20)));
    assert_json_snapshot!(response);
}

#[test]
fn param_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $param i32)
        (local.get $param)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(3, 25)));
    assert_json_snapshot!(response);
}

#[test]
fn local_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local i32)
        (local.get 0)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(3, 20)));
    assert_json_snapshot!(response);
}

#[test]
fn local_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local $local i32)
        (local.get $local)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(3, 25)));
    assert_json_snapshot!(response);
}

#[test]
fn global_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (global i32)
    (func (global.get 0))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(3, 23)));
    assert_json_snapshot!(response);
}

#[test]
fn global_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (global $global (mut i32))
    (func (global.get $global))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(3, 29)));
    assert_json_snapshot!(response);
}

#[test]
fn func_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $param i32) (param f32 f64) (result i32 i64)
        (call 0)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(3, 15)));
    assert_json_snapshot!(response);
}

#[test]
fn func_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $func (param $param i32) (param f32 f64) (result i32 i64)
        (call $func)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(3, 19)));
    assert_json_snapshot!(response);
}

#[test]
fn func_type_use_only() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (type $type (func (param $param i32) (param f32 f64) (result i32 i64)))
    (func $func (type $type)
        (call $func)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(4, 19)));
    assert_json_snapshot!(response);
}

#[test]
fn func_type_use_with_inlined() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (type $type (func (param $param i32) (param f32 f64) (result i32 i64)))
    (func $func (type $type) (param $p f64) (result i32)
        (call $func)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(4, 19)));
    assert_json_snapshot!(response);
}

#[test]
fn type_use_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (type (func (param $param i32) (param f32 f64) (result i32 i64)))
    (func (type 0))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(3, 17)));
    assert_json_snapshot!(response);
}

#[test]
fn type_use_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (type $type (func (param $param i32) (param f32 f64) (result i32 i64)))
    (func (type $type))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(3, 21)));
    assert_json_snapshot!(response);
}

#[test]
fn param_decl() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $param i64))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(2, 21)));
    assert_json_snapshot!(response);
}

#[test]
fn param_keyword() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $param i64))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(2, 15)));
    assert_json_snapshot!(response);
}

#[test]
fn local_decl() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local $local i64))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(2, 21)));
    assert_json_snapshot!(response);
}

#[test]
fn local_keyword() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local $local i64))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(2, 15)));
    assert_json_snapshot!(response);
}

#[test]
fn func_decl() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $func (param $param i32) (param f32 f64) (result i32 i64))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(2, 14)));
    assert_json_snapshot!(response);
}

#[test]
fn func_keyword() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $func (param $param i32) (param f32 f64) (result i32 i64))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(2, 8)));
    assert_json_snapshot!(response);
}

#[test]
fn type_use_only_func() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $func (type $t))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(2, 14)));
    assert_json_snapshot!(response);
}

#[test]
fn type_decl() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (type $type (func (param $param i32) (param f32 f64) (result i32 i64)))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(2, 14)));
    assert_json_snapshot!(response);
}

#[test]
fn type_keyword() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (type $type (func (param $param i32) (param f32 f64) (result i32 i64)))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(2, 8)));
    assert_json_snapshot!(response);
}

#[test]
fn type_decl_empty() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (type $type (func))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(2, 14)));
    assert_json_snapshot!(response);
}

#[test]
fn global_decl() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (global $global i64)
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(2, 17)));
    assert_json_snapshot!(response);
}

#[test]
fn global_decl_mut() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (global $global (mut i64))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(2, 17)));
    assert_json_snapshot!(response);
}

#[test]
fn global_keyword() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (global $global i64)
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(2, 9)));
    assert_json_snapshot!(response);
}

#[test]
fn num_type() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param i32))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(2, 19)));
    assert_json_snapshot!(response);
}

#[test]
fn vec_type() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param v128))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(2, 19)));
    assert_json_snapshot!(response);
}

#[test]
fn ref_type() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param funcref))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.hover(create_params(uri, Position::new(2, 19)));
    assert_json_snapshot!(response);
}
