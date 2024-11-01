use lsp_types::{
    Diagnostic, DocumentDiagnosticParams, DocumentDiagnosticReport, DocumentDiagnosticReportResult,
    FullDocumentDiagnosticReport, RelatedFullDocumentDiagnosticReport, TextDocumentIdentifier, Uri,
};

#[cfg(test)]
mod multi_modules;
#[cfg(test)]
mod typeck;
#[cfg(test)]
mod undef;

fn create_params(uri: Uri) -> DocumentDiagnosticParams {
    DocumentDiagnosticParams {
        text_document: TextDocumentIdentifier { uri },
        identifier: Some("wat".into()),
        previous_result_id: None,
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
    }
}

fn pick_diagnostics(response: DocumentDiagnosticReportResult) -> Vec<Diagnostic> {
    if let DocumentDiagnosticReportResult::Report(DocumentDiagnosticReport::Full(
        RelatedFullDocumentDiagnosticReport {
            full_document_diagnostic_report: FullDocumentDiagnosticReport { items, .. },
            ..
        },
    )) = response
    {
        items
    } else {
        panic!("expected full document diagnostic report");
    }
}
