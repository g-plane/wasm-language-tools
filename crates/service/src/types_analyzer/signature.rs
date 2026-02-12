use super::{
    def_type::get_def_types,
    extractor::extract_sig,
    types::{OperandType, ValType},
};
use crate::{
    binder::{SymbolKey, SymbolTable},
    document::Document,
    idx::InternIdent,
};
use wat_syntax::{
    GreenNode, NodeOrToken, SyntaxKind, SyntaxNodePtr,
    ast::{AstNode, TypeUse, support},
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, salsa::Update)]
pub(crate) struct Signature<'db> {
    pub(crate) params: Vec<(ValType<'db>, Option<InternIdent<'db>>)>,
    pub(crate) results: Vec<ValType<'db>>,
}
impl<'db> Signature<'db> {
    pub(crate) fn matches(
        &self,
        other: &Self,
        db: &'db dyn salsa::Database,
        document: Document,
        module_id: u32,
    ) -> bool {
        self.params.len() == other.params.len()
            && self.results.len() == other.results.len()
            && other
                .params
                .iter()
                .zip(&self.params)
                .all(|((a, _), (b, _))| a.matches(b, db, document, module_id))
            && self
                .results
                .iter()
                .zip(&other.results)
                .all(|(a, b)| a.matches(b, db, document, module_id))
    }

    pub(crate) fn type_equals(
        &self,
        other: &Self,
        db: &'db dyn salsa::Database,
        document: Document,
        module_id: u32,
    ) -> bool {
        self.params.len() == other.params.len()
            && self.results.len() == other.results.len()
            && self
                .params
                .iter()
                .zip(&other.params)
                .all(|((a, _), (b, _))| a.type_equals(b, db, document, module_id))
            && self
                .results
                .iter()
                .zip(&other.results)
                .all(|(a, b)| a.type_equals(b, db, document, module_id))
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ResolvedSig<'db> {
    pub(crate) params: Vec<OperandType<'db>>,
    pub(crate) results: Vec<OperandType<'db>>,
}

impl<'db> From<Signature<'db>> for ResolvedSig<'db> {
    fn from(sig: Signature<'db>) -> Self {
        ResolvedSig {
            params: sig.params.into_iter().map(|(ty, _)| OperandType::Val(ty)).collect(),
            results: sig.results.into_iter().map(OperandType::Val).collect(),
        }
    }
}

pub(crate) fn get_func_sig<'db>(
    db: &'db dyn salsa::Database,
    document: Document,
    key: SymbolKey,
    green: &GreenNode,
) -> Signature<'db> {
    green
        .children()
        .find_map(|child| match child {
            NodeOrToken::Node(node) if node.kind() == SyntaxKind::TYPE_USE => Some(node),
            _ => None,
        })
        .and_then(|type_use| {
            if type_use
                .children()
                .any(|child| matches!(child.kind(), SyntaxKind::PARAM | SyntaxKind::RESULT))
            {
                Some(extract_sig(db, type_use))
            } else {
                let node = key.to_node(&document.root_tree(db));
                let symbol_table = SymbolTable::of(db, document);
                let def_types = get_def_types(db, document);
                support::child::<TypeUse>(&node)
                    .and_then(|type_use| type_use.index())
                    .and_then(|idx| symbol_table.resolved.get(&SymbolKey::new(idx.syntax())))
                    .and_then(|def_key| def_types.get(def_key))
                    .and_then(|def_type| def_type.comp.as_func().cloned())
            }
        })
        .unwrap_or_default()
}

pub(crate) fn get_type_use_sig<'db>(
    db: &'db dyn salsa::Database,
    document: Document,
    ptr: SyntaxNodePtr,
    type_use: &GreenNode,
) -> Signature<'db> {
    if type_use
        .children()
        .any(|child| matches!(child.kind(), SyntaxKind::PARAM | SyntaxKind::RESULT))
    {
        extract_sig(db, type_use)
    } else {
        let symbol_table = SymbolTable::of(db, document);
        let def_types = get_def_types(db, document);
        TypeUse::cast(ptr.to_node(&document.root_tree(db)))
            .and_then(|type_use| type_use.index())
            .and_then(|idx| symbol_table.resolved.get(&SymbolKey::new(idx.syntax())))
            .and_then(|def_key| def_types.get(def_key))
            .and_then(|def_type| def_type.comp.as_func().cloned())
            .unwrap_or_default()
    }
}
