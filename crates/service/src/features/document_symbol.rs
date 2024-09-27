use crate::{binder::SymbolTablesCtx, files::FileInputCtx, helpers, LanguageServiceCtx};
use lsp_types::{DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse, SymbolKind};
use rowan::ast::AstNode;

#[allow(deprecated)]
pub fn document_symbol(
    service: &LanguageServiceCtx,
    params: DocumentSymbolParams,
) -> Option<DocumentSymbolResponse> {
    let uri = params.text_document.uri;
    let Some(module) = service.root(uri.clone()).modules().next() else {
        return None;
    };

    let line_index = service.line_index(uri.clone());
    let root = service.root(uri.clone());
    let root = root.syntax();
    let symbol_table = service.symbol_table(uri);

    let functions = symbol_table.functions.iter().map(|func| {
        let range =
            helpers::rowan_range_to_lsp_range(&line_index, func.ptr.syntax_node_ptr().text_range());
        DocumentSymbol {
            name: func
                .idx
                .name
                .clone()
                .unwrap_or_else(|| func.idx.num.to_string()),
            detail: None,
            kind: SymbolKind::FUNCTION,
            tags: None,
            deprecated: None,
            range: range.clone(),
            selection_range: func
                .ptr
                .to_node(&root)
                .ident_token()
                .map(|token| helpers::rowan_range_to_lsp_range(&line_index, token.text_range()))
                .unwrap_or(range),
            children: None,
        }
    });

    let module_range = helpers::rowan_range_to_lsp_range(&line_index, module.syntax().text_range());
    Some(DocumentSymbolResponse::Nested(vec![DocumentSymbol {
        name: "module".into(),
        detail: None,
        kind: SymbolKind::MODULE,
        tags: None,
        deprecated: None,
        range: module_range.clone(),
        selection_range: module_range,
        children: Some(functions.collect()),
    }]))
}
