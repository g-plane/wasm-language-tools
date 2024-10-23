use crate::{files::FilesCtx, InternUri, LanguageServiceCtx};
use lsp_types::Diagnostic;
use wat_syntax::{SyntaxKind, SyntaxNode};

mod multi_modules;

pub fn check_file(ctx: &LanguageServiceCtx, uri: InternUri) -> Vec<Diagnostic> {
    let line_index = ctx.line_index(uri);
    let root = SyntaxNode::new_root(ctx.root(uri));

    let mut diagnostics = ctx.parser_result(uri).1;
    root.descendants().for_each(|node| match node.kind() {
        SyntaxKind::ROOT => {
            multi_modules::check(&mut diagnostics, &line_index, &node);
        }
        _ => {}
    });

    diagnostics
}
