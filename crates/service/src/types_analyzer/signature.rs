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
use wat_syntax::{AmberNode, GreenNode, NodeOrToken, SyntaxKind, SyntaxNodePtr};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, salsa::Update)]
pub(crate) struct NamedSig<'db> {
    pub(crate) params: Vec<(ValType<'db>, Option<InternIdent<'db>>)>,
    pub(crate) results: Vec<ValType<'db>>,
}
impl<'db> NamedSig<'db> {
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
pub(crate) struct Sig<'db> {
    pub(crate) params: Vec<ValType<'db>>,
    pub(crate) results: Vec<ValType<'db>>,
}
impl<'db> Sig<'db> {
    pub(crate) fn from_func(db: &'db dyn salsa::Database, document: Document, node: AmberNode<'_>) -> Self {
        node.children_by_kind(SyntaxKind::TYPE_USE)
            .next()
            .map(|type_use| Self::from_type_use(db, document, type_use))
            .unwrap_or_default()
    }

    pub(crate) fn from_type_use(db: &'db dyn salsa::Database, document: Document, node: AmberNode<'_>) -> Self {
        if node
            .children_by_kind(|kind| matches!(kind, SyntaxKind::PARAM | SyntaxKind::RESULT))
            .next()
            .is_some()
        {
            let mut params = Vec::with_capacity(1);
            let mut results = Vec::with_capacity(1);
            node.children().for_each(|node| match node.kind() {
                SyntaxKind::PARAM => {
                    params.extend(
                        node.children()
                            .filter_map(|child| ValType::from_green(child.green(), db)),
                    );
                }
                SyntaxKind::RESULT => {
                    results.extend(
                        node.children()
                            .filter_map(|child| ValType::from_green(child.green(), db)),
                    );
                }
                _ => {}
            });
            Some(Self { params, results })
        } else {
            let symbol_table = SymbolTable::of(db, document);
            let def_types = get_def_types(db, document);
            node.children_by_kind(SyntaxKind::INDEX)
                .next()
                .and_then(|idx| symbol_table.resolved.get(&idx.to_ptr().into()))
                .and_then(|def_key| def_types.get(def_key))
                .and_then(|def_type| def_type.comp.as_func())
                .map(|sig| Self {
                    params: sig.params.iter().map(|(ty, _)| ty.clone()).collect(),
                    results: sig.results.clone(),
                })
        }
        .unwrap_or_default()
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ResolvedSig<'db> {
    pub(crate) params: Vec<OperandType<'db>>,
    pub(crate) results: Vec<OperandType<'db>>,
}

impl<'db> From<NamedSig<'db>> for ResolvedSig<'db> {
    fn from(sig: NamedSig<'db>) -> Self {
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
) -> NamedSig<'db> {
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
                let symbol_table = SymbolTable::of(db, document);
                let def_types = get_def_types(db, document);
                AmberNode::new(green, key.text_range().start())
                    .children_by_kind(SyntaxKind::TYPE_USE)
                    .next()
                    .and_then(|type_use| type_use.children_by_kind(SyntaxKind::INDEX).next())
                    .and_then(|idx| symbol_table.resolved.get(&idx.to_ptr().into()))
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
) -> NamedSig<'db> {
    if type_use
        .children()
        .any(|child| matches!(child.kind(), SyntaxKind::PARAM | SyntaxKind::RESULT))
    {
        extract_sig(db, type_use)
    } else {
        let symbol_table = SymbolTable::of(db, document);
        let def_types = get_def_types(db, document);
        AmberNode::new(type_use, ptr.text_range().start())
            .children_by_kind(SyntaxKind::INDEX)
            .next()
            .and_then(|idx| symbol_table.resolved.get(&idx.to_ptr().into()))
            .and_then(|def_key| def_types.get(def_key))
            .and_then(|def_type| def_type.comp.as_func().cloned())
            .unwrap_or_default()
    }
}
