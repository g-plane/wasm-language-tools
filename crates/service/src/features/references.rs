use super::find_meaningful_token;
use crate::{
    binder::{SymbolItem, SymbolItemKind, SymbolTable, SymbolTablesCtx},
    files::FilesCtx,
    helpers, LanguageService,
};
use line_index::LineIndex;
use lsp_types::{Location, ReferenceParams};
use rowan::ast::support::token;
use smallvec::SmallVec;
use wat_syntax::{SyntaxKind, SyntaxNode};

impl LanguageService {
    /// Handler for `textDocument/references` request.
    pub fn find_references(&self, params: ReferenceParams) -> Option<Vec<Location>> {
        let uri = self.uri(params.text_document_position.text_document.uri.clone());
        let root = SyntaxNode::new_root(self.root(uri));
        let token =
            find_meaningful_token(self, uri, &root, params.text_document_position.position)?;
        if !matches!(
            token.kind(),
            SyntaxKind::IDENT
                | SyntaxKind::INT
                | SyntaxKind::UNSIGNED_INT
                | SyntaxKind::NUM_TYPE
                | SyntaxKind::VEC_TYPE
                | SyntaxKind::REF_TYPE
                | SyntaxKind::KEYWORD
        ) {
            return None;
        }
        let parent = token.parent()?;

        let line_index = self.line_index(uri);
        let symbol_table = self.symbol_table(uri);

        let key = parent.into();
        let current_symbol = symbol_table
            .symbols
            .iter()
            .find(|symbol| symbol.key == key)?;
        match &current_symbol.kind {
            SymbolItemKind::Module => None,
            SymbolItemKind::Func => Some(
                symbol_table
                    .symbols
                    .iter()
                    .filter(|symbol| match symbol.kind {
                        SymbolItemKind::Func => {
                            params.context.include_declaration
                                && current_symbol.idx == symbol.idx
                                && symbol.region == current_symbol.region
                        }
                        SymbolItemKind::Call => {
                            symbol.idx.is_defined_by(&current_symbol.idx)
                                && symbol.region == current_symbol.region
                        }
                        _ => false,
                    })
                    .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                    .collect(),
            ),
            SymbolItemKind::Param => Some(
                symbol_table
                    .symbols
                    .iter()
                    .filter(|symbol| match &symbol.kind {
                        SymbolItemKind::Param => {
                            params.context.include_declaration
                                && current_symbol.idx == symbol.idx
                                && symbol.region == current_symbol.region
                        }
                        SymbolItemKind::LocalRef => {
                            symbol.idx.is_defined_by(&current_symbol.idx)
                                && symbol.region == current_symbol.region
                        }
                        _ => false,
                    })
                    .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                    .collect(),
            ),
            SymbolItemKind::Local => Some(
                symbol_table
                    .symbols
                    .iter()
                    .filter(|symbol| match &symbol.kind {
                        SymbolItemKind::Local => {
                            params.context.include_declaration
                                && current_symbol.idx == symbol.idx
                                && symbol.region == current_symbol.region
                        }
                        SymbolItemKind::LocalRef => {
                            symbol.idx.is_defined_by(&current_symbol.idx)
                                && symbol.region == current_symbol.region
                        }
                        _ => false,
                    })
                    .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                    .collect(),
            ),
            SymbolItemKind::Call => {
                let funcs = symbol_table
                    .find_defs(&current_symbol.key, SymbolItemKind::Func)?
                    .collect::<SmallVec<[_; 1]>>();
                Some(
                    symbol_table
                        .symbols
                        .iter()
                        .filter(|symbol| match &symbol.kind {
                            SymbolItemKind::Func => {
                                params.context.include_declaration
                                    && current_symbol.idx.is_defined_by(&symbol.idx)
                                    && symbol.region == current_symbol.region
                            }
                            SymbolItemKind::Call => {
                                funcs.iter().any(|func| symbol.idx.is_defined_by(&func.idx))
                                    && symbol.region == current_symbol.region
                            }
                            _ => false,
                        })
                        .map(|symbol| {
                            create_location_by_symbol(&params, &line_index, symbol, &root)
                        })
                        .collect(),
                )
            }
            SymbolItemKind::LocalRef => {
                let param_or_local = symbol_table.find_param_or_local_def(&current_symbol.key)?;
                Some(
                    symbol_table
                        .symbols
                        .iter()
                        .filter(|symbol| match &symbol.kind {
                            SymbolItemKind::Param | SymbolItemKind::Local => {
                                params.context.include_declaration
                                    && current_symbol.idx.is_defined_by(&symbol.idx)
                                    && symbol.region == current_symbol.region
                            }
                            SymbolItemKind::LocalRef => {
                                symbol.idx.is_defined_by(&param_or_local.idx)
                                    && symbol.region == current_symbol.region
                            }
                            _ => false,
                        })
                        .map(|symbol| {
                            create_location_by_symbol(&params, &line_index, symbol, &root)
                        })
                        .collect(),
                )
            }
            SymbolItemKind::Type => Some(
                symbol_table
                    .symbols
                    .iter()
                    .filter(|symbol| match &symbol.kind {
                        SymbolItemKind::Type => {
                            params.context.include_declaration
                                && current_symbol.idx == symbol.idx
                                && symbol.region == current_symbol.region
                        }
                        SymbolItemKind::TypeUse => {
                            symbol.idx.is_defined_by(&current_symbol.idx)
                                && symbol.region == current_symbol.region
                        }
                        _ => false,
                    })
                    .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                    .collect(),
            ),
            SymbolItemKind::TypeUse => {
                let types = symbol_table
                    .find_defs(&current_symbol.key, SymbolItemKind::Type)?
                    .collect::<SmallVec<[_; 1]>>();
                Some(
                    symbol_table
                        .symbols
                        .iter()
                        .filter(|symbol| match &symbol.kind {
                            SymbolItemKind::Type => {
                                params.context.include_declaration
                                    && current_symbol.idx.is_defined_by(&symbol.idx)
                                    && symbol.region == current_symbol.region
                            }
                            SymbolItemKind::TypeUse => {
                                types.iter().any(|ty| symbol.idx.is_defined_by(&ty.idx))
                                    && symbol.region == current_symbol.region
                            }
                            _ => false,
                        })
                        .map(|symbol| {
                            create_location_by_symbol(&params, &line_index, symbol, &root)
                        })
                        .collect(),
                )
            }
            SymbolItemKind::GlobalDef => Some(
                symbol_table
                    .symbols
                    .iter()
                    .filter(|symbol| match &symbol.kind {
                        SymbolItemKind::GlobalDef => {
                            params.context.include_declaration
                                && current_symbol.idx == symbol.idx
                                && symbol.region == current_symbol.region
                        }
                        SymbolItemKind::GlobalRef => {
                            symbol.idx.is_defined_by(&current_symbol.idx)
                                && symbol.region == current_symbol.region
                        }
                        _ => false,
                    })
                    .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                    .collect(),
            ),
            SymbolItemKind::GlobalRef => {
                let globals = symbol_table
                    .find_defs(&current_symbol.key, SymbolItemKind::GlobalDef)?
                    .collect::<SmallVec<[_; 1]>>();
                Some(
                    symbol_table
                        .symbols
                        .iter()
                        .filter(|symbol| match &symbol.kind {
                            SymbolItemKind::GlobalDef => {
                                params.context.include_declaration
                                    && current_symbol.idx.is_defined_by(&symbol.idx)
                                    && symbol.region == current_symbol.region
                            }
                            SymbolItemKind::GlobalRef => {
                                globals
                                    .iter()
                                    .any(|global| symbol.idx.is_defined_by(&global.idx))
                                    && symbol.region == current_symbol.region
                            }
                            _ => false,
                        })
                        .map(|symbol| {
                            create_location_by_symbol(&params, &line_index, symbol, &root)
                        })
                        .collect(),
                )
            }
            SymbolItemKind::MemoryDef => Some(
                symbol_table
                    .symbols
                    .iter()
                    .filter(|symbol| match &symbol.kind {
                        SymbolItemKind::MemoryDef => {
                            params.context.include_declaration
                                && current_symbol.idx == symbol.idx
                                && symbol.region == current_symbol.region
                        }
                        SymbolItemKind::MemoryRef => {
                            symbol.idx.is_defined_by(&current_symbol.idx)
                                && symbol.region == current_symbol.region
                        }
                        _ => false,
                    })
                    .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                    .collect(),
            ),
            SymbolItemKind::MemoryRef => {
                let memories = symbol_table
                    .find_defs(&current_symbol.key, SymbolItemKind::MemoryDef)?
                    .collect::<SmallVec<[_; 1]>>();
                Some(
                    symbol_table
                        .symbols
                        .iter()
                        .filter(|symbol| match &symbol.kind {
                            SymbolItemKind::MemoryDef => {
                                params.context.include_declaration
                                    && current_symbol.idx.is_defined_by(&symbol.idx)
                                    && symbol.region == current_symbol.region
                            }
                            SymbolItemKind::MemoryRef => {
                                memories
                                    .iter()
                                    .any(|memory| symbol.idx.is_defined_by(&memory.idx))
                                    && symbol.region == current_symbol.region
                            }
                            _ => false,
                        })
                        .map(|symbol| {
                            create_location_by_symbol(&params, &line_index, symbol, &root)
                        })
                        .collect(),
                )
            }
            SymbolItemKind::TableDef => Some(
                symbol_table
                    .symbols
                    .iter()
                    .filter(|symbol| match &symbol.kind {
                        SymbolItemKind::TableDef => {
                            params.context.include_declaration
                                && current_symbol.idx == symbol.idx
                                && symbol.region == current_symbol.region
                        }
                        SymbolItemKind::TableRef => {
                            symbol.idx.is_defined_by(&current_symbol.idx)
                                && symbol.region == current_symbol.region
                        }
                        _ => false,
                    })
                    .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                    .collect(),
            ),
            SymbolItemKind::TableRef => {
                let tables = symbol_table
                    .find_defs(&current_symbol.key, SymbolItemKind::TableDef)?
                    .collect::<SmallVec<[_; 1]>>();
                Some(
                    symbol_table
                        .symbols
                        .iter()
                        .filter(|symbol| match &symbol.kind {
                            SymbolItemKind::TableDef => {
                                params.context.include_declaration
                                    && current_symbol.idx.is_defined_by(&symbol.idx)
                                    && symbol.region == current_symbol.region
                            }
                            SymbolItemKind::TableRef => {
                                tables
                                    .iter()
                                    .any(|memory| symbol.idx.is_defined_by(&memory.idx))
                                    && symbol.region == current_symbol.region
                            }
                            _ => false,
                        })
                        .map(|symbol| {
                            create_location_by_symbol(&params, &line_index, symbol, &root)
                        })
                        .collect(),
                )
            }
            SymbolItemKind::BlockDef => Some(get_block_refs(
                current_symbol,
                &params,
                &symbol_table,
                &line_index,
                &root,
            )),
            SymbolItemKind::BlockRef => {
                let def_symbol = symbol_table.find_block_def(&key)?;
                Some(get_block_refs(
                    def_symbol,
                    &params,
                    &symbol_table,
                    &line_index,
                    &root,
                ))
            }
        }
    }
}

fn get_block_refs(
    def_symbol: &SymbolItem,
    params: &ReferenceParams,
    symbol_table: &SymbolTable,
    line_index: &LineIndex,
    root: &SyntaxNode,
) -> Vec<Location> {
    let mut locations = Vec::with_capacity(1);
    if params.context.include_declaration {
        locations.push(create_location_by_symbol(
            params, line_index, def_symbol, root,
        ));
    }
    locations.extend(
        symbol_table
            .blocks
            .iter()
            .filter(|(_, def_key, _)| def_key == &def_symbol.key)
            .filter_map(|(ref_key, _, _)| {
                symbol_table
                    .symbols
                    .iter()
                    .find(|symbol| &symbol.key == ref_key)
            })
            .map(|symbol| create_location_by_symbol(params, line_index, symbol, root)),
    );
    locations
}

fn create_location_by_symbol(
    params: &ReferenceParams,
    line_index: &LineIndex,
    symbol: &SymbolItem,
    root: &SyntaxNode,
) -> Location {
    let node = symbol.key.ptr.to_node(root);
    let range = token(&node, SyntaxKind::IDENT)
        .or_else(|| token(&node, SyntaxKind::KEYWORD))
        .map(|token| token.text_range())
        .unwrap_or_else(|| node.text_range());
    Location {
        uri: params.text_document_position.text_document.uri.clone(),
        range: helpers::rowan_range_to_lsp_range(line_index, range),
    }
}
