use super::{
    extractor::extract_fields,
    signature::Signature,
    types::{FieldType, Fields},
    TypesAnalyzerCtx,
};
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
                CompType::Func(func_type) => Some(DefType {
                    key: symbol.key,
                    idx: symbol.idx,
                    kind: DefTypeKind::Func(db.extract_sig(func_type.syntax().green().into())),
                }),
                CompType::Struct(struct_type) => Some(DefType {
                    key: symbol.key,
                    idx: symbol.idx,
                    kind: DefTypeKind::Struct(extract_fields(db, &struct_type)),
                }),
                CompType::Array(array_type) => Some(DefType {
                    key: symbol.key,
                    idx: symbol.idx,
                    kind: DefTypeKind::Array(
                        array_type
                            .field_type()
                            .and_then(|node| FieldType::from_ast(&node, db)),
                    ),
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
    Func(Signature),
    Struct(Fields),
    Array(Option<FieldType>),
}
impl DefTypeKind {
    pub(crate) fn matches(
        &self,
        other: &Self,
        db: &dyn TypesAnalyzerCtx,
        uri: InternUri,
        module_id: u32,
    ) -> bool {
        match (self, other) {
            (DefTypeKind::Func(a), DefTypeKind::Func(b)) => a.matches(b, db, uri, module_id),
            (DefTypeKind::Struct(a), DefTypeKind::Struct(b)) => a.matches(b, db, uri, module_id),
            (DefTypeKind::Array(Some(a)), DefTypeKind::Array(Some(b))) => {
                a.matches(b, db, uri, module_id)
            }
            _ => false,
        }
    }
}
