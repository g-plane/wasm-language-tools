mod binder;
mod diag;
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
    DeclarationCapability, Diagnostic, DiagnosticSeverity, HoverProviderCapability,
    InitializeParams, InitializeResult, OneOf, Position, Range, RenameOptions, SemanticTokenType,
    SemanticTokensClientCapabilities, SemanticTokensFullOptions, SemanticTokensLegend,
    SemanticTokensOptions, SemanticTokensServerCapabilities, ServerCapabilities, ServerInfo,
    TextDocumentClientCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
    TextDocumentSyncOptions, TextDocumentSyncSaveOptions, TypeDefinitionProviderCapability, Uri,
};
use rowan::ast::{support::children, AstNode};
use rustc_hash::FxBuildHasher;
use salsa::{InternId, InternKey};
use wat_syntax::ast::Module;

#[salsa::database(Files, SymbolTables, TypesAnalyzer)]
#[derive(Default)]
struct LanguageServiceCtx {
    storage: salsa::Storage<Self>,
}
impl salsa::Database for LanguageServiceCtx {}

#[derive(Default)]
pub struct LanguageService {
    ctx: LanguageServiceCtx,
    semantic_token_kinds: IndexSet<SemanticTokenKind, FxBuildHasher>,
}

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
                }
            });
            self.semantic_token_kinds = kinds_map.keys().cloned().collect();
        }

        InitializeResult {
            capabilities: ServerCapabilities {
                definition_provider: Some(OneOf::Left(true)),
                type_definition_provider: Some(TypeDefinitionProviderCapability::Simple(true)),
                declaration_provider: Some(DeclarationCapability::Simple(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
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

    pub fn commit_file(&mut self, uri: Uri, source: String) -> Vec<Diagnostic> {
        let uri = self.ctx.uri(uri);
        self.ctx.set_source(uri, source);

        let mut diagnostics = self
            .ctx
            .parser_result(uri)
            .1
            .into_iter()
            .map(|diag| Diagnostic {
                range: diag.range,
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("wat".into()),
                message: diag.message,
                ..Default::default()
            })
            .collect::<Vec<_>>();

        let line_index = self.ctx.line_index(uri);
        diagnostics.extend(
            children::<Module>(&self.ctx.root(uri))
                .skip(1)
                .map(|module| {
                    let range = module.syntax().text_range();
                    let start = line_index.line_col(range.start());
                    let end = line_index.line_col(range.end());
                    Diagnostic {
                        range: Range::new(
                            Position::new(start.line, start.col),
                            Position::new(end.line, end.col),
                        ),
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("wat".into()),
                        message: "only one module is allowed in one file".into(),
                        ..Default::default()
                    }
                }),
        );

        diagnostics
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
