use crate::{binder::SymbolTablesCtx, files::FilesCtx, helpers, LanguageService};
use lsp_types::{DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse, SymbolKind};
use rowan::ast::AstNode;

impl LanguageService {
    pub fn document_symbol(&self, params: DocumentSymbolParams) -> Option<DocumentSymbolResponse> {
        let uri = params.text_document.uri;
        let line_index = self.ctx.line_index(uri.clone());
        let root = self.ctx.root(uri.clone());
        let root = root.syntax();
        let symbol_table = self.ctx.symbol_table(uri);

        #[allow(deprecated)]
        let modules = symbol_table
            .modules
            .iter()
            .map(|module| {
                let functions = module.functions.iter().map(|func| {
                    let range = helpers::rowan_range_to_lsp_range(
                        &line_index,
                        func.ptr.syntax_node_ptr().text_range(),
                    );
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
                        range,
                        selection_range: func
                            .ptr
                            .to_node(root)
                            .ident_token()
                            .map(|token| {
                                helpers::rowan_range_to_lsp_range(&line_index, token.text_range())
                            })
                            .unwrap_or(range),
                        children: None,
                    }
                });

                let module_range = helpers::rowan_range_to_lsp_range(
                    &line_index,
                    module.ptr.syntax_node_ptr().text_range(),
                );
                DocumentSymbol {
                    name: "module".into(),
                    detail: None,
                    kind: SymbolKind::MODULE,
                    tags: None,
                    deprecated: None,
                    range: module_range,
                    selection_range: module_range,
                    children: Some(functions.collect()),
                }
            })
            .collect();
        Some(DocumentSymbolResponse::Nested(modules))
    }
}
