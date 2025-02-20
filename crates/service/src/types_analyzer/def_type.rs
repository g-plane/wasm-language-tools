use super::TypesAnalyzerCtx;
use crate::{
    binder::{SymbolKey, SymbolKind},
    idx::Idx,
    uri::InternUri,
};
use rowan::ast::AstNode;
use std::sync::Arc;
use wat_syntax::{
    ast::{CompType, TypeDef},
    SyntaxNode,
};

pub(super) fn create_def_types(db: &dyn TypesAnalyzerCtx, uri: InternUri) -> Arc<Vec<DefType>> {
    let root = SyntaxNode::new_root(db.root(uri));
    let symbol_table = db.symbol_table(uri);
    let types = symbol_table
        .symbols
        .iter()
        .filter(|symbol| symbol.kind == SymbolKind::Type)
        .filter_map(|symbol| {
            let node = TypeDef::cast(symbol.key.to_node(&root))?;
            match node.sub_type()?.comp_type()? {
                CompType::Func(..) => Some(DefType {
                    key: symbol.key,
                    idx: symbol.idx,
                    kind: DefTypeKind::Func,
                }),
                CompType::Struct(..) => Some(DefType {
                    key: symbol.key,
                    idx: symbol.idx,
                    kind: DefTypeKind::Struct,
                }),
                CompType::Array(..) => Some(DefType {
                    key: symbol.key,
                    idx: symbol.idx,
                    kind: DefTypeKind::Array,
                }),
            }
        })
        .collect();
    Arc::new(types)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DefType {
    pub key: SymbolKey,
    pub idx: Idx,
    pub kind: DefTypeKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum DefTypeKind {
    Func,
    Struct,
    Array,
}
