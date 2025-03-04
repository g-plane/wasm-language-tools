use lspt::{DocumentDiagnosticParams, TextDocumentIdentifier};
use wat_service::{LanguageService, LintLevel, Lints, ServiceConfig};

mod br_table_branches;
mod dup_names;
mod global_mut;
mod immediates;
mod implicit_module;
mod import_occur;
mod multi_modules;
mod needless_mut;
mod shadow;
mod start;
mod subtyping;
mod syntax;
mod typeck;
mod undef;
mod unknown_instr;
mod unreachable;
mod unused;

fn create_params(uri: String) -> DocumentDiagnosticParams {
    DocumentDiagnosticParams {
        text_document: TextDocumentIdentifier { uri },
        identifier: Some("wat".into()),
        previous_result_id: None,
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    }
}

fn calm(service: &mut LanguageService, uri: String) {
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
