use crate::{binder::SymbolTablesCtx, files::FilesCtx, InternUri, LanguageService};
use lsp_types::Diagnostic;
use wat_syntax::{SyntaxKind, SyntaxNode};

mod multi_modules;
mod undef;

pub fn check_file(service: &LanguageService, uri: InternUri) -> Vec<Diagnostic> {
    let line_index = service.line_index(uri);
    let root = SyntaxNode::new_root(service.root(uri));
    let symbol_table = service.symbol_table(uri);

    let mut diagnostics = service.parser_result(uri).1;
    root.descendants().for_each(|node| match node.kind() {
        SyntaxKind::ROOT => {
            multi_modules::check(&mut diagnostics, &line_index, &node);
        }
        _ => {}
    });
    undef::check(service, &mut diagnostics, &line_index, &symbol_table);

    diagnostics
}
