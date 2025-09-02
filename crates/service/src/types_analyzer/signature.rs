use super::{
    extractor::extract_sig,
    types::{OperandType, ValType},
};
use crate::{
    binder::{SymbolKey, SymbolTable},
    document::Document,
    helpers,
    idx::InternIdent,
};
use rowan::{
    GreenNodeData, NodeOrToken,
    ast::{AstNode, support},
};
use wat_syntax::{
    SyntaxKind, SyntaxNode, SyntaxNodePtr,
    ast::{BlockType, TypeUse},
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
            params: sig
                .params
                .into_iter()
                .map(|(ty, _)| OperandType::Val(ty))
                .collect(),
            results: sig.results.into_iter().map(OperandType::Val).collect(),
        }
    }
}

pub(crate) fn get_func_sig<'db>(
    db: &'db dyn salsa::Database,
    document: Document,
    ptr: SyntaxNodePtr,
    green: &GreenNodeData,
) -> Signature<'db> {
    green
        .children()
        .find_map(|child| match child {
            NodeOrToken::Node(node) if node.kind() == SyntaxKind::TYPE_USE.into() => Some(node),
            _ => None,
        })
        .and_then(|type_use| {
            if type_use.children().any(|child| {
                let kind = child.kind();
                kind == SyntaxKind::PARAM.into() || kind == SyntaxKind::RESULT.into()
            }) {
                Some(extract_sig(db, type_use.to_owned()))
            } else {
                let node = ptr.to_node(&document.root_tree(db));
                let symbol_table = SymbolTable::of(db, document);
                support::child::<TypeUse>(&node)
                    .and_then(|type_use| type_use.index())
                    .and_then(|idx| symbol_table.find_def(SymbolKey::new(idx.syntax())))
                    .and_then(|symbol| helpers::ast::find_func_type_of_type_def(&symbol.green))
                    .map(|func_type| extract_sig(db, func_type))
            }
        })
        .unwrap_or_default()
}

pub(crate) fn get_type_use_sig<'db>(
    db: &'db dyn salsa::Database,
    document: Document,
    ptr: SyntaxNodePtr,
    type_use: &GreenNodeData,
) -> Signature<'db> {
    if type_use.children().any(|child| {
        let kind = child.kind();
        kind == SyntaxKind::PARAM.into() || kind == SyntaxKind::RESULT.into()
    }) {
        extract_sig(db, type_use.to_owned())
    } else {
        let symbol_table = SymbolTable::of(db, document);
        TypeUse::cast(ptr.to_node(&document.root_tree(db)))
            .and_then(|type_use| type_use.index())
            .and_then(|idx| symbol_table.find_def(SymbolKey::new(idx.syntax())))
            .and_then(|symbol| helpers::ast::find_func_type_of_type_def(&symbol.green))
            .map(|func_type| extract_sig(db, func_type))
            .unwrap_or_default()
    }
}

pub(crate) fn get_block_sig<'db>(
    service: &'db dyn salsa::Database,
    document: Document,
    node: &SyntaxNode,
) -> Signature<'db> {
    support::child::<BlockType>(node)
        .and_then(|block_type| block_type.type_use())
        .map(|type_use| {
            let node = type_use.syntax();
            get_type_use_sig(service, document, SyntaxNodePtr::new(node), &node.green())
        })
        .unwrap_or_else(|| get_func_sig(service, document, SyntaxNodePtr::new(node), &node.green()))
}
