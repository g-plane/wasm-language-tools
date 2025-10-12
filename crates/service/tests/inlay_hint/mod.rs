use insta::assert_json_snapshot;
use lspt::{
    ClientCapabilities, InitializeParams, InlayHintParams, Position, Range, TextDocumentIdentifier,
    WorkspaceClientCapabilities,
};
use wat_service::{InlayHintOptions, LanguageService, ServiceConfig};

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
fn empty_config() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func))
";
    let mut service = LanguageService::default();
    service.initialize(InitializeParams {
        capabilities: ClientCapabilities {
            workspace: Some(WorkspaceClientCapabilities {
                configuration: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    });
    service.commit(uri.clone(), source.into());
    let response = service.inlay_hint(create_params(uri, 3, 0));
    assert!(response.is_none());
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
    service.set_config(
        uri.clone(),
        ServiceConfig {
            inlay_hint: InlayHintOptions {
                types: true,
                ending: false,
                index: false,
            },
            ..Default::default()
        },
    );
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

#[test]
fn types_only() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $s (struct (field (mut i32))))
  (global anyref)
  (func (param (ref null 0))
    local.get 0
    struct.get 0 0
    global.get 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    service.set_config(
        uri.clone(),
        ServiceConfig {
            inlay_hint: InlayHintOptions {
                types: true,
                ending: false,
                index: false,
            },
            ..Default::default()
        },
    );
    let response = service.inlay_hint(create_params(uri, 8, 0));
    assert_json_snapshot!(response);
}

#[test]
fn ending_only() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f
    (block $block
      loop $loop
        if $if
        end)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    service.set_config(
        uri.clone(),
        ServiceConfig {
            inlay_hint: InlayHintOptions {
                types: false,
                ending: true,
                index: false,
            },
            ..Default::default()
        },
    );
    let response = service.inlay_hint(create_params(uri, 7, 0));
    assert_json_snapshot!(response);
}

#[test]
fn index_only() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global $g i32)
  (func (param $p i32) (param f32 i64) (local (ref 0)))
  (memory 1 2)
  (table 1 2 funcref)
  (type (struct (field (mut i32)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    service.set_config(
        uri.clone(),
        ServiceConfig {
            inlay_hint: InlayHintOptions {
                types: false,
                ending: false,
                index: true,
            },
            ..Default::default()
        },
    );
    let response = service.inlay_hint(create_params(uri, 7, 0));
    assert_json_snapshot!(response);
}
