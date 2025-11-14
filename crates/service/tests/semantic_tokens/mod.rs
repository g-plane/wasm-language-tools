use insta::assert_json_snapshot;
use lspt::{
    ClientCapabilities, InitializeParams, Position, Range, SemanticTokensClientCapabilities,
    SemanticTokensParams, SemanticTokensRangeParams, TextDocumentClientCapabilities,
    TextDocumentIdentifier,
};
use wat_service::LanguageService;

fn create_service() -> LanguageService {
    let mut service = LanguageService::default();
    service.initialize(InitializeParams {
        capabilities: ClientCapabilities {
            text_document: Some(TextDocumentClientCapabilities {
                semantic_tokens: Some(SemanticTokensClientCapabilities {
                    token_types: vec![
                        "namespace".into(),
                        "type".into(),
                        "class".into(),
                        "enum".into(),
                        "interface".into(),
                        "struct".into(),
                        "typeParameter".into(),
                        "parameter".into(),
                        "variable".into(),
                        "property".into(),
                        "enumMember".into(),
                        "event".into(),
                        "function".into(),
                        "method".into(),
                        "macro".into(),
                        "keyword".into(),
                        "modifier".into(),
                        "comment".into(),
                        "string".into(),
                        "number".into(),
                        "regexp".into(),
                        "operator".into(),
                    ],
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    });
    service
}

const SOURCE: &str = r#"
(module
    (func $func (export "func") (param $param i32) (result i32) (local $local i32)
        (local.get $param) (local.get 0) (i32.const 0)
        (local.get $local) (local.get 1) (i32.const 1)
        (call $func) (ref.func $func) (call 0) (ref.func 0)
        (return_call $func) (return_call 0)
        (f32.const 0.1) (f32.const -1.2e+2)
    )
    (start $func)
    (start 0)
    (export "" (func $func))
    (export "" (func 0))
    (elem func $func)
    (global $g (mut))
    (type $arr (array (mut)))
    (type $struct (struct (field (mut))))
    (func
        global.get $g
        array.get $arr
        struct.get $struct 0)
    (@annotation "string")
)
(; block ;)
;; line
"#;

#[test]
fn full() {
    let uri = "untitled:test".to_string();
    let mut service = create_service();
    service.commit(uri.clone(), SOURCE.into());
    let response = service.semantic_tokens_full(SemanticTokensParams {
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
        text_document: TextDocumentIdentifier { uri },
    });
    assert_json_snapshot!(response);
}

#[test]
fn range() {
    let uri = "untitled:test".to_string();
    let mut service = create_service();
    service.commit(uri.clone(), SOURCE.into());
    let response = service.semantic_tokens_range(SemanticTokensRangeParams {
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
        text_document: TextDocumentIdentifier { uri },
        range: Range {
            start: Position {
                line: 4,
                character: 19,
            },
            end: Position {
                line: 5,
                character: 21,
            },
        },
    });
    assert_json_snapshot!(response);
}

#[test]
fn range_not_boundary() {
    let uri = "untitled:test".to_string();
    let mut service = create_service();
    service.commit(uri.clone(), SOURCE.into());
    let response = service.semantic_tokens_range(SemanticTokensRangeParams {
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
        text_document: TextDocumentIdentifier { uri },
        range: Range {
            start: Position {
                line: 4,
                character: 23,
            },
            end: Position {
                line: 5,
                character: 16,
            },
        },
    });
    assert_json_snapshot!(response);
}
