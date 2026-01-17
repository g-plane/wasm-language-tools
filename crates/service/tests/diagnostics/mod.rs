use lspt::{
    ClientCapabilities, DocumentDiagnosticParams, InitializeParams, TextDocumentIdentifier,
    WorkspaceClientCapabilities,
};
use std::thread;
use wat_service::{LanguageService, LintLevel, Lints, ServiceConfig};

mod block_type;
mod br_table_branches;
mod catch_type;
mod const_expr;
mod deprecated;
mod dup_names;
mod elem_type;
mod immediates;
mod implicit_module;
mod import_occur;
mod import_with_def;
mod mem_type;
mod multi_modules;
mod multi_starts;
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
mod unread;
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

fn calm(service: &mut LanguageService, uri: &str) {
    service.set_config(
        uri,
        Some(ServiceConfig {
            lint: Lints {
                unused: LintLevel::Allow,
                unread: LintLevel::Allow,
                unreachable: LintLevel::Allow,
                needless_mut: LintLevel::Allow,
                needless_try_table: LintLevel::Allow,
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
    service.commit(&uri, source.into());
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
    service.commit(&uri, source.into());
    service.set_config(&uri, None);
    let response = service.pull_diagnostics(create_params(uri));
    assert!(!response.items.is_empty());
}

#[test]
fn concurrent_pull() {
    let uri = "untitled:test".to_string();
    let mut service = LanguageService::default();
    service.commit(&uri, "(module (func)".into());
    thread::spawn({
        let mut service = service.clone();
        let uri = uri.clone();
        move || {
            service.commit(&uri, "(module (func))".into());
        }
    });
    service.pull_diagnostics(create_params(uri));
}

#[test]
fn concurrent_publish() {
    let uri = "untitled:test".to_string();
    let mut service = LanguageService::default();
    service.commit(&uri, "(module (func)".into());
    thread::spawn({
        let mut service = service.clone();
        let uri = uri.clone();
        move || {
            service.commit(&uri, "(module (func))".into());
        }
    });
    service.publish_diagnostics(uri);
}
