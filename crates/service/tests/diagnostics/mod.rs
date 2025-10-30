use lspt::{
    ClientCapabilities, DocumentDiagnosticParams, InitializeParams, TextDocumentIdentifier,
    WorkspaceClientCapabilities,
};
use wat_service::{LanguageService, LintLevel, Lints, ServiceConfig};

mod block_type;
mod br_table_branches;
mod const_expr;
mod dup_names;
mod elem_type;
mod immediates;
mod implicit_module;
mod import_occur;
mod mem_type;
mod multi_memories;
mod multi_modules;
mod mutated_immutable;
mod needless_mut;
mod needless_try_table;
mod new_non_defaultable;
mod packing;
mod shadow;
mod start;
mod subtyping;
mod syntax;
mod tag_type;
mod type_misuse;
mod typeck;
mod undef;
mod uninit;
mod unknown_instr;
mod unreachable;
mod unused;
mod useless_catch;

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
        Some(ServiceConfig {
            lint: Lints {
                unused: LintLevel::Allow,
                unreachable: LintLevel::Allow,
                needless_mut: LintLevel::Allow,
                useless_catch: LintLevel::Allow,
                ..Default::default()
            },
            ..Default::default()
        }),
    );
}

#[test]
fn uninit_config() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func))
";
    let mut service = LanguageService::default();
    service.initialize(InitializeParams {
        capabilities: ClientCapabilities {
            workspace: Some(WorkspaceClientCapabilities {
                configuration: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    });
    service.commit(uri.clone(), source.into());
    let response = service.pull_diagnostics(create_params(uri));
    assert!(response.items.is_empty());
}

#[test]
fn inherit_config() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func))
";
    let mut service = LanguageService::default();
    service.initialize(InitializeParams {
        capabilities: ClientCapabilities {
            workspace: Some(WorkspaceClientCapabilities {
                configuration: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    });
    service.commit(uri.clone(), source.into());
    service.set_config(uri.clone(), None);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(!response.items.is_empty());
}
