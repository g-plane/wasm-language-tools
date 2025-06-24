use criterion::{criterion_group, criterion_main, Criterion};
use lspt::{
    ClientCapabilities, CompletionContext, CompletionParams, CompletionTriggerKind,
    DocumentSymbolParams, InitializeParams, InlayHintParams, Position, Range,
    SemanticTokensClientCapabilities, SemanticTokensParams, TextDocumentClientCapabilities,
    TextDocumentIdentifier,
};
use std::hint::black_box;
use wat_service::LanguageService;

pub fn changed_text_bench(c: &mut Criterion) {
    let uri = "untitled:test".to_string();
    c.bench_function("changed text", |b| {
        b.iter(|| {
            let mut source = "
(module
    (func $f1 (param $p1 i32) (param $p2 i32) (result i32)
        (i32.add (local.get) (local.get $p2))
    )
    (global $g1 f64 (f64.const 0))
    (func $f2 (result f64)
        (global.get $g1)
    )
    (type $t (func (result f64)))
    (func $f3 (type $t)
        (call)
    )
)
"
            .to_string();
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
            service.commit(uri.clone(), source.clone());
            requests_on_changed(&mut service, &uri);

            let mut insert_char = |offset, char, line, col| {
                source.insert(offset, char);
                service.commit(uri.clone(), source.clone());
                let completions = service.completion(black_box(CompletionParams {
                    context: Some(CompletionContext {
                        trigger_character: Some(char.to_string()),
                        trigger_kind: CompletionTriggerKind::TriggerCharacter,
                    }),
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position {
                        line,
                        character: col,
                    },
                    work_done_token: Default::default(),
                    partial_result_token: Default::default(),
                }));
                black_box(completions);
                requests_on_changed(&mut service, &uri);
            };
            insert_char(95, ' ', 3, 27);
            insert_char(96, '$', 3, 28);
            insert_char(97, 'p', 3, 29);
            insert_char(287, ' ', 11, 13);
            insert_char(288, '$', 11, 14);
            insert_char(289, 'f', 11, 15);
        })
    });
}

fn requests_on_changed(service: &mut LanguageService, uri: &str) {
    let document_symbols = service.document_symbol(black_box(DocumentSymbolParams {
        text_document: TextDocumentIdentifier {
            uri: uri.to_string(),
        },
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    }));
    black_box(document_symbols);

    let inlay_hints = service.inlay_hint(black_box(InlayHintParams {
        text_document: TextDocumentIdentifier {
            uri: uri.to_string(),
        },
        range: Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 14,
                character: 0,
            },
        },
        work_done_token: Default::default(),
    }));
    black_box(inlay_hints);

    let semantic_tokens = service.semantic_tokens_full(black_box(SemanticTokensParams {
        text_document: TextDocumentIdentifier {
            uri: uri.to_string(),
        },
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    }));
    black_box(semantic_tokens);
}

criterion_group!(benches, changed_text_bench);
criterion_main!(benches);
