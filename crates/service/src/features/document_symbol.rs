use crate::{
    binder::{Symbol, SymbolKind, SymbolTablesCtx},
    helpers,
    idx::IdentsCtx,
    syntax_tree::SyntaxTreeCtx,
    uri::UrisCtx,
    LanguageService,
};
use lspt::{DocumentSymbol, DocumentSymbolParams, SymbolKind as LspSymbolKind};
use rustc_hash::FxHashMap;
use wat_syntax::{SyntaxKind, SyntaxNode};

impl LanguageService {
    /// Handler for `textDocument/documentSymbol` request.
    pub fn document_symbol(&self, params: DocumentSymbolParams) -> Option<Vec<DocumentSymbol>> {
        let uri = self.uri(params.text_document.uri);
        let line_index = self.line_index(uri);
        let root = SyntaxNode::new_root(self.root(uri));
        let symbol_table = self.symbol_table(uri);

        #[expect(deprecated)]
        let mut symbols_map = symbol_table
            .symbols
            .iter()
            .filter_map(|symbol| match symbol.kind {
                SymbolKind::Module => {
                    let module_range =
                        helpers::rowan_range_to_lsp_range(&line_index, symbol.key.text_range());
                    Some((
                        symbol.key,
                        DocumentSymbol {
                            name: "module".into(),
                            detail: None,
                            kind: LspSymbolKind::Module,
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
                            name: render_symbol_name(symbol, self),
                            detail: None,
                            kind: LspSymbolKind::Function,
                            tags: None,
                            deprecated: None,
                            range,
                            selection_range: helpers::create_selection_range(
                                symbol,
                                &root,
                                &line_index,
                            ),
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
                            name: render_symbol_name(symbol, self),
                            detail: None,
                            kind: LspSymbolKind::Variable,
                            tags: None,
                            deprecated: None,
                            range,
                            selection_range: helpers::create_selection_range(
                                symbol,
                                &root,
                                &line_index,
                            ),
                            children: None,
                        },
                    ))
                }
                SymbolKind::Type => {
                    let range =
                        helpers::rowan_range_to_lsp_range(&line_index, symbol.key.text_range());
                    Some((
                        symbol.key,
                        DocumentSymbol {
                            name: render_symbol_name(symbol, self),
                            detail: helpers::infer_type_def_symbol_detail(symbol, &root),
                            kind: LspSymbolKind::Class,
                            tags: None,
                            deprecated: None,
                            range,
                            selection_range: helpers::create_selection_range(
                                symbol,
                                &root,
                                &line_index,
                            ),
                            children: None,
                        },
                    ))
                }
                SymbolKind::GlobalDef | SymbolKind::MemoryDef | SymbolKind::TableDef => {
                    let range =
                        helpers::rowan_range_to_lsp_range(&line_index, symbol.key.text_range());
                    Some((
                        symbol.key,
                        DocumentSymbol {
                            name: render_symbol_name(symbol, self),
                            detail: None,
                            kind: LspSymbolKind::Variable,
                            tags: None,
                            deprecated: None,
                            range,
                            selection_range: helpers::create_selection_range(
                                symbol,
                                &root,
                                &line_index,
                            ),
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
        Some(lsp_symbols)
    }
}

fn render_symbol_name(symbol: &Symbol, service: &LanguageService) -> String {
    if let Some(name) = symbol.idx.name {
        service.lookup_ident(name).to_string()
    } else if let Some(num) = symbol.idx.num {
        let kind = match symbol.kind {
            SymbolKind::Func => "func",
            SymbolKind::Local => "local",
            SymbolKind::Type => "type",
            SymbolKind::GlobalDef => "global",
            SymbolKind::MemoryDef => "memory",
            SymbolKind::TableDef => "table",
            _ => unreachable!(),
        };
        format!("{kind} {num}")
    } else {
        String::new()
    }
}
