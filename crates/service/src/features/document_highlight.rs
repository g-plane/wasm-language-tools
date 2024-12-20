use crate::{
    binder::{SymbolItem, SymbolItemKey, SymbolItemKind, SymbolTable, SymbolTablesCtx},
    files::FilesCtx,
    helpers, LanguageService,
};
use line_index::LineIndex;
use lsp_types::{DocumentHighlight, DocumentHighlightKind, DocumentHighlightParams};
use rowan::Direction;
use smallvec::SmallVec;
use wat_syntax::{SyntaxElement, SyntaxKind, SyntaxNode};

impl LanguageService {
    /// Handler for `textDocument/documentHighlight` request.
    pub fn document_highlight(
        &self,
        params: DocumentHighlightParams,
    ) -> Option<Vec<DocumentHighlight>> {
        let uri = self.uri(params.text_document_position_params.text_document.uri);
        let line_index = self.line_index(uri);
        let root = SyntaxNode::new_root(self.root(uri));
        let token = super::find_meaningful_token(
            self,
            uri,
            &root,
            params.text_document_position_params.position,
        )?;
        let kind = token.kind();
        match kind {
            SyntaxKind::KEYWORD
            | SyntaxKind::INSTR_NAME
            | SyntaxKind::NUM_TYPE
            | SyntaxKind::VEC_TYPE
            | SyntaxKind::REF_TYPE
            | SyntaxKind::HEAP_TYPE
            | SyntaxKind::MEM_ARG
            | SyntaxKind::FLOAT
            | SyntaxKind::SHARE => {
                let text = token.text();
                Some(
                    root.descendants_with_tokens()
                        .filter_map(|element| match element {
                            SyntaxElement::Token(other)
                                if other.kind() == kind && other.text() == text =>
                            {
                                Some(DocumentHighlight {
                                    range: helpers::rowan_range_to_lsp_range(
                                        &line_index,
                                        other.text_range(),
                                    ),
                                    kind: Some(DocumentHighlightKind::TEXT),
                                })
                            }
                            _ => None,
                        })
                        .collect(),
                )
            }
            SyntaxKind::IDENT | SyntaxKind::INT | SyntaxKind::UNSIGNED_INT => {
                let symbol_table = self.symbol_table(uri);
                let key = token.parent()?.into();
                let current_symbol = symbol_table
                    .symbols
                    .iter()
                    .find(|symbol| symbol.key == key)?;
                match &current_symbol.kind {
                    SymbolItemKind::Module => None,
                    SymbolItemKind::Func
                    | SymbolItemKind::Param
                    | SymbolItemKind::Local
                    | SymbolItemKind::Type
                    | SymbolItemKind::GlobalDef
                    | SymbolItemKind::MemoryDef
                    | SymbolItemKind::TableDef => {
                        let ref_kind = match current_symbol.kind {
                            SymbolItemKind::Func => SymbolItemKind::Call,
                            SymbolItemKind::Param | SymbolItemKind::Local => {
                                SymbolItemKind::LocalRef
                            }
                            SymbolItemKind::Type => SymbolItemKind::TypeUse,
                            SymbolItemKind::GlobalDef => SymbolItemKind::GlobalRef,
                            SymbolItemKind::MemoryDef => SymbolItemKind::MemoryRef,
                            SymbolItemKind::TableDef => SymbolItemKind::TableRef,
                            _ => return None,
                        };
                        Some(
                            symbol_table
                                .symbols
                                .iter()
                                .filter(|symbol| {
                                    if symbol.kind == current_symbol.kind {
                                        current_symbol.idx == symbol.idx
                                            && symbol.region == current_symbol.region
                                    } else if symbol.kind == ref_kind {
                                        symbol.idx.is_defined_by(&current_symbol.idx)
                                            && symbol.region == current_symbol.region
                                    } else {
                                        false
                                    }
                                })
                                .filter_map(|symbol| {
                                    create_symbol_highlight(symbol, &root, &line_index)
                                })
                                .collect(),
                        )
                    }
                    SymbolItemKind::Call
                    | SymbolItemKind::TypeUse
                    | SymbolItemKind::GlobalRef
                    | SymbolItemKind::MemoryRef
                    | SymbolItemKind::TableRef => {
                        let def_kind = match current_symbol.kind {
                            SymbolItemKind::Call => SymbolItemKind::Func,
                            SymbolItemKind::TypeUse => SymbolItemKind::Type,
                            SymbolItemKind::GlobalRef => SymbolItemKind::GlobalDef,
                            SymbolItemKind::MemoryRef => SymbolItemKind::MemoryDef,
                            SymbolItemKind::TableRef => SymbolItemKind::TableDef,
                            _ => return None,
                        };
                        let defs = symbol_table
                            .find_defs(&current_symbol.key)?
                            .collect::<SmallVec<[_; 1]>>();
                        Some(
                            symbol_table
                                .symbols
                                .iter()
                                .filter(|symbol| {
                                    if symbol.kind == def_kind {
                                        current_symbol.idx.is_defined_by(&symbol.idx)
                                            && symbol.region == current_symbol.region
                                    } else if symbol.kind == current_symbol.kind {
                                        defs.iter().any(|func| symbol.idx.is_defined_by(&func.idx))
                                            && symbol.region == current_symbol.region
                                    } else {
                                        false
                                    }
                                })
                                .filter_map(|symbol| {
                                    create_symbol_highlight(symbol, &root, &line_index)
                                })
                                .collect(),
                        )
                    }
                    SymbolItemKind::LocalRef => {
                        let param_or_local =
                            symbol_table.find_param_or_local_def(&current_symbol.key)?;
                        Some(
                            symbol_table
                                .symbols
                                .iter()
                                .filter(|symbol| match &symbol.kind {
                                    SymbolItemKind::Param | SymbolItemKind::Local => {
                                        current_symbol.idx.is_defined_by(&symbol.idx)
                                            && symbol.region == current_symbol.region
                                    }
                                    SymbolItemKind::LocalRef => {
                                        symbol.idx.is_defined_by(&param_or_local.idx)
                                            && symbol.region == current_symbol.region
                                    }
                                    _ => false,
                                })
                                .filter_map(|symbol| {
                                    create_symbol_highlight(symbol, &root, &line_index)
                                })
                                .collect(),
                        )
                    }
                    SymbolItemKind::BlockDef => Some(create_block_highlights(
                        &current_symbol.key,
                        &symbol_table,
                        &line_index,
                        &root,
                    )),
                    SymbolItemKind::BlockRef => {
                        let def_key = symbol_table.find_block_def(&key)?;
                        Some(create_block_highlights(
                            def_key,
                            &symbol_table,
                            &line_index,
                            &root,
                        ))
                    }
                }
            }
            _ => None,
        }
    }
}

fn create_symbol_highlight(
    symbol: &SymbolItem,
    root: &SyntaxNode,
    line_index: &LineIndex,
) -> Option<DocumentHighlight> {
    let node = symbol.key.ptr.to_node(root);
    node.children_with_tokens()
        .find_map(|element| match element {
            SyntaxElement::Token(token)
                if matches!(
                    token.kind(),
                    SyntaxKind::IDENT | SyntaxKind::INT | SyntaxKind::UNSIGNED_INT
                ) =>
            {
                Some(DocumentHighlight {
                    range: helpers::rowan_range_to_lsp_range(line_index, token.text_range()),
                    kind: get_highlight_kind_of_symbol(symbol, root),
                })
            }
            _ => None,
        })
}

fn create_block_highlights(
    def_key: &SymbolItemKey,
    symbol_table: &SymbolTable,
    line_index: &LineIndex,
    root: &SyntaxNode,
) -> Vec<DocumentHighlight> {
    let mut highlights = Vec::with_capacity(1);
    if let Some(highlight) = symbol_table
        .symbols
        .iter()
        .find(|symbol| symbol.key == *def_key)
        .and_then(|def_symbol| create_symbol_highlight(def_symbol, root, line_index))
    {
        highlights.push(highlight);
    }
    highlights.extend(
        symbol_table
            .blocks
            .iter()
            .filter(|block| {
                block.def_key == *def_key && block.ref_idx.is_defined_by(&block.def_idx)
            })
            .map(|block| DocumentHighlight {
                range: helpers::rowan_range_to_lsp_range(
                    line_index,
                    block.ref_key.ptr.text_range(),
                ),
                kind: Some(DocumentHighlightKind::READ),
            }),
    );
    highlights
}

fn get_highlight_kind_of_symbol(
    symbol: &SymbolItem,
    root: &SyntaxNode,
) -> Option<DocumentHighlightKind> {
    match symbol.kind {
        SymbolItemKind::Func
        | SymbolItemKind::Param
        | SymbolItemKind::Local
        | SymbolItemKind::Type
        | SymbolItemKind::GlobalDef
        | SymbolItemKind::MemoryDef
        | SymbolItemKind::TableDef
        | SymbolItemKind::BlockDef => Some(DocumentHighlightKind::WRITE),
        SymbolItemKind::Call
        | SymbolItemKind::TypeUse
        | SymbolItemKind::MemoryRef
        | SymbolItemKind::BlockRef => Some(DocumentHighlightKind::READ),
        SymbolItemKind::LocalRef | SymbolItemKind::GlobalRef | SymbolItemKind::TableRef => {
            let node = symbol.key.ptr.to_node(root);
            if node
                .siblings_with_tokens(Direction::Prev)
                .any(|element| is_write_access_instr(element, &node))
            {
                Some(DocumentHighlightKind::WRITE)
            } else {
                Some(DocumentHighlightKind::READ)
            }
        }
        SymbolItemKind::Module => None,
    }
}

fn is_write_access_instr(element: SyntaxElement, node: &SyntaxNode) -> bool {
    if let SyntaxElement::Token(token) = element {
        if token.kind() != SyntaxKind::INSTR_NAME {
            return false;
        }
        let text = token.text();
        if text == "table.copy" {
            // The first operand in `table.copy` is the destination table.
            node.siblings_with_tokens(Direction::Prev)
                .skip(1)
                .all(|element| element.kind() != SyntaxKind::OPERAND)
        } else {
            matches!(
                text,
                "local.set"
                    | "global.set"
                    | "table.init"
                    | "table.set"
                    | "table.grow"
                    | "table.fill"
            )
        }
    } else {
        false
    }
}
