mod binder;
mod diag;
mod features;
mod files;
mod helpers;

use crate::{
    binder::SymbolTables,
    files::{Files, FilesCtx},
};
use lsp_types::{
    Diagnostic, DiagnosticSeverity, OneOf, Position, Range, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind, TypeDefinitionProviderCapability, Uri,
};
use rowan::ast::AstNode;
use salsa::{InternId, InternKey};

#[salsa::database(Files, SymbolTables)]
#[derive(Default)]
struct LanguageServiceCtx {
    storage: salsa::Storage<Self>,
}
impl salsa::Database for LanguageServiceCtx {}

#[derive(Default)]
pub struct LanguageService {
    ctx: LanguageServiceCtx,
}

impl LanguageService {
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
        diagnostics.extend(self.ctx.root(uri).modules().skip(1).map(|module| {
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
        }));

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

pub fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        definition_provider: Some(OneOf::Left(true)),
        type_definition_provider: Some(TypeDefinitionProviderCapability::Simple(true)),
        document_symbol_provider: Some(OneOf::Left(true)),
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        ..Default::default()
    }
}
