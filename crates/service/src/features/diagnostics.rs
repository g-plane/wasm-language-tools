use crate::{LanguageService, checker};
use lspt::{
    DocumentDiagnosticParams, PublishDiagnosticsParams, RelatedFullDocumentDiagnosticReport,
};

impl LanguageService {
    /// Handler for `textDocument/diagnostic` request.
    pub fn pull_diagnostics(
        &self,
        params: DocumentDiagnosticParams,
    ) -> RelatedFullDocumentDiagnosticReport {
        let diagnostics = self
            .get_document(params.text_document.uri)
            .map(|document| checker::check(self, *document))
            .unwrap_or_default();
        RelatedFullDocumentDiagnosticReport {
            kind: "full".into(),
            result_id: None,
            items: diagnostics,
            related_documents: None,
        }
    }

    /// Handler for `textDocument/publishDiagnostics` notification.
    pub fn publish_diagnostics(&self, uri: String) -> PublishDiagnosticsParams {
        let diagnostics = self
            .get_document(&uri)
            .map(|document| checker::check(self, *document))
            .unwrap_or_default();
        PublishDiagnosticsParams {
            uri,
            diagnostics,
            version: None,
        }
    }
}
