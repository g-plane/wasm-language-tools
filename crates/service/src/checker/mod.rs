use crate::{binder::SymbolTablesCtx, files::FilesCtx, InternUri, LanguageService};
use lsp_types::Diagnostic;
use wat_syntax::{SyntaxKind, SyntaxNode};

mod dup_names;
mod global_mut;
mod immediates;
mod implicit_module;
mod multi_modules;
mod shadow;
mod syntax;
mod typeck;
mod undef;
mod unknown_instr;
mod unreachable;
mod unused;

pub fn check(service: &LanguageService, uri: InternUri) -> Vec<Diagnostic> {
    let line_index = service.line_index(uri);
    let root = SyntaxNode::new_root(service.root(uri));
    let symbol_table = service.symbol_table(uri);
    let config = &service.get_config(uri);

    let mut diagnostics = Vec::with_capacity(4);
    syntax::check(service, &mut diagnostics, uri, &line_index, &root);
    root.descendants().for_each(|node| match node.kind() {
        SyntaxKind::ROOT => {
            multi_modules::check(&mut diagnostics, &line_index, &node);
        }
        SyntaxKind::MODULE => {
            implicit_module::check(
                &mut diagnostics,
                config.lint.implicit_module,
                &line_index,
                &node,
            );
        }
        SyntaxKind::MODULE_FIELD_FUNC => {
            typeck::check_func(
                &mut diagnostics,
                service,
                uri,
                &line_index,
                &symbol_table,
                &node,
            );
            unreachable::check(
                &mut diagnostics,
                config.lint.unreachable,
                &line_index,
                &root,
                &symbol_table,
                &node,
            );
        }
        SyntaxKind::MODULE_FIELD_GLOBAL => {
            typeck::check_global(
                &mut diagnostics,
                service,
                uri,
                &line_index,
                &symbol_table,
                &node,
            );
        }
        SyntaxKind::PLAIN_INSTR => {
            unknown_instr::check(&mut diagnostics, &line_index, &node);
            immediates::check(&mut diagnostics, &line_index, &node);
            global_mut::check(
                service,
                &mut diagnostics,
                uri,
                &line_index,
                &root,
                &symbol_table,
                &node,
            );
        }
        _ => {}
    });
    undef::check(service, &mut diagnostics, &line_index, &symbol_table);
    dup_names::check(
        service,
        &mut diagnostics,
        uri,
        &line_index,
        &root,
        &symbol_table,
    );
    unused::check(
        service,
        &mut diagnostics,
        config.lint.unused,
        &line_index,
        &root,
        &symbol_table,
    );
    shadow::check(
        service,
        &mut diagnostics,
        config.lint.shadow,
        uri,
        &line_index,
        &root,
        &symbol_table,
    );

    diagnostics
}
