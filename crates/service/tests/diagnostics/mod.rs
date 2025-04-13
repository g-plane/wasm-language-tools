use lspt::{DocumentDiagnosticParams, TextDocumentIdentifier};
use wat_service::{LanguageService, LintLevel, Lints, ServiceConfig};

mod block_type;
mod br_table_branches;
mod dup_names;
mod global_expr;
mod immediates;
mod implicit_module;
mod import_occur;
mod multi_memories;
mod multi_modules;
mod mutated_immutable;
mod needless_mut;
mod new_non_defaultable;
mod packing;
mod shadow;
mod start;
mod subtyping;
mod syntax;
mod type_misuse;
mod typeck;
mod undef;
mod uninit;
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
                needless_mut: LintLevel::Allow,
                ..Default::default()
            },
            ..Default::default()
        },
    );
}
