use super::find_meaningful_token;
use crate::{
    binder::{Symbol, SymbolKey, SymbolKind, SymbolTablesCtx},
    helpers,
    syntax_tree::SyntaxTreeCtx,
    uri::UrisCtx,
    LanguageService,
};
use line_index::LineIndex;
use lspt::{Location, ReferenceParams};
use rowan::ast::support::token;
use smallvec::SmallVec;
use wat_syntax::{SyntaxKind, SyntaxNode};

impl LanguageService {
    /// Handler for `textDocument/references` request.
    pub fn find_references(&self, params: ReferenceParams) -> Option<Vec<Location>> {
        let uri = self.uri(params.text_document.uri.clone());
        let root = SyntaxNode::new_root(self.root(uri));
        let token = find_meaningful_token(self, uri, &root, params.position)?;
        if !matches!(
            token.kind(),
            SyntaxKind::IDENT
                | SyntaxKind::INT
                | SyntaxKind::UNSIGNED_INT
                | SyntaxKind::TYPE_KEYWORD
                | SyntaxKind::KEYWORD
        ) {
            return None;
        }
        let parent = token.parent()?;

        let line_index = self.line_index(uri);
        let symbol_table = self.symbol_table(uri);

        let key = SymbolKey::new(&parent);
        let current_symbol = symbol_table
            .symbols
            .iter()
            .find(|symbol| symbol.key == key)?;
        match &current_symbol.kind {
            SymbolKind::Module => None,
            SymbolKind::Func
            | SymbolKind::Param
            | SymbolKind::Local
            | SymbolKind::Type
            | SymbolKind::GlobalDef
            | SymbolKind::MemoryDef
            | SymbolKind::TableDef => {
                let ref_kind = match current_symbol.kind {
                    SymbolKind::Func => SymbolKind::Call,
                    SymbolKind::Param | SymbolKind::Local => SymbolKind::LocalRef,
                    SymbolKind::Type => SymbolKind::TypeUse,
                    SymbolKind::GlobalDef => SymbolKind::GlobalRef,
                    SymbolKind::MemoryDef => SymbolKind::MemoryRef,
                    SymbolKind::TableDef => SymbolKind::TableRef,
                    _ => return None,
                };
                Some(
                    symbol_table
                        .symbols
                        .iter()
                        .filter(|symbol| {
                            if symbol.kind == current_symbol.kind {
                                params.context.include_declaration
                                    && current_symbol.idx == symbol.idx
                                    && symbol.region == current_symbol.region
                            } else if symbol.kind == ref_kind {
                                symbol.idx.is_defined_by(&current_symbol.idx)
                                    && symbol.region == current_symbol.region
                            } else {
                                false
                            }
                        })
                        .map(|symbol| {
                            create_location_by_symbol(&params, &line_index, symbol, &root)
                        })
                        .collect(),
                )
            }
            SymbolKind::Call
            | SymbolKind::TypeUse
            | SymbolKind::GlobalRef
            | SymbolKind::MemoryRef
            | SymbolKind::TableRef => {
                let def_kind = match current_symbol.kind {
                    SymbolKind::Call => SymbolKind::Func,
                    SymbolKind::TypeUse => SymbolKind::Type,
                    SymbolKind::GlobalRef => SymbolKind::GlobalDef,
                    SymbolKind::MemoryRef => SymbolKind::MemoryDef,
                    SymbolKind::TableRef => SymbolKind::TableDef,
                    _ => return None,
                };
                let defs = symbol_table
                    .find_defs(current_symbol.key)?
                    .collect::<SmallVec<[_; 1]>>();
                Some(
                    symbol_table
                        .symbols
                        .iter()
                        .filter(|symbol| {
                            if symbol.kind == def_kind {
                                params.context.include_declaration
                                    && current_symbol.idx.is_defined_by(&symbol.idx)
                                    && symbol.region == current_symbol.region
                            } else if symbol.kind == current_symbol.kind {
                                defs.iter().any(|func| symbol.idx.is_defined_by(&func.idx))
                                    && symbol.region == current_symbol.region
                            } else {
                                false
                            }
                        })
                        .map(|symbol| {
                            create_location_by_symbol(&params, &line_index, symbol, &root)
                        })
                        .collect(),
                )
            }
            SymbolKind::LocalRef => {
                let param_or_local = symbol_table.find_param_or_local_def(current_symbol.key)?;
                Some(
                    symbol_table
                        .symbols
                        .iter()
                        .filter(|symbol| match &symbol.kind {
                            SymbolKind::Param | SymbolKind::Local => {
                                params.context.include_declaration
                                    && current_symbol.idx.is_defined_by(&symbol.idx)
                                    && symbol.region == current_symbol.region
                            }
                            SymbolKind::LocalRef => {
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
            SymbolKind::BlockDef => Some(
                symbol_table
                    .find_block_references(current_symbol.key, params.context.include_declaration)
                    .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                    .collect(),
            ),
            SymbolKind::BlockRef => {
                let def_key = symbol_table.find_block_def(key)?;
                Some(
                    symbol_table
                        .find_block_references(def_key, params.context.include_declaration)
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
    symbol: &Symbol,
    root: &SyntaxNode,
) -> Location {
    let node = symbol.key.to_node(root);
    let range = token(&node, SyntaxKind::IDENT)
        .or_else(|| token(&node, SyntaxKind::KEYWORD))
        .map(|token| token.text_range())
        .unwrap_or_else(|| node.text_range());
    Location {
        uri: params.text_document.uri.clone(),
        range: helpers::rowan_range_to_lsp_range(line_index, range),
    }
}
