use crate::{
    LanguageService,
    binder::{Symbol, SymbolKind, SymbolTable},
    deprecation,
    helpers::LineIndexExt,
    mutability,
    types_analyzer::{self, CompositeType},
};
use lspt::{DocumentSymbol, DocumentSymbolParams, SymbolKind as LspSymbolKind, SymbolTag};
use rustc_hash::FxHashMap;
use wat_syntax::SyntaxKind;

impl LanguageService {
    /// Handler for `textDocument/documentSymbol` request.
    pub fn document_symbol(&self, params: DocumentSymbolParams) -> Option<Vec<DocumentSymbol>> {
        let document = self.get_document(params.text_document.uri)?;
        self.with_db(|db| {
            let line_index = document.line_index(db);
            let symbol_table = SymbolTable::of(db, document);
            let deprecation = deprecation::get_deprecation(db, document);

            let mut symbols_map = symbol_table
                .symbols
                .values()
                .filter_map(|symbol| {
                    let range = line_index.convert(symbol.key.text_range());
                    let selection_range = line_index.convert(*symbol_table.def_poi.get(&symbol.key)?);
                    let tags = if deprecation.contains_key(&symbol.key) {
                        Some(vec![SymbolTag::Deprecated])
                    } else {
                        None
                    };
                    match symbol.kind {
                        SymbolKind::Module => Some((
                            symbol.key,
                            DocumentSymbol {
                                name: "module".into(),
                                detail: None,
                                kind: LspSymbolKind::Module,
                                tags,
                                range,
                                selection_range,
                                children: None,
                            },
                        )),
                        SymbolKind::Func => {
                            let range = line_index.convert(symbol.key.text_range());
                            Some((
                                symbol.key,
                                DocumentSymbol {
                                    name: render_symbol_name(symbol, db),
                                    detail: None,
                                    kind: LspSymbolKind::Function,
                                    tags,
                                    range,
                                    selection_range,
                                    children: None,
                                },
                            ))
                        }
                        SymbolKind::Local => {
                            let range = line_index.convert(symbol.key.text_range());
                            Some((
                                symbol.key,
                                DocumentSymbol {
                                    name: render_symbol_name(symbol, db),
                                    detail: types_analyzer::extract_type(db, &symbol.green)
                                        .map(|ty| ty.render(db).to_string()),
                                    kind: LspSymbolKind::Variable,
                                    tags,
                                    range,
                                    selection_range,
                                    children: None,
                                },
                            ))
                        }
                        SymbolKind::Type => {
                            let def_types = types_analyzer::get_def_types(db, document);
                            let range = line_index.convert(symbol.key.text_range());
                            Some((
                                symbol.key,
                                DocumentSymbol {
                                    name: render_symbol_name(symbol, db),
                                    detail: def_types.get(&symbol.key).map(|def_type| match def_type.comp {
                                        CompositeType::Func(..) => "func".into(),
                                        CompositeType::Struct(..) => "struct".into(),
                                        CompositeType::Array(..) => "array".into(),
                                    }),
                                    kind: LspSymbolKind::Class,
                                    tags,
                                    range,
                                    selection_range,
                                    children: None,
                                },
                            ))
                        }
                        SymbolKind::GlobalDef => {
                            let range = line_index.convert(symbol.key.text_range());
                            Some((
                                symbol.key,
                                DocumentSymbol {
                                    name: render_symbol_name(symbol, db),
                                    detail: types_analyzer::extract_global_type(db, &symbol.green).map(|ty| {
                                        if mutability::get_mutabilities(db, document)
                                            .get(&symbol.key)
                                            .and_then(|mutability| mutability.mut_keyword)
                                            .is_some()
                                        {
                                            format!("(mut {})", ty.render(db))
                                        } else {
                                            ty.render(db).to_string()
                                        }
                                    }),
                                    kind: LspSymbolKind::Variable,
                                    tags,
                                    range,
                                    selection_range,
                                    children: None,
                                },
                            ))
                        }
                        SymbolKind::MemoryDef
                        | SymbolKind::TableDef
                        | SymbolKind::TagDef
                        | SymbolKind::DataDef
                        | SymbolKind::ElemDef => {
                            let range = line_index.convert(symbol.key.text_range());
                            Some((
                                symbol.key,
                                DocumentSymbol {
                                    name: render_symbol_name(symbol, db),
                                    detail: None,
                                    kind: LspSymbolKind::Variable,
                                    tags,
                                    range,
                                    selection_range,
                                    children: None,
                                },
                            ))
                        }
                        SymbolKind::FieldDef => {
                            let range = line_index.convert(symbol.key.text_range());
                            Some((
                                symbol.key,
                                DocumentSymbol {
                                    name: render_symbol_name(symbol, db),
                                    detail: types_analyzer::resolve_field_type(db, document, symbol.key, symbol.region)
                                        .map(|ty| ty.render(db).to_string()),
                                    kind: LspSymbolKind::Field,
                                    tags,
                                    range,
                                    selection_range,
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
                        | SymbolKind::FieldRef
                        | SymbolKind::TagRef
                        | SymbolKind::DataRef
                        | SymbolKind::ElemRef => None,
                    }
                })
                .collect::<FxHashMap<_, _>>();
            symbol_table
                .symbols
                .values()
                .filter(|symbol| symbol.region.kind() != SyntaxKind::ROOT)
                .rev()
                .for_each(|symbol| {
                    if let Some((mut lsp_symbol, parent)) =
                        symbols_map.remove(&symbol.key).zip(symbols_map.get_mut(&symbol.region))
                    {
                        if let Some(children) = &mut lsp_symbol.children {
                            children.sort_unstable_by_key(|symbol| symbol.range.start);
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
                        children.sort_unstable_by_key(|symbol| symbol.range.start);
                        Some(lsp_symbol)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            lsp_symbols.sort_unstable_by_key(|symbol| symbol.range.start);
            lsp_symbols
        })
    }
}

fn render_symbol_name(symbol: &Symbol, db: &dyn salsa::Database) -> String {
    if let Some(name) = symbol.idx.name {
        name.ident(db).to_string()
    } else if let Some(num) = symbol.idx.num {
        format!("{} {num}", symbol.kind)
    } else {
        String::new()
    }
}
