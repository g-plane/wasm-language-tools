use insta::assert_json_snapshot;
use lsp_types::{
    ClientCapabilities, InitializeParams, Position, Range, SemanticTokenType,
    SemanticTokensClientCapabilities, SemanticTokensParams, SemanticTokensRangeParams,
    TextDocumentClientCapabilities, TextDocumentIdentifier, Uri,
};
use wat_service::LanguageService;

fn create_service() -> LanguageService {
    let mut service = LanguageService::default();
    service.initialize(InitializeParams {
        capabilities: ClientCapabilities {
            text_document: Some(TextDocumentClientCapabilities {
                semantic_tokens: Some(SemanticTokensClientCapabilities {
                    token_types: vec![
                        SemanticTokenType::NAMESPACE,
                        SemanticTokenType::TYPE,
                        SemanticTokenType::CLASS,
                        SemanticTokenType::ENUM,
                        SemanticTokenType::INTERFACE,
                        SemanticTokenType::STRUCT,
                        SemanticTokenType::TYPE_PARAMETER,
                        SemanticTokenType::PARAMETER,
                        SemanticTokenType::VARIABLE,
                        SemanticTokenType::PROPERTY,
                        SemanticTokenType::ENUM_MEMBER,
                        SemanticTokenType::EVENT,
                        SemanticTokenType::FUNCTION,
                        SemanticTokenType::METHOD,
                        SemanticTokenType::MACRO,
                        SemanticTokenType::KEYWORD,
                        SemanticTokenType::MODIFIER,
                        SemanticTokenType::COMMENT,
                        SemanticTokenType::STRING,
                        SemanticTokenType::NUMBER,
                        SemanticTokenType::REGEXP,
                        SemanticTokenType::OPERATOR,
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
        (f32.const 0.1) (f32.const -1.2e+2)
    )
    (start $func)
    (start 0)
)
(; block ;)
;; line
"#;

#[test]
fn full() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let mut service = create_service();
    service.commit_file(uri.clone(), SOURCE.into());
    let response = service.semantic_tokens_full(SemanticTokensParams {
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
        text_document: TextDocumentIdentifier { uri },
    });
    assert_json_snapshot!(response);
}

#[test]
fn range() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let mut service = create_service();
    service.commit_file(uri.clone(), SOURCE.into());
    let response = service.semantic_tokens_range(SemanticTokensRangeParams {
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
        text_document: TextDocumentIdentifier { uri },
        range: Range::new(Position::new(4, 19), Position::new(5, 21)),
    });
    assert_json_snapshot!(response);
}

#[test]
fn range_not_boundary() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let mut service = create_service();
    service.commit_file(uri.clone(), SOURCE.into());
    let response = service.semantic_tokens_range(SemanticTokensRangeParams {
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
        text_document: TextDocumentIdentifier { uri },
        range: Range::new(Position::new(4, 23), Position::new(5, 16)),
    });
    assert_json_snapshot!(response);
}
