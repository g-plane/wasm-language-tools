use crate::{LanguageService, binder::SymbolTable, config::ConfigState, document::Document};
use lspt::Diagnostic;
use wat_syntax::SyntaxKind;

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
mod mem_type;
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
mod unread;
mod unused;
mod useless_catch;

pub fn check(service: &LanguageService, document: Document) -> Vec<Diagnostic> {
    // Some clients like VS Code support pulling configuration per document.
    // In that case, we won't use global configuration,
    // but document-specific configuration may not be available if client doesn't send it yet.
    // If it isn't ready, we will skip the checker to avoid diagnostics flickering.
    let config_state = service.configs.get(&document.uri(service));
    let config = match config_state.as_deref() {
        Some(ConfigState::Inherit) => &service.global_config,
        Some(ConfigState::Override(config)) => config,
        None => return Vec::new(),
    };

    let uri = document.uri(service);
    let line_index = document.line_index(service);
    let root = document.root_tree(service);
    let symbol_table = SymbolTable::of(service, document);

    let mut diagnostics = Vec::with_capacity(4);
    syntax::check(service, &mut diagnostics, document, line_index);
    multi_modules::check(
        &mut diagnostics,
        config.lint.multi_modules,
        line_index,
        &root,
    );
    root.children().enumerate().for_each(|(module_id, module)| {
        let module_id = module_id as u32;
        if let Some(diagnostic) =
            implicit_module::check(config.lint.implicit_module, line_index, &module)
        {
            diagnostics.push(diagnostic);
        }
        module.descendants().for_each(|node| match node.kind() {
            SyntaxKind::MODULE_FIELD_FUNC => {
                typeck::check_func(
                    &mut diagnostics,
                    service,
                    document,
                    line_index,
                    symbol_table,
                    module_id,
                    &node,
                );
                unreachable::check(
                    &mut diagnostics,
                    service,
                    document,
                    config.lint.unreachable,
                    line_index,
                    &root,
                    &node,
                );
                uninit::check(
                    &mut diagnostics,
                    service,
                    document,
                    line_index,
                    &root,
                    symbol_table,
                    &node,
                );
                unread::check(
                    &mut diagnostics,
                    service,
                    document,
                    config.lint.unread,
                    line_index,
                    &root,
                    symbol_table,
                    &node,
                );
            }
            SyntaxKind::MODULE_FIELD_GLOBAL => {
                typeck::check_global(
                    &mut diagnostics,
                    service,
                    document,
                    line_index,
                    symbol_table,
                    module_id,
                    &node,
                );
                unreachable::check(
                    &mut diagnostics,
                    service,
                    document,
                    config.lint.unreachable,
                    line_index,
                    &root,
                    &node,
                );
                if let Some(diagnostic) = const_expr::check(line_index, &node) {
                    diagnostics.push(diagnostic);
                }
            }
            SyntaxKind::MODULE_FIELD_IMPORT => {
                if let Some(diagnostic) = import_occur::check(line_index, &node) {
                    diagnostics.push(diagnostic);
                }
            }
            SyntaxKind::PLAIN_INSTR => {
                if let Some(diagnostic) = unknown_instr::check(line_index, &node) {
                    diagnostics.push(diagnostic);
                }
                immediates::check(&mut diagnostics, line_index, &node);
                br_table_branches::check(
                    &mut diagnostics,
                    service,
                    document,
                    line_index,
                    symbol_table,
                    &node,
                );
                if let Some(diagnostic) =
                    packing::check(service, uri, document, line_index, symbol_table, &node)
                {
                    diagnostics.push(diagnostic);
                }
                type_misuse::check(
                    service,
                    &mut diagnostics,
                    document,
                    line_index,
                    symbol_table,
                    module_id,
                    &node,
                );
                if let Some(diagnostic) =
                    new_non_defaultable::check(service, document, line_index, symbol_table, &node)
                {
                    diagnostics.push(diagnostic);
                }
            }
            SyntaxKind::BLOCK_TYPE => {
                if let Some(diagnostic) =
                    block_type::check(service, document, line_index, symbol_table, &node)
                {
                    diagnostics.push(diagnostic);
                }
            }
            SyntaxKind::MODULE_FIELD_START => {
                if let Some(diagnostic) =
                    start::check(service, document, line_index, symbol_table, &node)
                {
                    diagnostics.push(diagnostic);
                }
            }
            SyntaxKind::MODULE_FIELD_TABLE => {
                typeck::check_table(
                    &mut diagnostics,
                    service,
                    document,
                    line_index,
                    symbol_table,
                    module_id,
                    &node,
                );
                if let Some(diagnostic) = const_expr::check(line_index, &node) {
                    diagnostics.push(diagnostic);
                }
            }
            SyntaxKind::MODULE_FIELD_ELEM => {
                if let Some(diagnostic) = elem_type::check(
                    service,
                    document,
                    line_index,
                    &root,
                    symbol_table,
                    module_id,
                    &node,
                ) {
                    diagnostics.push(diagnostic);
                }
            }
            SyntaxKind::MEMORY_TYPE => {
                mem_type::check(&mut diagnostics, line_index, &node);
            }
            SyntaxKind::OFFSET => {
                typeck::check_offset(
                    &mut diagnostics,
                    service,
                    document,
                    line_index,
                    symbol_table,
                    module_id,
                    &node,
                );
                if let Some(diagnostic) = const_expr::check(line_index, &node) {
                    diagnostics.push(diagnostic);
                }
            }
            SyntaxKind::ELEM_LIST => {
                typeck::check_elem_list(
                    &mut diagnostics,
                    service,
                    document,
                    line_index,
                    symbol_table,
                    module_id,
                    &node,
                );
            }
            SyntaxKind::ELEM_EXPR => {
                if let Some(diagnostic) = const_expr::check(line_index, &node) {
                    diagnostics.push(diagnostic);
                }
            }
            SyntaxKind::MODULE_FIELD_TAG | SyntaxKind::EXTERN_TYPE_TAG => {
                tag_type::check(
                    &mut diagnostics,
                    service,
                    document,
                    line_index,
                    symbol_table,
                    &node,
                );
            }
            SyntaxKind::BLOCK_TRY_TABLE => {
                if let Some(diagnostic) =
                    needless_try_table::check(config.lint.needless_try_table, line_index, &node)
                {
                    diagnostics.push(diagnostic);
                }
                useless_catch::check(
                    service,
                    &mut diagnostics,
                    config.lint.useless_catch,
                    uri,
                    line_index,
                    symbol_table,
                    &node,
                );
            }
            SyntaxKind::CATCH | SyntaxKind::CATCH_ALL => {
                if let Some(diagnostic) = catch_type::check(
                    service,
                    document,
                    line_index,
                    &root,
                    symbol_table,
                    module_id,
                    node,
                ) {
                    diagnostics.push(diagnostic);
                }
            }
            _ => {}
        });
    });
    undef::check(service, &mut diagnostics, line_index, symbol_table);
    dup_names::check(
        service,
        &mut diagnostics,
        uri,
        document,
        line_index,
        &root,
        symbol_table,
    );
    unused::check(
        service,
        &mut diagnostics,
        config.lint.unused,
        line_index,
        &root,
        symbol_table,
    );
    shadow::check(
        service,
        &mut diagnostics,
        config.lint.shadow,
        uri,
        line_index,
        &root,
        symbol_table,
    );
    mutated_immutable::check(
        service,
        &mut diagnostics,
        uri,
        document,
        line_index,
        symbol_table,
    );
    needless_mut::check(
        service,
        &mut diagnostics,
        config.lint.needless_mut,
        document,
        line_index,
        symbol_table,
    );
    subtyping::check(
        &mut diagnostics,
        service,
        document,
        line_index,
        &root,
        symbol_table,
    );
    deprecated::check(
        &mut diagnostics,
        service,
        document,
        config.lint.deprecated,
        line_index,
        symbol_table,
    );

    diagnostics
}
