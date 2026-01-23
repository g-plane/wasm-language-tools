use criterion::{Criterion, criterion_group, criterion_main};
use lspt::{
    ClientCapabilities, DefinitionParams, DocumentSymbolParams, HoverParams, InitializeParams, InlayHintParams,
    Position, Range, SemanticTokensClientCapabilities, SemanticTokensParams, TextDocumentClientCapabilities,
    TextDocumentIdentifier,
};
use std::hint::black_box;
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
    (func $conditional_compute (param $x i32) (result i32)
        local.get $x
        i32.const 10
        i32.gt_s
        if (result i32)
            local.get $x
            i32.const 2
            i32.mul
            i32.const 5
            i32.add
        else
            local.get $x
            i32.const 100
            i32.add
        end
    )
)
";
    let uri = "untitled:test".to_string();
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
    service.commit(&uri, source.into());

    c.bench_function("unchanged text", |b| {
        b.iter(|| {
            let document_symbols = service.document_symbol(black_box(DocumentSymbolParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                work_done_token: Default::default(),
                partial_result_token: Default::default(),
            }));
            black_box(document_symbols);

            let inlay_hints = service.inlay_hint(black_box(InlayHintParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                range: Range {
                    start: Position { line: 0, character: 0 },
                    end: Position { line: 14, character: 0 },
                },
                work_done_token: Default::default(),
            }));
            black_box(inlay_hints);

            let semantic_tokens = service.semantic_tokens_full(black_box(SemanticTokensParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                work_done_token: Default::default(),
                partial_result_token: Default::default(),
            }));
            black_box(semantic_tokens);

            let hover_func = service.hover(black_box(HoverParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                position: Position { line: 2, character: 12 },
                work_done_token: Default::default(),
            }));
            black_box(hover_func);
            let hover_param = service.hover(black_box(HoverParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                position: Position { line: 2, character: 39 },
                work_done_token: Default::default(),
            }));
            black_box(hover_param);
            let hover_global = service.hover(black_box(HoverParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                position: Position { line: 5, character: 14 },
                work_done_token: Default::default(),
            }));
            black_box(hover_global);

            let goto_def = service.goto_definition(black_box(DefinitionParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                position: Position {
                    line: 11,
                    character: 16,
                },
                work_done_token: Default::default(),
                partial_result_token: Default::default(),
            }));
            black_box(goto_def);
        })
    });
}

criterion_group!(benches, unchanged_text_bench);
criterion_main!(benches);
