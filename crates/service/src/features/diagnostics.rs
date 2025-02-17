use crate::{checker, uri::UrisCtx, LanguageService};
use lspt::{
    DocumentDiagnosticParams, PublishDiagnosticsParams, RelatedFullDocumentDiagnosticReport,
};

impl LanguageService {
    /// Handler for `textDocument/diagnostic` request.
    pub fn pull_diagnostics(
        &self,
        params: DocumentDiagnosticParams,
    ) -> RelatedFullDocumentDiagnosticReport {
        let diagnostics = checker::check(self, self.uri(params.text_document.uri));
        RelatedFullDocumentDiagnosticReport {
            kind: "full".into(),
            result_id: None,
            items: diagnostics,
            related_documents: None,
        }
    }

    /// Handler for `textDocument/publishDiagnostics` notification.
    pub fn publish_diagnostics(&self, uri: String) -> PublishDiagnosticsParams {
        PublishDiagnosticsParams {
            uri: uri.clone(),
            diagnostics: checker::check(self, self.uri(uri)),
            version: None,
        }
    }
}
