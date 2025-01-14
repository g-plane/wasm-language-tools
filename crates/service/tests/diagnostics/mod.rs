use lsp_types::{
    Diagnostic, DocumentDiagnosticParams, DocumentDiagnosticReport, DocumentDiagnosticReportResult,
    FullDocumentDiagnosticReport, RelatedFullDocumentDiagnosticReport, TextDocumentIdentifier, Uri,
};
use wat_service::{LanguageService, LintLevel, Lints, ServiceConfig};

mod dup_names;
mod global_mut;
mod immediates;
mod implicit_module;
mod import_occur;
mod multi_modules;
mod shadow;
mod syntax;
mod typeck;
mod undef;
mod unknown_instr;
mod unreachable;
mod unused;

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

fn calm(service: &mut LanguageService, uri: Uri) {
    service.set_config(
        uri,
        ServiceConfig {
            lint: Lints {
                unused: LintLevel::Allow,
                unreachable: LintLevel::Allow,
                ..Default::default()
            },
            ..Default::default()
        },
    );
}
