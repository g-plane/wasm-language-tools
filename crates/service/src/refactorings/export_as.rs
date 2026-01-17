use crate::{
    LanguageService, binder::SymbolKey, document::Document, exports, helpers, uri::InternUri,
};
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rowan::{TextRange, ast::support};
use rustc_hash::{FxBuildHasher, FxHashMap};
use wat_syntax::{SyntaxKind, SyntaxNode, SyntaxNodePtr};

pub fn act(
    service: &LanguageService,
    uri: InternUri,
    document: Document,
    line_index: &LineIndex,
    node: &SyntaxNode,
) -> Option<CodeAction> {
    let def_key = SymbolKey::new(node);
    let ident_token = support::token(node, SyntaxKind::IDENT)?;
    let module = node.parent()?;
    let exports = exports::get_exports(service, document);
    if exports
        .get(&SyntaxNodePtr::new(&module))
        .is_some_and(|exports| exports.iter().any(|export| export.def_key == def_key))
    {
        return None;
    }

    let ident = ident_token.text().strip_prefix('$')?;
    let name = if ident.starts_with('"') {
        ident.into()
    } else {
        format!("\"{ident}\"")
    };
    let mut changes = FxHashMap::with_capacity_and_hasher(1, FxBuildHasher);
    changes.insert(
        uri.raw(service),
        vec![TextEdit {
            range: helpers::rowan_range_to_lsp_range(
                line_index,
                TextRange::empty(ident_token.text_range().end()),
            ),
            new_text: format!(" (export {name})"),
        }],
    );
    Some(CodeAction {
        title: format!("Export as {name}"),
        kind: Some(CodeActionKind::Refactor),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        ..Default::default()
    })
}
