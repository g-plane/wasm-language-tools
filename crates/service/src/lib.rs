mod binder;
mod checker;
mod dataset;
mod features;
mod files;
mod helpers;
mod types_analyzer;

use self::features::SemanticTokenKind;
use crate::{
    binder::SymbolTables,
    files::{Files, FilesCtx},
    types_analyzer::TypesAnalyzer,
};
use indexmap::{IndexMap, IndexSet};
use lsp_types::{
    CallHierarchyServerCapability, CodeActionKind, CodeActionOptions, CodeActionProviderCapability,
    CompletionOptions, DeclarationCapability, DiagnosticOptions, DiagnosticServerCapabilities,
    HoverProviderCapability, InitializeParams, InitializeResult, OneOf, RenameOptions,
    SemanticTokenType, SemanticTokensClientCapabilities, SemanticTokensFullOptions,
    SemanticTokensLegend, SemanticTokensOptions, SemanticTokensServerCapabilities,
    ServerCapabilities, ServerInfo, TextDocumentClientCapabilities, TextDocumentSyncCapability,
    TextDocumentSyncKind, TextDocumentSyncOptions, TextDocumentSyncSaveOptions,
    TypeDefinitionProviderCapability, Uri,
};
use rustc_hash::FxBuildHasher;
use salsa::{InternId, InternKey};

#[salsa::database(Files, SymbolTables, TypesAnalyzer)]
#[derive(Default)]
pub struct LanguageService {
    storage: salsa::Storage<Self>,
    semantic_token_kinds: IndexSet<SemanticTokenKind, FxBuildHasher>,
}
impl salsa::Database for LanguageService {}

impl LanguageService {
    pub fn initialize(&mut self, params: InitializeParams) -> InitializeResult {
        let mut kinds_map = IndexMap::<_, _, FxBuildHasher>::default();
        if let Some(TextDocumentClientCapabilities {
            semantic_tokens: Some(SemanticTokensClientCapabilities { token_types, .. }),
            ..
        }) = params.capabilities.text_document
        {
            token_types.iter().for_each(|token_type| {
                if *token_type == SemanticTokenType::TYPE {
                    kinds_map.insert(SemanticTokenKind::Type, SemanticTokenType::TYPE);
                } else if *token_type == SemanticTokenType::PARAMETER {
                    kinds_map.insert(SemanticTokenKind::Param, SemanticTokenType::PARAMETER);
                } else if *token_type == SemanticTokenType::VARIABLE {
                    kinds_map.insert(SemanticTokenKind::Var, SemanticTokenType::VARIABLE);
                } else if *token_type == SemanticTokenType::FUNCTION {
                    kinds_map.insert(SemanticTokenKind::Func, SemanticTokenType::FUNCTION);
                } else if *token_type == SemanticTokenType::KEYWORD {
                    kinds_map.insert(SemanticTokenKind::Keyword, SemanticTokenType::KEYWORD);
                } else if *token_type == SemanticTokenType::COMMENT {
                    kinds_map.insert(SemanticTokenKind::Comment, SemanticTokenType::COMMENT);
                } else if *token_type == SemanticTokenType::STRING {
                    kinds_map.insert(SemanticTokenKind::String, SemanticTokenType::STRING);
                } else if *token_type == SemanticTokenType::NUMBER {
                    kinds_map.insert(SemanticTokenKind::Number, SemanticTokenType::NUMBER);
                } else if *token_type == SemanticTokenType::OPERATOR {
                    kinds_map.insert(SemanticTokenKind::Op, SemanticTokenType::OPERATOR);
                }
            });
            self.semantic_token_kinds = kinds_map.keys().cloned().collect();
        }

        InitializeResult {
            capabilities: ServerCapabilities {
                call_hierarchy_provider: Some(CallHierarchyServerCapability::Simple(true)),
                code_action_provider: Some(CodeActionProviderCapability::Options(
                    CodeActionOptions {
                        code_action_kinds: Some(vec![CodeActionKind::QUICKFIX]),
                        resolve_provider: Some(false),
                        ..Default::default()
                    },
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(
                        [
                            '$', '(', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
                            'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
                            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
                        ]
                        .iter()
                        .map(char::to_string)
                        .collect(),
                    ),
                    all_commit_characters: Some(vec![")".into()]),
                    ..Default::default()
                }),
                definition_provider: Some(OneOf::Left(true)),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        identifier: Some("wat".into()),
                        inter_file_dependencies: false,
                        workspace_diagnostics: false,
                        ..Default::default()
                    },
                )),
                type_definition_provider: Some(TypeDefinitionProviderCapability::Simple(true)),
                declaration_provider: Some(DeclarationCapability::Simple(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                inlay_hint_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Right(RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress_options: Default::default(),
                })),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            legend: SemanticTokensLegend {
                                token_types: kinds_map.into_values().collect(),
                                token_modifiers: vec![],
                            },
                            range: Some(true),
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                            ..Default::default()
                        },
                    ),
                ),
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        will_save: Some(false),
                        will_save_wait_until: Some(false),
                        save: Some(TextDocumentSyncSaveOptions::Supported(false)),
                    },
                )),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "WebAssembly Language Tools".into(),
                version: Some(env!("CARGO_PKG_VERSION").into()),
            }),
        }
    }

    /// Commit a file to the service, usually called when handling `textDocument/didOpen` or
    /// `textDocument/didChange` notifications.
    pub fn commit_file(&mut self, uri: Uri, source: String) {
        let uri = self.uri(uri);
        self.set_source(uri, source);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct InternUri(InternId);
impl InternKey for InternUri {
    fn from_intern_id(v: salsa::InternId) -> Self {
        InternUri(v)
    }
    fn as_intern_id(&self) -> InternId {
        self.0
    }
}
