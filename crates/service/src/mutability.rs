use crate::{
    binder::{SymbolKey, SymbolKind, SymbolTablesCtx},
    syntax_tree::SyntaxTreeCtx,
    uri::InternUri,
};
use rowan::{
    ast::{support, AstNode},
    TextRange,
};
use rustc_hash::FxHashMap;
use std::sync::Arc;
use wat_syntax::{
    ast::{CompType, FieldType, ImportDescGlobalType, ModuleFieldGlobal, PlainInstr, TypeDef},
    SyntaxKind, SyntaxNode,
};

#[salsa::query_group(Mutabilities)]
pub(crate) trait MutabilitiesCtx: SyntaxTreeCtx + SymbolTablesCtx {
    #[salsa::memoized]
    #[salsa::invoke(create_mutabilities)]
    fn mutabilities(&self, uri: InternUri) -> Arc<FxHashMap<SymbolKey, Mutability>>;

    #[salsa::memoized]
    #[salsa::invoke(create_mutation_actions)]
    fn mutation_actions(&self, uri: InternUri) -> Arc<FxHashMap<SymbolKey, MutationAction>>;
}

fn create_mutabilities(
    db: &dyn MutabilitiesCtx,
    uri: InternUri,
) -> Arc<FxHashMap<SymbolKey, Mutability>> {
    let root = SyntaxNode::new_root(db.root(uri));
    let symbol_table = db.symbol_table(uri);
    let mutabilities = symbol_table
        .symbols
        .iter()
        .filter_map(|symbol| match symbol.kind {
            SymbolKind::GlobalDef => {
                let node = symbol.key.to_node(&root);
                match node.kind() {
                    SyntaxKind::MODULE_FIELD_GLOBAL => {
                        let global = ModuleFieldGlobal::cast(node)?;
                        let range = global
                            .global_type()
                            .and_then(|global_type| global_type.mut_keyword())
                            .map(|token| token.text_range());
                        Some((
                            symbol.key,
                            Mutability {
                                mut_keyword: range,
                                cross_module: global.export().is_some(),
                            },
                        ))
                    }
                    SyntaxKind::IMPORT_DESC_GLOBAL_TYPE => {
                        let global = ImportDescGlobalType::cast(node)?;
                        let range = global
                            .global_type()
                            .and_then(|global_type| global_type.mut_keyword())
                            .map(|token| token.text_range());
                        Some((
                            symbol.key,
                            Mutability {
                                mut_keyword: range,
                                cross_module: true,
                            },
                        ))
                    }
                    _ => None,
                }
            }
            SymbolKind::Type => TypeDef::cast(symbol.key.to_node(&root))
                .and_then(|type_def| type_def.sub_type())
                .and_then(|sub_type| sub_type.comp_type())
                .and_then(|comp_type| {
                    if let CompType::Array(array_type) = comp_type {
                        array_type.field_type()
                    } else {
                        None
                    }
                })
                .map(|field_type| {
                    let range = field_type.mut_keyword().map(|token| token.text_range());
                    (
                        symbol.key,
                        Mutability {
                            mut_keyword: range,
                            cross_module: false,
                        },
                    )
                }),
            SymbolKind::FieldDef => {
                let node = symbol.key.to_node(&root);
                let range = if node.kind() == SyntaxKind::FIELD {
                    support::child::<FieldType>(&node)
                } else {
                    FieldType::cast(node)
                }
                .and_then(|field_type| field_type.mut_keyword())
                .map(|token| token.text_range());
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
        .collect();
    Arc::new(mutabilities)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Mutability {
    pub(crate) mut_keyword: Option<TextRange>,
    pub(crate) cross_module: bool,
}

fn create_mutation_actions(
    db: &dyn MutabilitiesCtx,
    uri: InternUri,
) -> Arc<FxHashMap<SymbolKey, MutationAction>> {
    let root = SyntaxNode::new_root(db.root(uri));
    let symbol_table = db.symbol_table(uri);
    let mutation_actions = symbol_table
        .symbols
        .iter()
        .filter_map(|symbol| match symbol.kind {
            SymbolKind::GlobalRef => {
                let parent = symbol.key.to_node(&root).parent()?;
                let kind = match parent.kind() {
                    SyntaxKind::PLAIN_INSTR => {
                        match PlainInstr::cast(parent)?.instr_name()?.text() {
                            "global.get" => MutationActionKind::Get,
                            "global.set" => MutationActionKind::Set,
                            _ => return None,
                        }
                    }
                    SyntaxKind::EXPORT_DESC_GLOBAL => MutationActionKind::Export,
                    _ => return None,
                };
                let target = symbol_table
                    .find_def_by_symbol(symbol)
                    .map(|symbol| symbol.key);
                Some((symbol.key, MutationAction { target, kind }))
            }
            SymbolKind::FieldRef => {
                let parent = symbol.key.to_node(&root).parent()?;
                let kind = match PlainInstr::cast(parent)?.instr_name()?.text() {
                    "struct.get" | "struct.get_s" | "struct.get_u" => MutationActionKind::Get,
                    "struct.set" => MutationActionKind::Set,
                    _ => return None,
                };
                let target = symbol_table
                    .find_def_by_symbol(symbol)
                    .map(|symbol| symbol.key);
                Some((symbol.key, MutationAction { target, kind }))
            }
            SymbolKind::TypeUse => {
                let current_node = symbol.key.to_node(&root);
                let parent = current_node.parent()?;
                let kind = match PlainInstr::cast(parent.clone())?.instr_name()?.text() {
                    "array.get" | "array.get_s" | "array.get_u" => MutationActionKind::Get,
                    "array.set" | "array.fill" | "array.init_data" | "array.init_elem" => {
                        MutationActionKind::Set
                    }
                    "array.copy" => {
                        if parent.children().next() == Some(current_node) {
                            MutationActionKind::Set
                        } else {
                            MutationActionKind::Get
                        }
                    }
                    _ => return None,
                };
                let target = symbol_table
                    .find_def_by_symbol(symbol)
                    .map(|symbol| symbol.key);
                Some((symbol.key, MutationAction { target, kind }))
            }
            _ => None,
        })
        .collect();
    Arc::new(mutation_actions)
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
