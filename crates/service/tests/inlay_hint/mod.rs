use insta::assert_json_snapshot;
use lsp_types::{InlayHintParams, Position, Range, TextDocumentIdentifier, Uri};
use wat_service::LanguageService;

fn create_params(uri: Uri, doc_end: Position) -> InlayHintParams {
    InlayHintParams {
        work_done_progress_params: Default::default(),
        text_document: TextDocumentIdentifier { uri },
        range: Range::new(Position::new(0, 0), doc_end),
    }
}

#[test]
fn param() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $param f32)
        (local.get $param)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.inlay_hint(create_params(uri, Position::new(6, 0)));
    assert_json_snapshot!(response);
}

#[test]
fn local() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local $local i64)
        (local.get $local)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.inlay_hint(create_params(uri, Position::new(6, 0)));
    assert_json_snapshot!(response);
}

#[test]
fn global() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (global $global funcref)
    (func (global.get $global))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.inlay_hint(create_params(uri, Position::new(5, 0)));
    assert_json_snapshot!(response);
}

#[test]
fn func_end() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $name
        (i32.const 0)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.inlay_hint(create_params(uri, Position::new(6, 0)));
    assert_json_snapshot!(response);
}

#[test]
fn block_end() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (block $b)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.inlay_hint(create_params(uri, Position::new(4, 0)));
    assert_json_snapshot!(response);
}

#[test]
fn loop_end() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (loop $b)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.inlay_hint(create_params(uri, Position::new(4, 0)));
    assert_json_snapshot!(response);
}

#[test]
fn if_end() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (if $b)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.inlay_hint(create_params(uri, Position::new(4, 0)));
    assert_json_snapshot!(response);
}

#[test]
fn ref_type() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local $local (ref 0))
        (local.get $local)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.inlay_hint(create_params(uri, Position::new(6, 0)));
    assert_json_snapshot!(response);
}
