use crate::{
    binder::{SymbolKind, SymbolTablesCtx},
    helpers,
    syntax_tree::SyntaxTreeCtx,
    uri::UrisCtx,
    LanguageService,
};
use lsp_types::{
    DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse, SymbolKind as LspSymbolKind,
};
use rowan::ast::support::token;
use rustc_hash::FxHashMap;
use wat_syntax::{SyntaxKind, SyntaxNode};

impl LanguageService {
    /// Handler for `textDocument/documentSymbol` request.
    pub fn document_symbol(&self, params: DocumentSymbolParams) -> Option<DocumentSymbolResponse> {
        let uri = self.uri(params.text_document.uri);
        let line_index = self.line_index(uri);
        let root = SyntaxNode::new_root(self.root(uri));
        let symbol_table = self.symbol_table(uri);

        #[expect(deprecated)]
        let mut symbols_map = symbol_table
            .symbols
            .iter()
            .filter_map(|symbol| match &symbol.kind {
                SymbolKind::Module => {
                    let module_range =
                        helpers::rowan_range_to_lsp_range(&line_index, symbol.key.text_range());
                    Some((
                        symbol.key,
                        DocumentSymbol {
                            name: "module".into(),
                            detail: None,
                            kind: LspSymbolKind::MODULE,
                            tags: None,
                            deprecated: None,
                            range: module_range,
                            selection_range: module_range,
                            children: None,
                        },
                    ))
                }
                SymbolKind::Func => {
                    let range =
                        helpers::rowan_range_to_lsp_range(&line_index, symbol.key.text_range());
                    Some((
                        symbol.key,
                        DocumentSymbol {
                            name: symbol.idx.render(self).to_string(),
                            detail: None,
                            kind: LspSymbolKind::FUNCTION,
                            tags: None,
                            deprecated: None,
                            range,
                            selection_range: token(&symbol.key.to_node(&root), SyntaxKind::IDENT)
                                .map(|token| {
                                    helpers::rowan_range_to_lsp_range(
                                        &line_index,
                                        token.text_range(),
                                    )
                                })
                                .unwrap_or(range),
                            children: None,
                        },
                    ))
                }
                SymbolKind::Local => {
                    let range =
                        helpers::rowan_range_to_lsp_range(&line_index, symbol.key.text_range());
                    Some((
                        symbol.key,
                        DocumentSymbol {
                            name: symbol.idx.render(self).to_string(),
                            detail: None,
                            kind: LspSymbolKind::VARIABLE,
                            tags: None,
                            deprecated: None,
                            range,
                            selection_range: token(&symbol.key.to_node(&root), SyntaxKind::IDENT)
                                .map(|token| {
                                    helpers::rowan_range_to_lsp_range(
                                        &line_index,
                                        token.text_range(),
                                    )
                                })
                                .unwrap_or(range),
                            children: None,
                        },
                    ))
                }
                SymbolKind::Type
                | SymbolKind::GlobalDef
                | SymbolKind::MemoryDef
                | SymbolKind::TableDef => {
                    let range =
                        helpers::rowan_range_to_lsp_range(&line_index, symbol.key.text_range());
                    Some((
                        symbol.key,
                        DocumentSymbol {
                            name: symbol.idx.render(self).to_string(),
                            detail: None,
                            kind: LspSymbolKind::VARIABLE,
                            tags: None,
                            deprecated: None,
                            range,
                            selection_range: token(&symbol.key.to_node(&root), SyntaxKind::IDENT)
                                .map(|token| {
                                    helpers::rowan_range_to_lsp_range(
                                        &line_index,
                                        token.text_range(),
                                    )
                                })
                                .unwrap_or(range),
                            children: None,
                        },
                    ))
                }
                SymbolKind::Param
                | SymbolKind::Call
                | SymbolKind::LocalRef
                | SymbolKind::TypeUse
                | SymbolKind::GlobalRef
                | SymbolKind::MemoryRef
                | SymbolKind::TableRef
                | SymbolKind::BlockDef
                | SymbolKind::BlockRef => None,
            })
            .collect::<FxHashMap<_, _>>();
        symbol_table
            .symbols
            .iter()
            .filter(|symbol| symbol.region.kind() != SyntaxKind::ROOT)
            .rev()
            .for_each(|symbol| {
                if let Some((mut lsp_symbol, parent)) = symbols_map
                    .remove(&symbol.key)
                    .zip(symbols_map.get_mut(&symbol.region))
                {
                    if let Some(children) = &mut lsp_symbol.children {
                        children.sort_by_key(|symbol| symbol.range.start);
                    }
                    parent
                        .children
                        .get_or_insert_with(|| Vec::with_capacity(1))
                        .push(lsp_symbol);
                }
            });
        let mut lsp_symbols = symbols_map
            .into_values()
            .filter_map(|mut lsp_symbol| {
                if let Some(children) = &mut lsp_symbol.children {
                    children.sort_by_key(|symbol| symbol.range.start);
                    Some(lsp_symbol)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        lsp_symbols.sort_by_key(|symbol| symbol.range.start);
        Some(DocumentSymbolResponse::Nested(lsp_symbols))
    }
}
