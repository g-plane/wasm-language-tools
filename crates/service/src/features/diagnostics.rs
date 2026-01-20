use crate::{LanguageService, checker, uri::InternUri};
use lspt::{
    Diagnostic, DocumentDiagnosticParams, PublishDiagnosticsParams,
    RelatedFullDocumentDiagnosticReport,
};

impl LanguageService {
    /// Handler for `textDocument/diagnostic` request.
    pub fn pull_diagnostics(
        &self,
        params: DocumentDiagnosticParams,
    ) -> RelatedFullDocumentDiagnosticReport {
        RelatedFullDocumentDiagnosticReport {
            kind: "full".into(),
            result_id: None,
            items: get_diagnostics(self, &params.text_document.uri),
            related_documents: None,
        }
    }

    /// Handler for `textDocument/publishDiagnostics` notification.
    pub fn publish_diagnostics(&self, uri: String) -> PublishDiagnosticsParams {
        let diagnostics = get_diagnostics(self, &uri);
        PublishDiagnosticsParams {
            uri,
            diagnostics,
            version: None,
        }
    }
}

fn get_diagnostics(service: &LanguageService, uri: &str) -> Vec<Diagnostic> {
    // Some clients like VS Code support pulling configuration per document.
    // In that case, we won't use global configuration,
    // but document-specific configuration may not be available if client doesn't send it yet.
    // If it isn't ready, we will skip the checker to avoid diagnostics flickering.
    if let Some(document) = service.get_document(uri)
        && let Some(config_state) = service.configs.get(&InternUri::new(service, uri))
    {
        let config = config_state.get_or_global(service);
        checker::check(service, *document, config)
    } else {
        vec![]
    }
}
