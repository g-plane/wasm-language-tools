#![doc = include_str!("../README.md")]

mod binder;
mod cfa;
mod checker;
mod config;
mod data_set;
mod deprecation;
mod document;
mod features;
mod helpers;
mod idx;
mod imex;
mod mutability;
mod refactorings;
mod types_analyzer;
mod uri;

pub use crate::config::*;
use crate::{
    document::Document,
    features::{SemanticTokenType, SemanticTokenTypes},
    uri::InternUri,
};
use indexmap::IndexMap;
use lspt::{
    CallHierarchyProvider, CodeActionKind, CodeActionOptions, CodeActionProvider, CodeLensOptions, CompletionOptions,
    DeclarationProvider, DefinitionProvider, DiagnosticOptions, DiagnosticProvider, DocumentFormattingProvider,
    DocumentHighlightProvider, DocumentRangeFormattingOptions, DocumentRangeFormattingProvider, DocumentSymbolProvider,
    ExecuteCommandOptions, FoldingRangeProvider, HoverProvider, InitializeParams, InitializeResult, InlayHintProvider,
    ReferencesProvider, RenameOptions, RenameProvider, SelectionRangeProvider, SemanticTokensClientCapabilities,
    SemanticTokensFull, SemanticTokensLegend, SemanticTokensOptions, SemanticTokensProvider, SemanticTokensRange,
    ServerCapabilities, ServerInfo, SignatureHelpOptions, TextDocumentClientCapabilities, TextDocumentSync,
    TextDocumentSyncKind, TextDocumentSyncOptions, TextDocumentSyncSave, TypeDefinitionProvider, TypeHierarchyProvider,
};
use parking_lot::RwLock;
use rustc_hash::{FxBuildHasher, FxHashMap};
use salsa::Database;
use std::{
    panic::{AssertUnwindSafe, UnwindSafe},
    sync::Arc,
};

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
    semantic_token_types: Arc<SemanticTokenTypes>,
    documents: Arc<RwLock<FxHashMap<InternUri, Document>>>,
    global_config: Arc<ServiceConfig>,
    configs: Arc<RwLock<FxHashMap<InternUri, ConfigState>>>,
    support_pull_config: bool,
}
#[salsa::db]
impl Database for LanguageService {}

impl LanguageService {
    /// Handler for `initialize` request.
    ///
    /// This method isn't used to create language service instance.
    /// Instead, you can call `LanguageService::default()` to create instance,
    /// then call this method when the language server is initializing.
    pub fn initialize(&mut self, params: InitializeParams) -> InitializeResult {
        let mut types_map = IndexMap::<_, _, FxBuildHasher>::default();
        if let Some(TextDocumentClientCapabilities {
            semantic_tokens: Some(SemanticTokensClientCapabilities { token_types, .. }),
            ..
        }) = params.capabilities.text_document
        {
            types_map = token_types
                .iter()
                .filter_map(|token_type| {
                    let internal_type = match &**token_type {
                        "type" => SemanticTokenType::Type,
                        "parameter" => SemanticTokenType::Param,
                        "variable" => SemanticTokenType::Var,
                        "function" => SemanticTokenType::Func,
                        "keyword" => SemanticTokenType::Keyword,
                        "comment" => SemanticTokenType::Comment,
                        "string" => SemanticTokenType::String,
                        "number" => SemanticTokenType::Number,
                        "operator" => SemanticTokenType::Op,
                        _ => return None,
                    };
                    Some((internal_type, token_type.clone()))
                })
                .collect();
            self.semantic_token_types = Arc::new(types_map.keys().cloned().collect());
        }

        self.support_pull_config = matches!(
            params.capabilities.workspace.as_ref().and_then(|it| it.configuration),
            Some(true)
        );
        if let Some(config) = params
            .initialization_options
            .and_then(|config| serde_json::from_value(config).ok())
        {
            self.global_config = Arc::new(config);
        }

        InitializeResult {
            capabilities: ServerCapabilities {
                call_hierarchy_provider: Some(CallHierarchyProvider::Bool(true)),
                code_action_provider: Some(CodeActionProvider::Options(CodeActionOptions {
                    code_action_kinds: Some(vec![
                        CodeActionKind::QuickFix,
                        CodeActionKind::RefactorRewrite,
                        CodeActionKind::RefactorInline,
                    ]),
                    resolve_provider: Some(false),
                    ..Default::default()
                })),
                code_lens_provider: Some(CodeLensOptions {
                    resolve_provider: Some(true),
                    ..Default::default()
                }),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(
                        [
                            '$', '(', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                            'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7',
                            '8', '9', '.', '@',
                        ]
                        .iter()
                        .map(char::to_string)
                        .collect(),
                    ),
                    all_commit_characters: Some(vec![")".into()]),
                    ..Default::default()
                }),
                definition_provider: Some(DefinitionProvider::Bool(true)),
                diagnostic_provider: Some(DiagnosticProvider::Options(DiagnosticOptions {
                    identifier: Some("wat".into()),
                    inter_file_dependencies: false,
                    workspace_diagnostics: false,
                    ..Default::default()
                })),
                type_definition_provider: Some(TypeDefinitionProvider::Bool(true)),
                declaration_provider: Some(DeclarationProvider::Bool(true)),
                document_formatting_provider: Some(DocumentFormattingProvider::Bool(true)),
                document_highlight_provider: Some(DocumentHighlightProvider::Bool(true)),
                document_range_formatting_provider: Some(DocumentRangeFormattingProvider::Options(
                    DocumentRangeFormattingOptions {
                        ranges_support: Some(true),
                        ..Default::default()
                    },
                )),
                document_symbol_provider: Some(DocumentSymbolProvider::Bool(true)),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["wasmLanguageTools.__generateControlFlowGraphDot".into()],
                    ..Default::default()
                }),
                folding_range_provider: Some(FoldingRangeProvider::Bool(true)),
                hover_provider: Some(HoverProvider::Bool(true)),
                inlay_hint_provider: Some(InlayHintProvider::Bool(true)),
                references_provider: Some(ReferencesProvider::Bool(true)),
                rename_provider: Some(RenameProvider::Options(RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress: Default::default(),
                })),
                selection_range_provider: Some(SelectionRangeProvider::Bool(true)),
                semantic_tokens_provider: Some(SemanticTokensProvider::Options(SemanticTokensOptions {
                    legend: SemanticTokensLegend {
                        token_types: types_map.into_values().collect(),
                        token_modifiers: vec!["mutable".into()],
                    },
                    range: Some(SemanticTokensRange::Bool(true)),
                    full: Some(SemanticTokensFull::Bool(true)),
                    ..Default::default()
                })),
                signature_help_provider: Some(SignatureHelpOptions {
                    trigger_characters: Some(['(', ')'].iter().map(char::to_string).collect()),
                    ..Default::default()
                }),
                type_hierarchy_provider: Some(TypeHierarchyProvider::Bool(true)),
                text_document_sync: Some(TextDocumentSync::Options(TextDocumentSyncOptions {
                    open_close: Some(true),
                    change: Some(TextDocumentSyncKind::Incremental),
                    will_save: Some(false),
                    will_save_wait_until: Some(false),
                    save: Some(TextDocumentSyncSave::Bool(false)),
                })),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "WebAssembly Language Tools".into(),
                version: Some(env!("CARGO_PKG_VERSION").into()),
            }),
        }
    }

    /// Run computation that may be cancelled on pending write.
    /// It should only run read-only computation for LSP requests.
    fn with_db<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&dyn salsa::Database) -> R + UnwindSafe,
    {
        let service = AssertUnwindSafe(self);
        salsa::Cancelled::catch(|| f(*service)).ok()
    }
}
