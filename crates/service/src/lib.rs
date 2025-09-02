#![doc = include_str!("../README.md")]

mod binder;
mod checker;
mod config;
mod data_set;
mod document;
mod features;
mod helpers;
mod idx;
mod mutability;
mod refactorings;
mod types_analyzer;
mod uri;

pub use crate::config::*;
use crate::{document::Document, features::SemanticTokenKind, uri::InternUri};
use indexmap::{IndexMap, IndexSet};
use lspt::{
    CodeActionKind, CodeActionOptions, CompletionOptions, DiagnosticOptions, InitializeParams,
    InitializeResult, Registration, RegistrationParams, RenameOptions,
    SemanticTokensClientCapabilities, SemanticTokensLegend, SemanticTokensOptions,
    ServerCapabilities, ServerInfo, SignatureHelpOptions, TextDocumentClientCapabilities,
    TextDocumentSyncKind, TextDocumentSyncOptions, Union2, Union3,
    notification::DidChangeConfigurationNotification,
};
use rustc_hash::{FxBuildHasher, FxHashMap};
use salsa::{Database, Setter};

#[salsa::db]
#[derive(Clone, Default)]
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
pub struct LanguageService {
    storage: salsa::Storage<Self>,
    semantic_token_kinds: IndexSet<SemanticTokenKind, FxBuildHasher>,
    documents: FxHashMap<InternUri, Document>,
    global_config: ServiceConfig,
}
#[salsa::db]
impl Database for LanguageService {}

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
    // This should be used internally.
    fn get_config(&self, document: Document) -> &ServiceConfig {
        document.config(self).unwrap_or(&self.global_config)
    }

    #[inline]
    /// Get configurations of all opened documents.
    pub fn get_configs(&self) -> impl Iterator<Item = (String, &ServiceConfig)> {
        self.documents.iter().filter_map(|(uri, document)| {
            document.config(self).map(|config| (uri.raw(self), config))
        })
    }

    #[inline]
    /// Update or insert configuration of a specific document.
    pub fn set_config(&mut self, uri: String, config: ServiceConfig) {
        if let Some(document) = self.get_document(uri) {
            document.set_config(self).to(Some(config));
        }
    }

    #[inline]
    /// Update global configuration.
    pub fn set_global_config(&mut self, config: ServiceConfig) {
        self.global_config = config;
    }

    #[inline]
    /// Get dynamically registered capabilities.
    pub fn dynamic_capabilities(&self) -> RegistrationParams {
        use lspt::notification::Notification;
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
        false
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
        service.commit(uri.clone(), "".into());
        service.set_config(uri.clone(), ServiceConfig::default());
        assert_eq!(service.get_configs().next().unwrap().0, uri);
    }
}
