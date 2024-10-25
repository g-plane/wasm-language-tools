use super::find_meaningful_token;
use crate::{
    binder::{SymbolItem, SymbolItemKind, SymbolTablesCtx},
    files::FilesCtx,
    helpers, LanguageService,
};
use line_index::LineIndex;
use lsp_types::{Location, ReferenceParams};
use rowan::ast::support::token;
use smallvec::SmallVec;
use wat_syntax::{SyntaxKind, SyntaxNode};

impl LanguageService {
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
            SymbolItemKind::Func(def_idx) => Some(
                symbol_table
                    .symbols
                    .iter()
                    .filter(|symbol| match &symbol.kind {
                        SymbolItemKind::Func(idx) => {
                            params.context.include_declaration
                                && def_idx == idx
                                && symbol.region == current_symbol.region
                        }
                        SymbolItemKind::Call(idx) => {
                            def_idx == idx && symbol.region == current_symbol.region
                        }
                        _ => false,
                    })
                    .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                    .collect(),
            ),
            SymbolItemKind::Param(def_idx) => Some(
                symbol_table
                    .symbols
                    .iter()
                    .filter(|symbol| match &symbol.kind {
                        SymbolItemKind::Param(idx) => {
                            params.context.include_declaration
                                && def_idx == idx
                                && symbol.region == current_symbol.region
                        }
                        SymbolItemKind::LocalRef(idx) => {
                            def_idx == idx && symbol.region == current_symbol.region
                        }
                        _ => false,
                    })
                    .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                    .collect(),
            ),
            SymbolItemKind::Local(def_idx) => Some(
                symbol_table
                    .symbols
                    .iter()
                    .filter(|symbol| match &symbol.kind {
                        SymbolItemKind::Local(idx) => {
                            params.context.include_declaration
                                && def_idx == idx
                                && symbol.region == current_symbol.region
                        }
                        SymbolItemKind::LocalRef(idx) => {
                            def_idx == idx && symbol.region == current_symbol.region
                        }
                        _ => false,
                    })
                    .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                    .collect(),
            ),
            SymbolItemKind::Call(ref_idx) => {
                let funcs = symbol_table
                    .find_func_defs(&current_symbol.key)?
                    .filter_map(|symbol| {
                        if let SymbolItemKind::Func(idx) = &symbol.kind {
                            Some(idx)
                        } else {
                            None
                        }
                    })
                    .collect::<SmallVec<[_; 1]>>();
                Some(
                    symbol_table
                        .symbols
                        .iter()
                        .filter(|symbol| match &symbol.kind {
                            SymbolItemKind::Func(idx) => {
                                params.context.include_declaration
                                    && ref_idx == idx
                                    && symbol.region == current_symbol.region
                            }
                            SymbolItemKind::Call(idx) => {
                                funcs.iter().any(|def_idx| *def_idx == idx)
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
            SymbolItemKind::LocalRef(ref_idx) => {
                let Some(SymbolItem {
                    kind: SymbolItemKind::Param(def_idx) | SymbolItemKind::Local(def_idx),
                    ..
                }) = symbol_table.find_param_or_local_def(&current_symbol.key)
                else {
                    return None;
                };
                Some(
                    symbol_table
                        .symbols
                        .iter()
                        .filter(|symbol| match &symbol.kind {
                            SymbolItemKind::Param(idx) | SymbolItemKind::Local(idx) => {
                                params.context.include_declaration
                                    && ref_idx == idx
                                    && symbol.region == current_symbol.region
                            }
                            SymbolItemKind::LocalRef(idx) => {
                                def_idx == idx && symbol.region == current_symbol.region
                            }
                            _ => false,
                        })
                        .map(|symbol| {
                            create_location_by_symbol(&params, &line_index, symbol, &root)
                        })
                        .collect(),
                )
            }
            SymbolItemKind::Type(def_idx) => Some(
                symbol_table
                    .symbols
                    .iter()
                    .filter(|symbol| match &symbol.kind {
                        SymbolItemKind::Type(idx) => {
                            params.context.include_declaration
                                && def_idx == idx
                                && symbol.region == current_symbol.region
                        }
                        SymbolItemKind::TypeUse(idx) => {
                            def_idx == idx && symbol.region == current_symbol.region
                        }
                        _ => false,
                    })
                    .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                    .collect(),
            ),
            SymbolItemKind::TypeUse(ref_idx) => {
                let func_types = symbol_table
                    .find_type_use_defs(&current_symbol.key)?
                    .filter_map(|symbol| {
                        if let SymbolItemKind::Type(idx) = &symbol.kind {
                            Some(idx)
                        } else {
                            None
                        }
                    })
                    .collect::<SmallVec<[_; 1]>>();
                Some(
                    symbol_table
                        .symbols
                        .iter()
                        .filter(|symbol| match &symbol.kind {
                            SymbolItemKind::Type(idx) => {
                                params.context.include_declaration
                                    && ref_idx == idx
                                    && symbol.region == current_symbol.region
                            }
                            SymbolItemKind::TypeUse(idx) => {
                                func_types.iter().any(|def_idx| *def_idx == idx)
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
            SymbolItemKind::GlobalDef(def_idx) => Some(
                symbol_table
                    .symbols
                    .iter()
                    .filter(|symbol| match &symbol.kind {
                        SymbolItemKind::GlobalDef(idx) => {
                            params.context.include_declaration
                                && def_idx == idx
                                && symbol.region == current_symbol.region
                        }
                        SymbolItemKind::GlobalRef(idx) => {
                            def_idx == idx && symbol.region == current_symbol.region
                        }
                        _ => false,
                    })
                    .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                    .collect(),
            ),
            SymbolItemKind::GlobalRef(ref_idx) => {
                let globals = symbol_table
                    .find_global_defs(&current_symbol.key)?
                    .filter_map(|symbol| {
                        if let SymbolItemKind::GlobalDef(idx) = &symbol.kind {
                            Some(idx)
                        } else {
                            None
                        }
                    })
                    .collect::<SmallVec<[_; 1]>>();
                Some(
                    symbol_table
                        .symbols
                        .iter()
                        .filter(|symbol| match &symbol.kind {
                            SymbolItemKind::GlobalDef(idx) => {
                                params.context.include_declaration
                                    && ref_idx == idx
                                    && symbol.region == current_symbol.region
                            }
                            SymbolItemKind::GlobalRef(idx) => {
                                globals.iter().any(|def_idx| *def_idx == idx)
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
            SymbolItemKind::MemoryDef(def_idx) => Some(
                symbol_table
                    .symbols
                    .iter()
                    .filter(|symbol| match &symbol.kind {
                        SymbolItemKind::MemoryDef(idx) => {
                            params.context.include_declaration
                                && def_idx == idx
                                && symbol.region == current_symbol.region
                        }
                        SymbolItemKind::MemoryRef(idx) => {
                            def_idx == idx && symbol.region == current_symbol.region
                        }
                        _ => false,
                    })
                    .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                    .collect(),
            ),
            SymbolItemKind::MemoryRef(ref_idx) => {
                let memories = symbol_table
                    .find_memory_defs(&current_symbol.key)?
                    .filter_map(|symbol| {
                        if let SymbolItemKind::MemoryDef(idx) = &symbol.kind {
                            Some(idx)
                        } else {
                            None
                        }
                    })
                    .collect::<SmallVec<[_; 1]>>();
                Some(
                    symbol_table
                        .symbols
                        .iter()
                        .filter(|symbol| match &symbol.kind {
                            SymbolItemKind::MemoryDef(idx) => {
                                params.context.include_declaration
                                    && ref_idx == idx
                                    && symbol.region == current_symbol.region
                            }
                            SymbolItemKind::MemoryRef(idx) => {
                                memories.iter().any(|def_idx| *def_idx == idx)
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
        }
    }
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
