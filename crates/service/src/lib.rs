#![doc = include_str!("../README.md")]

mod binder;
mod checker;
mod config;
mod data_set;
mod features;
mod helpers;
mod idx;
mod mutability;
mod refactorings;
mod syntax_tree;
mod types_analyzer;
mod uri;

use self::features::SemanticTokenKind;
pub use crate::config::*;
use crate::{
    binder::SymbolTables,
    idx::Idents,
    mutability::Mutabilities,
    syntax_tree::{SyntaxTree, SyntaxTreeCtx},
    types_analyzer::TypesAnalyzer,
    uri::{Uris, UrisCtx},
};
use indexmap::{IndexMap, IndexSet};
use lspt::{
    CodeActionKind, CodeActionOptions, CompletionOptions, DiagnosticOptions,
    DidChangeConfigurationNotification, InitializeParams, InitializeResult, Registration,
    RegistrationParams, RenameOptions, SemanticTokensClientCapabilities, SemanticTokensLegend,
    SemanticTokensOptions, ServerCapabilities, ServerInfo, SignatureHelpOptions,
    TextDocumentClientCapabilities, TextDocumentSyncKind, TextDocumentSyncOptions, Union2, Union3,
};
use rustc_hash::{FxBuildHasher, FxHashMap};
use salsa::{Database, ParallelDatabase, Snapshot};

#[salsa::database(Uris, Idents, SyntaxTree, SymbolTables, TypesAnalyzer, Mutabilities)]
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
            kinds_map = token_types
                .iter()
                .filter_map(|token_type| {
                    let internal_kind = match &**token_type {
                        "type" => SemanticTokenKind::Type,
                        "parameter" => SemanticTokenKind::Param,
                        "variable" => SemanticTokenKind::Var,
                        "function" => SemanticTokenKind::Func,
                        "keyword" => SemanticTokenKind::Keyword,
                        "comment" => SemanticTokenKind::Comment,
                        "string" => SemanticTokenKind::String,
                        "number" => SemanticTokenKind::Number,
                        "operator" => SemanticTokenKind::Op,
                        _ => return None,
                    };
                    Some((internal_kind, token_type.clone()))
                })
                .collect();
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
                call_hierarchy_provider: Some(Union3::A(true)),
                code_action_provider: Some(Union2::B(CodeActionOptions {
                    code_action_kinds: Some(vec![
                        CodeActionKind::QuickFix,
                        CodeActionKind::RefactorRewrite,
                        CodeActionKind::RefactorInline,
                    ]),
                    resolve_provider: Some(false),
                    ..Default::default()
                })),
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
                definition_provider: Some(Union2::A(true)),
                diagnostic_provider: Some(Union2::A(DiagnosticOptions {
                    identifier: Some("wat".into()),
                    inter_file_dependencies: false,
                    workspace_diagnostics: false,
                    ..Default::default()
                })),
                type_definition_provider: Some(Union3::A(true)),
                declaration_provider: Some(Union3::A(true)),
                document_formatting_provider: Some(Union2::A(true)),
                document_highlight_provider: Some(Union2::A(true)),
                document_range_formatting_provider: Some(Union2::A(true)),
                document_symbol_provider: Some(Union2::A(true)),
                folding_range_provider: Some(Union3::A(true)),
                hover_provider: Some(Union2::A(true)),
                inlay_hint_provider: Some(Union3::A(true)),
                references_provider: Some(Union2::A(true)),
                rename_provider: Some(Union2::B(RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress: Default::default(),
                })),
                selection_range_provider: Some(Union3::A(true)),
                semantic_tokens_provider: Some(Union2::A(SemanticTokensOptions {
                    legend: SemanticTokensLegend {
                        token_types: kinds_map.into_values().collect(),
                        token_modifiers: vec![],
                    },
                    range: Some(Union2::A(true)),
                    full: Some(Union2::A(true)),
                    ..Default::default()
                })),
                signature_help_provider: Some(SignatureHelpOptions {
                    trigger_characters: Some(['(', ')'].iter().map(char::to_string).collect()),
                    ..Default::default()
                }),
                type_hierarchy_provider: Some(Union3::A(true)),
                text_document_sync: Some(Union2::A(TextDocumentSyncOptions {
                    open_close: Some(true),
                    change: Some(TextDocumentSyncKind::Full),
                    will_save: Some(false),
                    will_save_wait_until: Some(false),
                    save: Some(Union2::A(false)),
                })),
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
    pub fn commit(&mut self, uri: String, source: String) {
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
    pub fn get_configs(&self) -> impl Iterator<Item = (String, &ServiceConfig)> {
        self.configs
            .iter()
            .map(|(uri, config)| (self.lookup_uri(*uri), config))
    }

    #[inline]
    /// Update or insert configuration of a specific document.
    pub fn set_config(&mut self, uri: String, config: ServiceConfig) {
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
        use lspt::Notification;
        RegistrationParams {
            registrations: vec![Registration {
                id: DidChangeConfigurationNotification::METHOD.into(),
                method: DidChangeConfigurationNotification::METHOD.into(),
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

        let uri = "untitled://test".to_string();
        service.set_config(uri.clone(), ServiceConfig::default());
        assert_eq!(service.get_configs().next().unwrap().0, uri);
    }
}
