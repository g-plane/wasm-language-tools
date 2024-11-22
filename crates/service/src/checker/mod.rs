use crate::{binder::SymbolTablesCtx, files::FilesCtx, InternUri, LanguageService};
use lsp_types::Diagnostic;
use wat_syntax::{SyntaxKind, SyntaxNode};

mod dup_names;
mod literal_operands;
mod multi_modules;
mod typeck;
mod undef;

pub fn check(service: &LanguageService, uri: InternUri) -> Vec<Diagnostic> {
    let line_index = service.line_index(uri);
    let root = SyntaxNode::new_root(service.root(uri));
    let symbol_table = service.symbol_table(uri);

    let mut diagnostics = service.parser_result(uri).1;
    root.descendants().for_each(|node| match node.kind() {
        SyntaxKind::ROOT => {
            multi_modules::check(&mut diagnostics, &line_index, &node);
        }
        SyntaxKind::MODULE_FIELD_FUNC | SyntaxKind::MODULE_FIELD_GLOBAL => {
            typeck::check_stacked(
                &mut diagnostics,
                service,
                uri,
                &line_index,
                &node,
                &symbol_table,
            );
        }
        SyntaxKind::PLAIN_INSTR => {
            literal_operands::check(&mut diagnostics, &line_index, &node);
            typeck::check_folded(
                &mut diagnostics,
                service,
                uri,
                &line_index,
                node,
                &symbol_table,
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

    diagnostics
}
