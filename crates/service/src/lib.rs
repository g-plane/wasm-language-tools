#![doc = include_str!("../README.md")]

mod binder;
mod checker;
mod config;
mod data_set;
mod features;
mod helpers;
mod idx;
mod refactorings;
mod syntax_tree;
mod types_analyzer;
mod uri;

use self::features::SemanticTokenKind;
pub use crate::config::*;
use crate::{
    binder::SymbolTables,
    idx::Idents,
    syntax_tree::{SyntaxTree, SyntaxTreeCtx},
    types_analyzer::TypesAnalyzer,
    uri::{Uris, UrisCtx},
};
use indexmap::{IndexMap, IndexSet};
use lsp_types::{
    notification::DidChangeConfiguration, CallHierarchyServerCapability, CodeActionKind,
    CodeActionOptions, CodeActionProviderCapability, CompletionOptions, DeclarationCapability,
    DiagnosticOptions, DiagnosticServerCapabilities, FoldingRangeProviderCapability,
    HoverProviderCapability, InitializeParams, InitializeResult, OneOf, Registration,
    RegistrationParams, RenameOptions, SelectionRangeProviderCapability, SemanticTokenType,
    SemanticTokensClientCapabilities, SemanticTokensFullOptions, SemanticTokensLegend,
    SemanticTokensOptions, SemanticTokensServerCapabilities, ServerCapabilities, ServerInfo,
    SignatureHelpOptions, TextDocumentClientCapabilities, TextDocumentSyncCapability,
    TextDocumentSyncKind, TextDocumentSyncOptions, TextDocumentSyncSaveOptions,
    TypeDefinitionProviderCapability, Uri,
};
use rustc_hash::{FxBuildHasher, FxHashMap};
use salsa::{Database, ParallelDatabase, Snapshot};

#[salsa::database(Uris, Idents, SyntaxTree, SymbolTables, TypesAnalyzer)]
#[derive(Default)]
/// The language service comes with handlers for LSP requests.
///
/// The language service only does computation.
/// It doesn't require IO to read source file,
/// instead, you need to call [`commit`](LanguageService::commit) to add or update file.
/// Also, it doesn't process language server protocol.
/// You should call the corresponding method for each request.
///
/// To create a language service instance, you should call `LanguageService::default()`,
/// not the `initialize` method.
///
/// ​
pub struct LanguageService {
    storage: salsa::Storage<Self>,
    semantic_token_kinds: IndexSet<SemanticTokenKind, FxBuildHasher>,
    configs: FxHashMap<crate::uri::InternUri, ServiceConfig>,
    global_config: ServiceConfig,
}
impl Database for LanguageService {}
impl ParallelDatabase for LanguageService {
    fn snapshot(&self) -> Snapshot<Self> {
        Snapshot::new(LanguageService {
            storage: self.storage.snapshot(),
            semantic_token_kinds: self.semantic_token_kinds.clone(),
            configs: self.configs.clone(),
            global_config: self.global_config.clone(),
        })
    }
}

impl LanguageService {
    /// This method isn't used to create language service instance.
    /// Instead, you can call `LanguageService::default()` to create instance,
    /// then call this method when the language server is initializing.
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

        if let Some(config) = params
            .initialization_options
            .and_then(|config| serde_json::from_value(config).ok())
        {
            self.global_config = config;
        }

        InitializeResult {
            capabilities: ServerCapabilities {
                call_hierarchy_provider: Some(CallHierarchyServerCapability::Simple(true)),
                code_action_provider: Some(CodeActionProviderCapability::Options(
                    CodeActionOptions {
                        code_action_kinds: Some(vec![
                            CodeActionKind::QUICKFIX,
                            CodeActionKind::REFACTOR_REWRITE,
                            CodeActionKind::REFACTOR_INLINE,
                        ]),
                        resolve_provider: Some(false),
                        ..Default::default()
                    },
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(
                        [
                            '$', '(', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
                            'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
                            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.',
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
                document_formatting_provider: Some(OneOf::Left(true)),
                document_highlight_provider: Some(OneOf::Left(true)),
                document_range_formatting_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                inlay_hint_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Right(RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress_options: Default::default(),
                })),
                selection_range_provider: Some(SelectionRangeProviderCapability::Simple(true)),
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
                signature_help_provider: Some(SignatureHelpOptions {
                    trigger_characters: Some(['(', ')'].iter().map(char::to_string).collect()),
                    ..Default::default()
                }),
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

    #[inline]
    /// Commit a document to the service, usually called when handling `textDocument/didOpen` or
    /// `textDocument/didChange` notifications.
    pub fn commit(&mut self, uri: Uri, source: String) {
        let uri = self.uri(uri);
        self.set_source(uri, source);
    }

    #[inline]
    // This should be used internally.
    fn get_config(&self, uri: crate::uri::InternUri) -> &ServiceConfig {
        self.configs.get(&uri).unwrap_or(&self.global_config)
    }

    #[inline]
    /// Get configurations of all opened documents.
    pub fn get_configs(&self) -> impl Iterator<Item = (Uri, &ServiceConfig)> {
        self.configs
            .iter()
            .map(|(uri, config)| (self.lookup_uri(*uri), config))
    }

    #[inline]
    /// Update or insert configuration of a specific document.
    pub fn set_config(&mut self, uri: Uri, config: ServiceConfig) {
        self.configs.insert(self.uri(uri), config);
    }

    #[inline]
    /// Update global configuration.
    pub fn set_global_config(&mut self, config: ServiceConfig) {
        self.global_config = config;
    }

    #[inline]
    /// Get dynamically registered capabilities.
    pub fn dynamic_capabilities(&self) -> RegistrationParams {
        use lsp_types::notification::Notification;
        RegistrationParams {
            registrations: vec![Registration {
                id: DidChangeConfiguration::METHOD.into(),
                method: DidChangeConfiguration::METHOD.into(),
                register_options: None,
            }],
        }
    }

    #[inline]
    /// Check if the current request is cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.storage.salsa_runtime().is_current_revision_canceled()
    }

    #[inline]
    /// Fork to create a read-only language service snapshot
    /// which can be used in a different thread or async context.
    pub fn fork(&self) -> Snapshot<Self> {
        self.snapshot()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_and_set_configs() {
        let mut service = LanguageService::default();
        assert_eq!(service.get_configs().count(), 0);

        let uri = "untitled://test".parse::<Uri>().unwrap();
        service.set_config(uri.clone(), ServiceConfig::default());
        assert_eq!(service.get_configs().next().unwrap().0, uri);
    }
}
