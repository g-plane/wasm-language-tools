use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lsp_types::{
    ClientCapabilities, DocumentSymbolParams, GotoDefinitionParams, HoverParams, InitializeParams,
    InlayHintParams, Position, Range, SemanticTokenType, SemanticTokensClientCapabilities,
    SemanticTokensParams, TextDocumentClientCapabilities, TextDocumentIdentifier,
    TextDocumentPositionParams, Uri,
};
use wat_service::LanguageService;

pub fn unchanged_text_bench(c: &mut Criterion) {
    let source = "
(module
    (func $f1 (param $p1 i32) (param $p2 i32) (result i32)
        (i32.add (local.get $p1) (local.get $p2))
    )
    (global $g1 f64 (f64.const 0))
    (func $f2 (result f64)
        (global.get $g1)
    )
    (type $t (func (result f64)))
    (func $f3 (type $t)
        (call $f2)
    )
)
";
    let uri = "untitled:test".parse::<Uri>().unwrap();
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
    service.commit_file(uri.clone(), source.into());

    c.bench_function("unchanged text", |b| {
        b.iter(|| {
            let document_symbols = service.document_symbol(black_box(DocumentSymbolParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                work_done_progress_params: Default::default(),
                partial_result_params: Default::default(),
            }));
            black_box(document_symbols);

            let inlay_hints = service.inlay_hint(black_box(InlayHintParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                range: Range::new(Position::new(0, 0), Position::new(14, 0)),
                work_done_progress_params: Default::default(),
            }));
            black_box(inlay_hints);

            let semantic_tokens = service.semantic_tokens_full(black_box(SemanticTokensParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                work_done_progress_params: Default::default(),
                partial_result_params: Default::default(),
            }));
            black_box(semantic_tokens);

            let hover_func = service.hover(black_box(HoverParams {
                text_document_position_params: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position::new(2, 12),
                },
                work_done_progress_params: Default::default(),
            }));
            black_box(hover_func);
            let hover_param = service.hover(black_box(HoverParams {
                text_document_position_params: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position::new(2, 39),
                },
                work_done_progress_params: Default::default(),
            }));
            black_box(hover_param);
            let hover_global = service.hover(black_box(HoverParams {
                text_document_position_params: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position::new(5, 14),
                },
                work_done_progress_params: Default::default(),
            }));
            black_box(hover_global);

            let goto_def = service.goto_definition(black_box(GotoDefinitionParams {
                text_document_position_params: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position::new(11, 16),
                },
                work_done_progress_params: Default::default(),
                partial_result_params: Default::default(),
            }));
            black_box(goto_def);
        })
    });
}

criterion_group!(benches, unchanged_text_bench);
criterion_main!(benches);
