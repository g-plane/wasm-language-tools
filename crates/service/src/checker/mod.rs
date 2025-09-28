use crate::{LanguageService, binder::SymbolTable, document::Document};
use lspt::Diagnostic;
use wat_syntax::SyntaxKind;

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

pub fn check(service: &LanguageService, document: Document) -> Vec<Diagnostic> {
    let uri = document.uri(service);
    let line_index = document.line_index(service);
    let root = document.root_tree(service);
    let symbol_table = SymbolTable::of(service, document);
    let config = service.get_config(document);

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
        implicit_module::check(
            &mut diagnostics,
            config.lint.implicit_module,
            line_index,
            &module,
        );
        multi_memories::check(
            &mut diagnostics,
            config.lint.multi_memories,
            line_index,
            &module,
        );
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
                    config.lint.unreachable,
                    line_index,
                    &root,
                    symbol_table,
                    &node,
                );
                uninit::check(
                    service,
                    &mut diagnostics,
                    document,
                    line_index,
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
                    config.lint.unreachable,
                    line_index,
                    &root,
                    symbol_table,
                    &node,
                );
                const_expr::check(&mut diagnostics, line_index, &node);
            }
            SyntaxKind::MODULE_FIELD_IMPORT => {
                import_occur::check(&mut diagnostics, line_index, &node);
            }
            SyntaxKind::PLAIN_INSTR => {
                unknown_instr::check(&mut diagnostics, line_index, &node);
                immediates::check(&mut diagnostics, line_index, &node);
                br_table_branches::check(
                    &mut diagnostics,
                    service,
                    document,
                    line_index,
                    symbol_table,
                    &node,
                );
                packing::check(
                    service,
                    &mut diagnostics,
                    uri,
                    document,
                    line_index,
                    symbol_table,
                    &node,
                );
                type_misuse::check(
                    service,
                    &mut diagnostics,
                    document,
                    line_index,
                    symbol_table,
                    module_id,
                    &node,
                );
                new_non_defaultable::check(
                    service,
                    &mut diagnostics,
                    document,
                    line_index,
                    symbol_table,
                    &node,
                );
            }
            SyntaxKind::BLOCK_TYPE => {
                block_type::check(
                    &mut diagnostics,
                    service,
                    document,
                    line_index,
                    symbol_table,
                    &node,
                );
            }
            SyntaxKind::MODULE_FIELD_START => {
                start::check(
                    &mut diagnostics,
                    service,
                    document,
                    line_index,
                    symbol_table,
                    &node,
                );
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
                const_expr::check(&mut diagnostics, line_index, &node);
            }
            SyntaxKind::MODULE_FIELD_ELEM => {
                elem_type::check(
                    &mut diagnostics,
                    service,
                    document,
                    line_index,
                    &root,
                    symbol_table,
                    module_id,
                    &node,
                );
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
                const_expr::check(&mut diagnostics, line_index, &node);
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
                const_expr::check(&mut diagnostics, line_index, &node);
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

    diagnostics
}
