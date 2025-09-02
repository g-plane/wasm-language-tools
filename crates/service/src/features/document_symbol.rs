use crate::{
    LanguageService,
    binder::{Symbol, SymbolKind, SymbolTable},
    helpers, types_analyzer,
};
use lspt::{DocumentSymbol, DocumentSymbolParams, SymbolKind as LspSymbolKind};
use rowan::ast::AstNode;
use rustc_hash::FxHashMap;
use wat_syntax::{SyntaxKind, ast::ModuleFieldGlobal};

impl LanguageService {
    /// Handler for `textDocument/documentSymbol` request.
    pub fn document_symbol(&self, params: DocumentSymbolParams) -> Option<Vec<DocumentSymbol>> {
        let document = self.get_document(params.text_document.uri)?;
        let line_index = document.line_index(self);
        let root = document.root_tree(self);
        let symbol_table = SymbolTable::of(self, document);

        #[expect(deprecated)]
        let mut symbols_map = symbol_table
            .symbols
            .iter()
            .filter_map(|symbol| match symbol.kind {
                SymbolKind::Module => {
                    let module_range =
                        helpers::rowan_range_to_lsp_range(line_index, symbol.key.text_range());
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
                        helpers::rowan_range_to_lsp_range(line_index, symbol.key.text_range());
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
                                symbol, &root, line_index,
                            ),
                            children: None,
                        },
                    ))
                }
                SymbolKind::Local => {
                    let range =
                        helpers::rowan_range_to_lsp_range(line_index, symbol.key.text_range());
                    Some((
                        symbol.key,
                        DocumentSymbol {
                            name: render_symbol_name(symbol, self),
                            detail: types_analyzer::extract_type(self, &symbol.green)
                                .map(|ty| ty.render(self).to_string()),
                            kind: LspSymbolKind::Variable,
                            tags: None,
                            deprecated: None,
                            range,
                            selection_range: helpers::create_selection_range(
                                symbol, &root, line_index,
                            ),
                            children: None,
                        },
                    ))
                }
                SymbolKind::Type => {
                    let range =
                        helpers::rowan_range_to_lsp_range(line_index, symbol.key.text_range());
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
                                symbol, &root, line_index,
                            ),
                            children: None,
                        },
                    ))
                }
                SymbolKind::GlobalDef => {
                    let range =
                        helpers::rowan_range_to_lsp_range(line_index, symbol.key.text_range());
                    Some((
                        symbol.key,
                        DocumentSymbol {
                            name: render_symbol_name(symbol, self),
                            detail: types_analyzer::extract_global_type(self, &symbol.green)
                                .map(|ty| {
                                    if ModuleFieldGlobal::cast(symbol.key.to_node(&root))
                                        .and_then(|global| global.global_type())
                                        .and_then(|global_type| global_type.mut_keyword())
                                        .is_some()
                                    {
                                        format!("(mut {})", ty.render(self))
                                    } else {
                                        ty.render(self).to_string()
                                    }
                                }),
                            kind: LspSymbolKind::Variable,
                            tags: None,
                            deprecated: None,
                            range,
                            selection_range: helpers::create_selection_range(
                                symbol, &root, line_index,
                            ),
                            children: None,
                        },
                    ))
                }
                SymbolKind::MemoryDef | SymbolKind::TableDef => {
                    let range =
                        helpers::rowan_range_to_lsp_range(line_index, symbol.key.text_range());
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
                                symbol, &root, line_index,
                            ),
                            children: None,
                        },
                    ))
                }
                SymbolKind::FieldDef => {
                    let range =
                        helpers::rowan_range_to_lsp_range(line_index, symbol.key.text_range());
                    Some((
                        symbol.key,
                        DocumentSymbol {
                            name: render_symbol_name(symbol, self),
                            detail: types_analyzer::resolve_field_type(
                                self,
                                document,
                                symbol.key,
                                symbol.region,
                            )
                            .map(|ty| ty.render(self).to_string()),
                            kind: LspSymbolKind::Field,
                            tags: None,
                            deprecated: None,
                            range,
                            selection_range: helpers::create_selection_range(
                                symbol, &root, line_index,
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
                | SymbolKind::BlockRef
                | SymbolKind::FieldRef => None,
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
        name.ident(service).to_string()
    } else if let Some(num) = symbol.idx.num {
        let kind = match symbol.kind {
            SymbolKind::Func => "func",
            SymbolKind::Local => "local",
            SymbolKind::Type => "type",
            SymbolKind::GlobalDef => "global",
            SymbolKind::MemoryDef => "memory",
            SymbolKind::TableDef => "table",
            SymbolKind::FieldDef => "field",
            _ => unreachable!(),
        };
        format!("{kind} {num}")
    } else {
        String::new()
    }
}
