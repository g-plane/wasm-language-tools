use crate::{
    binder::{SymbolItemKind, SymbolTablesCtx},
    files::FilesCtx,
    helpers, LanguageService,
};
use lsp_types::{DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse, SymbolKind};
use rowan::ast::support::token;
use rustc_hash::FxHashMap;
use wat_syntax::SyntaxKind;

impl LanguageService {
    pub fn document_symbol(&self, params: DocumentSymbolParams) -> Option<DocumentSymbolResponse> {
        let uri = self.ctx.uri(params.text_document.uri);
        let line_index = self.ctx.line_index(uri);
        let root = self.ctx.root(uri);
        let symbol_table = self.ctx.symbol_table(uri);

        #[allow(deprecated)]
        let mut symbols_map = symbol_table
            .symbols
            .iter()
            .filter_map(|symbol| match &symbol.kind {
                SymbolItemKind::Module => {
                    let module_range =
                        helpers::rowan_range_to_lsp_range(&line_index, symbol.key.ptr.text_range());
                    Some((
                        symbol.key.clone(),
                        DocumentSymbol {
                            name: "module".into(),
                            detail: None,
                            kind: SymbolKind::MODULE,
                            tags: None,
                            deprecated: None,
                            range: module_range,
                            selection_range: module_range,
                            children: None,
                        },
                    ))
                }
                SymbolItemKind::Func(func) => {
                    let range =
                        helpers::rowan_range_to_lsp_range(&line_index, symbol.key.ptr.text_range());
                    Some((
                        symbol.key.clone(),
                        DocumentSymbol {
                            name: func.name.clone().unwrap_or_else(|| func.num.to_string()),
                            detail: None,
                            kind: SymbolKind::FUNCTION,
                            tags: None,
                            deprecated: None,
                            range,
                            selection_range: token(
                                &symbol.key.ptr.to_node(&root),
                                SyntaxKind::IDENT,
                            )
                            .map(|token| {
                                helpers::rowan_range_to_lsp_range(&line_index, token.text_range())
                            })
                            .unwrap_or(range),
                            children: None,
                        },
                    ))
                }
                SymbolItemKind::Local(local) => {
                    let range =
                        helpers::rowan_range_to_lsp_range(&line_index, symbol.key.ptr.text_range());
                    Some((
                        symbol.key.clone(),
                        DocumentSymbol {
                            name: local.name.clone().unwrap_or_else(|| local.num.to_string()),
                            detail: None,
                            kind: SymbolKind::VARIABLE,
                            tags: None,
                            deprecated: None,
                            range,
                            selection_range: token(
                                &symbol.key.ptr.to_node(&root),
                                SyntaxKind::IDENT,
                            )
                            .map(|token| {
                                helpers::rowan_range_to_lsp_range(&line_index, token.text_range())
                            })
                            .unwrap_or(range),
                            children: None,
                        },
                    ))
                }
                SymbolItemKind::Type(ty) => {
                    let range =
                        helpers::rowan_range_to_lsp_range(&line_index, symbol.key.ptr.text_range());
                    Some((
                        symbol.key.clone(),
                        DocumentSymbol {
                            name: ty.name.clone().unwrap_or_else(|| ty.num.to_string()),
                            detail: None,
                            kind: SymbolKind::VARIABLE,
                            tags: None,
                            deprecated: None,
                            range,
                            selection_range: token(
                                &symbol.key.ptr.to_node(&root),
                                SyntaxKind::IDENT,
                            )
                            .map(|token| {
                                helpers::rowan_range_to_lsp_range(&line_index, token.text_range())
                            })
                            .unwrap_or(range),
                            children: None,
                        },
                    ))
                }
                SymbolItemKind::Param(..)
                | SymbolItemKind::Call(..)
                | SymbolItemKind::LocalRef(..)
                | SymbolItemKind::TypeUse(..) => None,
            })
            .collect::<FxHashMap<_, _>>();
        symbol_table.symbols.iter().rev().for_each(|symbol| {
            if let Some((lsp_symbol, parent)) = symbol.parent.as_ref().and_then(|parent| {
                symbols_map
                    .remove(&symbol.key)
                    .zip(symbols_map.get_mut(parent))
            }) {
                parent
                    .children
                    .get_or_insert_with(|| Vec::with_capacity(1))
                    .push(lsp_symbol);
            }
        });
        Some(DocumentSymbolResponse::Nested(
            symbols_map
                .into_iter()
                .filter_map(|(_, v)| v.children.is_some().then_some(v))
                .collect(),
        ))
    }
}
