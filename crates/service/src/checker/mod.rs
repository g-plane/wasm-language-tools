use crate::{binder::SymbolTablesCtx, syntax_tree::SyntaxTreeCtx, uri::InternUri, LanguageService};
use lspt::Diagnostic;
use wat_syntax::{SyntaxKind, SyntaxNode};

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
mod packing;
mod shadow;
mod start;
mod subtyping;
mod syntax;
mod typeck;
mod undef;
mod uninit;
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
    multi_modules::check(
        &mut diagnostics,
        config.lint.multi_modules,
        &line_index,
        &root,
    );
    root.children().enumerate().for_each(|(module_id, module)| {
        implicit_module::check(
            &mut diagnostics,
            config.lint.implicit_module,
            &line_index,
            &module,
        );
        multi_memories::check(
            &mut diagnostics,
            config.lint.multi_memories,
            &line_index,
            &module,
        );
        module.descendants().for_each(|node| match node.kind() {
            SyntaxKind::MODULE_FIELD_FUNC => {
                typeck::check_func(
                    &mut diagnostics,
                    service,
                    uri,
                    &line_index,
                    &symbol_table,
                    module_id as u32,
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
                uninit::check(service, &mut diagnostics, &line_index, &symbol_table, &node);
            }
            SyntaxKind::MODULE_FIELD_GLOBAL => {
                typeck::check_global(
                    &mut diagnostics,
                    service,
                    uri,
                    &line_index,
                    &symbol_table,
                    module_id as u32,
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
                global_expr::check(&mut diagnostics, &line_index, &node);
            }
            SyntaxKind::MODULE_FIELD_IMPORT => {
                import_occur::check(&mut diagnostics, &line_index, &node);
            }
            SyntaxKind::PLAIN_INSTR => {
                unknown_instr::check(&mut diagnostics, &line_index, &node);
                immediates::check(&mut diagnostics, &line_index, &node);
                br_table_branches::check(
                    &mut diagnostics,
                    service,
                    uri,
                    &line_index,
                    &symbol_table,
                    &node,
                );
                packing::check(
                    service,
                    &mut diagnostics,
                    uri,
                    &line_index,
                    &symbol_table,
                    &node,
                );
            }
            SyntaxKind::BLOCK_TYPE => {
                block_type::check(
                    &mut diagnostics,
                    service,
                    uri,
                    &line_index,
                    &symbol_table,
                    &node,
                );
            }
            SyntaxKind::MODULE_FIELD_START => {
                start::check(
                    &mut diagnostics,
                    service,
                    uri,
                    &line_index,
                    &symbol_table,
                    &node,
                );
            }
            _ => {}
        });
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
    mutated_immutable::check(service, &mut diagnostics, uri, &line_index, &symbol_table);
    needless_mut::check(
        service,
        &mut diagnostics,
        config.lint.needless_mut,
        uri,
        &line_index,
        &symbol_table,
    );
    subtyping::check(
        &mut diagnostics,
        service,
        uri,
        &line_index,
        &root,
        &symbol_table,
    );

    diagnostics
}
