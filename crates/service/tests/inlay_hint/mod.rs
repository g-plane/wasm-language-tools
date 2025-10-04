use insta::assert_json_snapshot;
use lspt::{InlayHintParams, Position, Range, TextDocumentIdentifier};
use wat_service::LanguageService;

fn create_params(uri: String, line: u32, character: u32) -> InlayHintParams {
    InlayHintParams {
        work_done_token: Default::default(),
        text_document: TextDocumentIdentifier { uri },
        range: Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position { line, character },
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
    let response = service.inlay_hint(create_params(uri, 6, 0));
    assert_json_snapshot!(response);
}

#[test]
fn param_via_type_def() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (func (param $a i32) (param $b f32)))
  (func (type 0)
    (local.get $a)
    (local.get 1)
    i32.add))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.inlay_hint(create_params(uri, 7, 0));
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
    let response = service.inlay_hint(create_params(uri, 6, 0));
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
    let response = service.inlay_hint(create_params(uri, 5, 0));
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
    let response = service.inlay_hint(create_params(uri, 6, 0));
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
    let response = service.inlay_hint(create_params(uri, 4, 0));
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
    let response = service.inlay_hint(create_params(uri, 4, 0));
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
    let response = service.inlay_hint(create_params(uri, 4, 0));
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
    let response = service.inlay_hint(create_params(uri, 6, 0));
    assert_json_snapshot!(response);
}

#[test]
fn field() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field (mut i32))))
  (func
    struct.get 0 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.inlay_hint(create_params(uri, 5, 0));
    assert_json_snapshot!(response);
}

#[test]
fn field_with_struct_changed() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field i32)))
  (type (struct (field (mut i32))))
  (func
    struct.get 0 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service
        .inlay_hint(create_params(uri.clone(), 6, 0))
        .unwrap();
    let first = &response.first().unwrap().label;

    let source = "
(module
  (type (struct (field i32)))
  (type (struct (field (mut i32))))
  (func
    struct.get 1 0))
";
    service.commit(uri.clone(), source.into());
    let response = service
        .inlay_hint(create_params(uri.clone(), 6, 0))
        .unwrap();
    let second = &response.first().unwrap().label;
    assert_ne!(first, second);
}
