use crate::{
    LanguageService,
    binder::{Symbol, SymbolKey, SymbolKind, SymbolTable},
    helpers,
};
use line_index::LineIndex;
use lspt::{DocumentHighlight, DocumentHighlightKind, DocumentHighlightParams};
use rowan::{Direction, ast::AstNode};
use wat_syntax::{SyntaxElement, SyntaxKind, SyntaxNode, ast::PlainInstr};

impl LanguageService {
    /// Handler for `textDocument/documentHighlight` request.
    pub fn document_highlight(
        &self,
        params: DocumentHighlightParams,
    ) -> Option<Vec<DocumentHighlight>> {
        let document = self.get_document(params.text_document.uri)?;
        let line_index = document.line_index(self);
        let root = document.root_tree(self);
        let token = super::find_meaningful_token(self, document, &root, params.position)?;
        let kind = token.kind();
        match kind {
            SyntaxKind::KEYWORD
            | SyntaxKind::INSTR_NAME
            | SyntaxKind::TYPE_KEYWORD
            | SyntaxKind::MEM_ARG
            | SyntaxKind::FLOAT => {
                let text = token.text();
                Some(
                    root.descendants_with_tokens()
                        .filter_map(|element| match element {
                            SyntaxElement::Token(other)
                                if other.kind() == kind && other.text() == text =>
                            {
                                Some(DocumentHighlight {
                                    range: helpers::rowan_range_to_lsp_range(
                                        line_index,
                                        other.text_range(),
                                    ),
                                    kind: Some(DocumentHighlightKind::Text),
                                })
                            }
                            _ => None,
                        })
                        .collect(),
                )
            }
            SyntaxKind::IDENT | SyntaxKind::INT | SyntaxKind::UNSIGNED_INT => {
                let symbol_table = SymbolTable::of(self, document);
                let key = SymbolKey::new(&token.parent()?);
                if let Some(current_symbol) =
                    symbol_table.symbols.iter().find(|symbol| symbol.key == key)
                {
                    match &current_symbol.kind {
                        SymbolKind::Module => None,
                        SymbolKind::Func
                        | SymbolKind::Param
                        | SymbolKind::Local
                        | SymbolKind::Type
                        | SymbolKind::GlobalDef
                        | SymbolKind::MemoryDef
                        | SymbolKind::TableDef
                        | SymbolKind::FieldDef => {
                            let ref_kind = match current_symbol.kind {
                                SymbolKind::Func => SymbolKind::Call,
                                SymbolKind::Param | SymbolKind::Local => SymbolKind::LocalRef,
                                SymbolKind::Type => SymbolKind::TypeUse,
                                SymbolKind::GlobalDef => SymbolKind::GlobalRef,
                                SymbolKind::MemoryDef => SymbolKind::MemoryRef,
                                SymbolKind::TableDef => SymbolKind::TableRef,
                                SymbolKind::FieldDef => SymbolKind::FieldRef,
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
                                        create_symbol_highlight(symbol, &root, line_index)
                                    })
                                    .collect(),
                            )
                        }
                        SymbolKind::Call
                        | SymbolKind::TypeUse
                        | SymbolKind::GlobalRef
                        | SymbolKind::MemoryRef
                        | SymbolKind::TableRef
                        | SymbolKind::FieldRef => {
                            let def_kind = match current_symbol.kind {
                                SymbolKind::Call => SymbolKind::Func,
                                SymbolKind::TypeUse => SymbolKind::Type,
                                SymbolKind::GlobalRef => SymbolKind::GlobalDef,
                                SymbolKind::MemoryRef => SymbolKind::MemoryDef,
                                SymbolKind::TableRef => SymbolKind::TableDef,
                                SymbolKind::FieldRef => SymbolKind::FieldDef,
                                _ => return None,
                            };
                            let def = symbol_table.find_def(current_symbol.key)?;
                            Some(
                                symbol_table
                                    .symbols
                                    .iter()
                                    .filter(|symbol| {
                                        if symbol.kind == def_kind {
                                            current_symbol.idx.is_defined_by(&symbol.idx)
                                                && symbol.region == current_symbol.region
                                        } else if symbol.kind == current_symbol.kind {
                                            symbol.idx.is_defined_by(&def.idx)
                                                && symbol.region == current_symbol.region
                                        } else {
                                            false
                                        }
                                    })
                                    .filter_map(|symbol| {
                                        create_symbol_highlight(symbol, &root, line_index)
                                    })
                                    .collect(),
                            )
                        }
                        SymbolKind::LocalRef => {
                            let param_or_local =
                                symbol_table.find_param_or_local_def(current_symbol.key)?;
                            Some(
                                symbol_table
                                    .symbols
                                    .iter()
                                    .filter(|symbol| match symbol.kind {
                                        SymbolKind::Param | SymbolKind::Local => {
                                            current_symbol.idx.is_defined_by(&symbol.idx)
                                                && symbol.region == current_symbol.region
                                        }
                                        SymbolKind::LocalRef => {
                                            symbol.idx.is_defined_by(&param_or_local.idx)
                                                && symbol.region == current_symbol.region
                                        }
                                        _ => false,
                                    })
                                    .filter_map(|symbol| {
                                        create_symbol_highlight(symbol, &root, line_index)
                                    })
                                    .collect(),
                            )
                        }
                        SymbolKind::BlockDef => Some(
                            symbol_table
                                .find_block_references(current_symbol.key, true)
                                .filter_map(|symbol| {
                                    create_symbol_highlight(symbol, &root, line_index)
                                })
                                .collect(),
                        ),
                        SymbolKind::BlockRef => {
                            let def_key = symbol_table.find_block_def(key)?;
                            Some(
                                symbol_table
                                    .find_block_references(def_key, true)
                                    .filter_map(|symbol| {
                                        create_symbol_highlight(symbol, &root, line_index)
                                    })
                                    .collect(),
                            )
                        }
                    }
                } else {
                    let text = token.text();
                    Some(
                        root.descendants_with_tokens()
                            .filter_map(|element| match element {
                                SyntaxElement::Token(other)
                                    if other.kind() == kind
                                        && other.text() == text
                                        && other
                                            .parent()
                                            .and_then(|parent| parent.parent())
                                            .and_then(PlainInstr::cast)
                                            .and_then(|instr| instr.instr_name())
                                            .is_some_and(|name| {
                                                name.text().ends_with(".const")
                                            }) =>
                                {
                                    Some(DocumentHighlight {
                                        range: helpers::rowan_range_to_lsp_range(
                                            line_index,
                                            other.text_range(),
                                        ),
                                        kind: Some(DocumentHighlightKind::Text),
                                    })
                                }
                                _ => None,
                            })
                            .collect(),
                    )
                }
            }
            _ => None,
        }
    }
}

fn create_symbol_highlight(
    symbol: &Symbol,
    root: &SyntaxNode,
    line_index: &LineIndex,
) -> Option<DocumentHighlight> {
    let node = symbol.key.to_node(root);
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

fn get_highlight_kind_of_symbol(
    symbol: &Symbol,
    root: &SyntaxNode,
) -> Option<DocumentHighlightKind> {
    match symbol.kind {
        SymbolKind::Func
        | SymbolKind::Param
        | SymbolKind::Local
        | SymbolKind::Type
        | SymbolKind::GlobalDef
        | SymbolKind::MemoryDef
        | SymbolKind::TableDef
        | SymbolKind::BlockDef
        | SymbolKind::FieldDef => Some(DocumentHighlightKind::Write),
        SymbolKind::Call | SymbolKind::TypeUse | SymbolKind::MemoryRef | SymbolKind::BlockRef => {
            Some(DocumentHighlightKind::Read)
        }
        SymbolKind::LocalRef
        | SymbolKind::GlobalRef
        | SymbolKind::TableRef
        | SymbolKind::FieldRef => {
            let node = symbol.key.to_node(root);
            if node
                .siblings_with_tokens(Direction::Prev)
                .any(|element| is_write_access_instr(element, &node))
            {
                Some(DocumentHighlightKind::Write)
            } else {
                Some(DocumentHighlightKind::Read)
            }
        }
        SymbolKind::Module => None,
    }
}

fn is_write_access_instr(element: SyntaxElement, node: &SyntaxNode) -> bool {
    if let SyntaxElement::Token(token) = element {
        if token.kind() != SyntaxKind::INSTR_NAME {
            return false;
        }
        let text = token.text();
        if text == "table.copy" {
            // The first immediate in `table.copy` is the destination table.
            node.siblings_with_tokens(Direction::Prev)
                .skip(1)
                .all(|element| element.kind() != SyntaxKind::IMMEDIATE)
        } else {
            matches!(
                text,
                "local.set"
                    | "global.set"
                    | "table.init"
                    | "table.set"
                    | "table.grow"
                    | "table.fill"
                    | "struct.set"
            )
        }
    } else {
        false
    }
}
