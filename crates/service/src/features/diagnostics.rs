use crate::{checker, files::FilesCtx, LanguageService};
use lsp_types::{
    DocumentDiagnosticParams, DocumentDiagnosticReport, DocumentDiagnosticReportResult,
    FullDocumentDiagnosticReport, PublishDiagnosticsParams, RelatedFullDocumentDiagnosticReport,
    Uri,
};

impl LanguageService {
    pub fn pull_diagnostics(
        &self,
        params: DocumentDiagnosticParams,
    ) -> DocumentDiagnosticReportResult {
        let diagnostics = checker::check(self, self.uri(params.text_document.uri));
        DocumentDiagnosticReportResult::Report(DocumentDiagnosticReport::Full(
            RelatedFullDocumentDiagnosticReport {
                related_documents: None,
                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                    result_id: None,
                    items: diagnostics,
                },
            },
        ))
    }

    pub fn publish_diagnostics(&self, uri: Uri) -> PublishDiagnosticsParams {
        PublishDiagnosticsParams {
            uri: uri.clone(),
            diagnostics: checker::check(self, self.uri(uri)),
            version: None,
        }
    }
}
