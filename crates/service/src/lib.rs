mod binder;
mod diag;
mod features;
mod files;
mod helpers;

use crate::{
    binder::SymbolTables,
    files::{Files, FilesCtx},
};
use lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range, Uri};
use rowan::ast::AstNode;

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
    pub fn set_file(&mut self, uri: Uri, source: String) {
        self.ctx.set_source(uri, source);
    }

    pub fn fetch_syntax_errors(&self, uri: Uri) -> Vec<Diagnostic> {
        let mut diagnostics = self
            .ctx
            .parser_result(uri.clone())
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

        let line_index = self.ctx.line_index(uri.clone());
        diagnostics.extend(self.ctx.root(uri.clone()).modules().skip(1).map(|module| {
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
