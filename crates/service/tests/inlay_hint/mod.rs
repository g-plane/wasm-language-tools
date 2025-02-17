use insta::assert_json_snapshot;
use lspt::{InlayHintParams, Position, Range, TextDocumentIdentifier, Uri};
use wat_service::LanguageService;

fn create_params(uri: String, doc_end: Position) -> InlayHintParams {
    InlayHintParams {
        work_done_token: Default::default(),
        text_document: TextDocumentIdentifier { uri },
        range: Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: doc_end,
        },
    }
}

#[test]
fn param() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param $param f32)
        (local.get $param)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.inlay_hint(create_params(
        uri,
        Position {
            line: 6,
            character: 0,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn local() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (local $local i64)
        (local.get $local)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.inlay_hint(create_params(
        uri,
        Position {
            line: 6,
            character: 0,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn global() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global $global funcref)
    (func (global.get $global))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.inlay_hint(create_params(
        uri,
        Position {
            line: 5,
            character: 0,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn func_end() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $name
        (i32.const 0)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.inlay_hint(create_params(
        uri,
        Position {
            line: 6,
            character: 0,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn block_end() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block $b)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.inlay_hint(create_params(
        uri,
        Position {
            line: 4,
            character: 0,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn loop_end() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (loop $b)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.inlay_hint(create_params(
        uri,
        Position {
            line: 4,
            character: 0,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn if_end() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (if $b)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.inlay_hint(create_params(
        uri,
        Position {
            line: 4,
            character: 0,
        },
    ));
    assert_json_snapshot!(response);
}

#[test]
fn ref_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (local $local (ref 0))
        (local.get $local)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.inlay_hint(create_params(
        uri,
        Position {
            line: 6,
            character: 0,
        },
    ));
    assert_json_snapshot!(response);
}
