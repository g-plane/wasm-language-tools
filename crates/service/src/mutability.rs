use crate::{
    binder::{SymbolKey, SymbolKind, SymbolTable},
    document::Document,
};
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;
use wat_syntax::{
    AmberNode, SyntaxKind, TextRange,
    ast::{AstNode, PlainInstr},
};

#[salsa::tracked(returns(ref))]
pub(crate) fn get_mutabilities(
    db: &dyn salsa::Database,
    document: Document,
) -> IndexMap<SymbolKey, Mutability, FxBuildHasher> {
    fn extract_mut(node: AmberNode) -> Option<TextRange> {
        node.tokens_by_kind(SyntaxKind::KEYWORD)
            .next()
            .map(|token| token.text_range())
    }
    fn extract_mut_from_global(node: AmberNode) -> Option<TextRange> {
        node.children_by_kind(SyntaxKind::GLOBAL_TYPE)
            .next()
            .and_then(extract_mut)
    }

    let symbol_table = SymbolTable::of(db, document);
    symbol_table
        .symbols
        .values()
        .filter_map(|symbol| match symbol.kind {
            SymbolKind::GlobalDef => {
                let node = symbol.amber();
                match node.kind() {
                    SyntaxKind::MODULE_FIELD_GLOBAL => Some((
                        symbol.key,
                        Mutability {
                            mut_keyword: extract_mut_from_global(node),
                            cross_module: node.children_by_kind(SyntaxKind::EXPORT).count() > 0,
                        },
                    )),
                    SyntaxKind::EXTERN_TYPE_GLOBAL => Some((
                        symbol.key,
                        Mutability {
                            mut_keyword: extract_mut_from_global(node),
                            cross_module: true,
                        },
                    )),
                    _ => None,
                }
            }
            SymbolKind::Type => symbol
                .amber()
                .children_by_kind(SyntaxKind::SUB_TYPE)
                .next()
                .and_then(|sub_type| sub_type.children_by_kind(SyntaxKind::ARRAY_TYPE).next())
                .and_then(|array_type| array_type.children_by_kind(SyntaxKind::FIELD_TYPE).next())
                .map(|field_type| {
                    (
                        symbol.key,
                        Mutability {
                            mut_keyword: extract_mut(field_type),
                            cross_module: false,
                        },
                    )
                }),
            SymbolKind::FieldDef => {
                let node = symbol.amber();
                let range = if node.kind() == SyntaxKind::FIELD {
                    node.children_by_kind(SyntaxKind::FIELD_TYPE).next()
                } else {
                    Some(node)
                }
                .and_then(extract_mut);
                Some((
                    symbol.key,
                    Mutability {
                        mut_keyword: range,
                        cross_module: false,
                    },
                ))
            }
            _ => None,
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Mutability {
    pub(crate) mut_keyword: Option<TextRange>,
    pub(crate) cross_module: bool,
}

#[salsa::tracked(returns(ref))]
pub(crate) fn get_mutation_actions(
    db: &dyn salsa::Database,
    document: Document,
) -> IndexMap<SymbolKey, MutationAction, FxBuildHasher> {
    let root = document.root_tree(db);
    let symbol_table = SymbolTable::of(db, document);
    symbol_table
        .symbols
        .values()
        .filter_map(|symbol| match symbol.kind {
            SymbolKind::GlobalRef => {
                let parent = symbol.key.to_node(&root)?.parent()?;
                let kind = match parent.kind() {
                    SyntaxKind::PLAIN_INSTR => match PlainInstr::cast(parent)?.instr_name()?.text() {
                        "global.get" => MutationActionKind::Get,
                        "global.set" => MutationActionKind::Set,
                        _ => return None,
                    },
                    SyntaxKind::EXTERN_IDX_GLOBAL => MutationActionKind::Export,
                    _ => return None,
                };
                let target = symbol_table.resolved.get(&symbol.key).copied();
                Some((symbol.key, MutationAction { target, kind }))
            }
            SymbolKind::FieldRef => {
                let parent = symbol.key.to_node(&root)?.parent()?;
                let kind = match PlainInstr::cast(parent)?.instr_name()?.text() {
                    "struct.get" | "struct.get_s" | "struct.get_u" => MutationActionKind::Get,
                    "struct.set" => MutationActionKind::Set,
                    _ => return None,
                };
                let target = symbol_table.resolved.get(&symbol.key).copied();
                Some((symbol.key, MutationAction { target, kind }))
            }
            SymbolKind::TypeUse => {
                let current_node = symbol.key.to_node(&root)?;
                let parent = current_node.parent()?;
                let kind = match PlainInstr::cast(parent.clone())?.instr_name()?.text() {
                    "array.get" | "array.get_s" | "array.get_u" => MutationActionKind::Get,
                    "array.set" | "array.fill" | "array.init_data" | "array.init_elem" => MutationActionKind::Set,
                    "array.copy" => {
                        if parent.children().next() == Some(current_node) {
                            MutationActionKind::Set
                        } else {
                            MutationActionKind::Get
                        }
                    }
                    _ => return None,
                };
                let target = symbol_table.resolved.get(&symbol.key).copied();
                Some((symbol.key, MutationAction { target, kind }))
            }
            _ => None,
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MutationAction {
    pub(crate) target: Option<SymbolKey>,
    pub(crate) kind: MutationActionKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum MutationActionKind {
    Get,
    Set,
    Export,
}
