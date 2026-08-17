use crate::{
    LanguageService,
    binder::{Symbol, SymbolKind, SymbolTable},
    deprecation,
    helpers::{self, LineIndexExt},
    mutability,
    types_analyzer::{self, CompositeType},
};
use lspt::{DocumentSymbol, DocumentSymbolParams, SymbolKind as LspSymbolKind, SymbolTag};

impl LanguageService {
    /// Handler for `textDocument/documentSymbol` request.
    pub fn document_symbol(&self, params: DocumentSymbolParams) -> Option<Vec<DocumentSymbol>> {
        let document = self.get_document(params.text_document.uri)?;
        self.with_db(|db| {
            let line_index = document.line_index(db);
            let symbol_table = SymbolTable::of(db, document);
            let deprecation = deprecation::get_deprecation(db, document);
            let def_types = types_analyzer::get_def_types(db, document);

            let mut module_lsp_symbol = None;
            let mut mf_lsp_symbol = None;
            let mut lsp_symbols = Vec::with_capacity(1);
            symbol_table.symbols.iter().for_each(|symbol| {
                let Some(range) = line_index.convert(symbol.key.text_range()) else {
                    return;
                };
                let Some(selection_range) = line_index.convert(helpers::syntax::infer_def_poi(symbol.amber())) else {
                    return;
                };
                let tags = if deprecation.contains_key(&symbol.key) {
                    Some(vec![SymbolTag::Deprecated])
                } else {
                    None
                };
                match symbol.kind {
                    SymbolKind::Module => {
                        if let Some(mut module_lsp_symbol) = module_lsp_symbol.replace(DocumentSymbol {
                            name: render_symbol_name(symbol, db),
                            detail: None,
                            kind: LspSymbolKind::Module,
                            tags,
                            range,
                            selection_range,
                            children: Some(Vec::with_capacity(symbol.green.children_len() / 10)),
                        }) {
                            if let Some(mf_lsp_symbol) = mf_lsp_symbol.take() {
                                module_lsp_symbol.children.get_or_insert_default().push(mf_lsp_symbol);
                            }
                            lsp_symbols.push(module_lsp_symbol);
                        }
                    }
                    SymbolKind::Func => {
                        if let Some(module_lsp_symbol) = &mut module_lsp_symbol
                            && let Some(mf_lsp_symbol) = mf_lsp_symbol.replace(DocumentSymbol {
                                name: render_symbol_name(symbol, db),
                                detail: None,
                                kind: LspSymbolKind::Function,
                                tags,
                                range,
                                selection_range,
                                children: None,
                            })
                        {
                            module_lsp_symbol.children.get_or_insert_default().push(mf_lsp_symbol);
                        }
                    }
                    SymbolKind::Local => {
                        if let Some(lsp_symbol) = &mut mf_lsp_symbol {
                            lsp_symbol.children.get_or_insert_default().push(DocumentSymbol {
                                name: render_symbol_name(symbol, db),
                                detail: types_analyzer::extract_type(db, &symbol.green)
                                    .map(|ty| ty.render(db).to_string()),
                                kind: LspSymbolKind::Variable,
                                tags,
                                range,
                                selection_range,
                                children: None,
                            });
                        }
                    }
                    SymbolKind::Type => {
                        if let Some(module_lsp_symbol) = &mut module_lsp_symbol
                            && let Some(mf_lsp_symbol) = mf_lsp_symbol.replace(DocumentSymbol {
                                name: render_symbol_name(symbol, db),
                                detail: def_types.get(&symbol.key).map(|def_type| match def_type.comp {
                                    CompositeType::Func(..) => "func".into(),
                                    CompositeType::Struct(..) => "struct".into(),
                                    CompositeType::Array(..) => "array".into(),
                                    CompositeType::Cont(..) => "cont".into(),
                                }),
                                kind: LspSymbolKind::Class,
                                tags,
                                range,
                                selection_range,
                                children: None,
                            })
                        {
                            module_lsp_symbol.children.get_or_insert_default().push(mf_lsp_symbol);
                        }
                    }
                    SymbolKind::GlobalDef => {
                        if let Some(module_lsp_symbol) = &mut module_lsp_symbol {
                            let children = module_lsp_symbol.children.get_or_insert_default();
                            if let Some(mf_lsp_symbol) = mf_lsp_symbol.take() {
                                children.push(mf_lsp_symbol);
                            }
                            children.push(DocumentSymbol {
                                name: render_symbol_name(symbol, db),
                                detail: types_analyzer::extract_global_type(
                                    db,
                                    symbol_table.get_type_node_of(symbol).green(),
                                )
                                .map(|ty| {
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
                            });
                        }
                    }
                    SymbolKind::MemoryDef
                    | SymbolKind::TableDef
                    | SymbolKind::TagDef
                    | SymbolKind::DataDef
                    | SymbolKind::ElemDef => {
                        if let Some(module_lsp_symbol) = &mut module_lsp_symbol {
                            let children = module_lsp_symbol.children.get_or_insert_default();
                            if let Some(mf_lsp_symbol) = mf_lsp_symbol.take() {
                                children.push(mf_lsp_symbol);
                            }
                            children.push(DocumentSymbol {
                                name: render_symbol_name(symbol, db),
                                detail: None,
                                kind: LspSymbolKind::Variable,
                                tags,
                                range,
                                selection_range,
                                children: None,
                            });
                        }
                    }
                    SymbolKind::FieldDef => {
                        if let Some(lsp_symbol) = &mut mf_lsp_symbol {
                            lsp_symbol.children.get_or_insert_default().push(DocumentSymbol {
                                name: render_symbol_name(symbol, db),
                                detail: types_analyzer::resolve_field_type(db, document, symbol.key, symbol.region)
                                    .map(|ty| ty.render(db).to_string()),
                                kind: LspSymbolKind::Field,
                                tags,
                                range,
                                selection_range,
                                children: None,
                            });
                        }
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
                    | SymbolKind::ElemRef => {}
                }
            });
            if let Some(mut module_lsp_symbol) = module_lsp_symbol.take() {
                if let Some(mf_lsp_symbol) = mf_lsp_symbol.take() {
                    module_lsp_symbol.children.get_or_insert_default().push(mf_lsp_symbol);
                }
                lsp_symbols.push(module_lsp_symbol);
            }
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
