use super::{
    types::{OperandType, ValType},
    TypesAnalyzerCtx,
};
use crate::{binder::SymbolKey, helpers, idx::InternIdent, uri::InternUri, LanguageService};
use rowan::{
    ast::{support, AstNode},
    GreenNode, NodeOrToken,
};
use wat_syntax::{
    ast::{BlockType, TypeUse},
    SyntaxKind, SyntaxNode, SyntaxNodePtr,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) struct Signature {
    pub(crate) params: Vec<(ValType, Option<InternIdent>)>,
    pub(crate) results: Vec<ValType>,
}
impl Signature {
    pub(crate) fn matches(
        &self,
        other: &Self,
        db: &dyn TypesAnalyzerCtx,
        uri: InternUri,
        module_id: u32,
    ) -> bool {
        self.params.len() == other.params.len()
            && self.results.len() == other.results.len()
            && other
                .params
                .iter()
                .zip(&self.params)
                .all(|((a, _), (b, _))| a.matches(b, db, uri, module_id))
            && self
                .results
                .iter()
                .zip(&other.results)
                .all(|(a, b)| a.matches(b, db, uri, module_id))
    }

    pub(crate) fn type_equals(
        &self,
        other: &Self,
        db: &dyn TypesAnalyzerCtx,
        uri: InternUri,
        module_id: u32,
    ) -> bool {
        self.params.len() == other.params.len()
            && self.results.len() == other.results.len()
            && self
                .params
                .iter()
                .zip(&other.params)
                .all(|((a, _), (b, _))| a.type_equals(b, db, uri, module_id))
            && self
                .results
                .iter()
                .zip(&other.results)
                .all(|(a, b)| a.type_equals(b, db, uri, module_id))
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ResolvedSig {
    pub(crate) params: Vec<OperandType>,
    pub(crate) results: Vec<OperandType>,
}

impl From<Signature> for ResolvedSig {
    fn from(sig: Signature) -> Self {
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

pub(super) fn get_func_sig(
    db: &dyn TypesAnalyzerCtx,
    uri: InternUri,
    ptr: SyntaxNodePtr,
    green: GreenNode,
) -> Option<Signature> {
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
                Some(db.extract_sig(type_use.to_owned()))
            } else {
                let node = ptr.to_node(&SyntaxNode::new_root(db.root(uri)));
                let symbol_table = db.symbol_table(uri);
                support::child::<TypeUse>(&node)
                    .and_then(|type_use| type_use.index())
                    .and_then(|idx| symbol_table.find_def(SymbolKey::new(idx.syntax())))
                    .and_then(|symbol| helpers::ast::find_func_type_of_type_def(&symbol.green))
                    .map(|func_type| db.extract_sig(func_type))
            }
        })
}

pub(super) fn get_type_use_sig(
    db: &dyn TypesAnalyzerCtx,
    uri: InternUri,
    ptr: SyntaxNodePtr,
    type_use: GreenNode,
) -> Option<Signature> {
    if type_use.children().any(|child| {
        let kind = child.kind();
        kind == SyntaxKind::PARAM.into() || kind == SyntaxKind::RESULT.into()
    }) {
        Some(db.extract_sig(type_use.to_owned()))
    } else {
        let symbol_table = db.symbol_table(uri);
        TypeUse::cast(ptr.to_node(&SyntaxNode::new_root(db.root(uri))))
            .and_then(|type_use| type_use.index())
            .and_then(|idx| symbol_table.find_def(SymbolKey::new(idx.syntax())))
            .and_then(|symbol| helpers::ast::find_func_type_of_type_def(&symbol.green))
            .map(|func_type| db.extract_sig(func_type))
    }
}

// The reason why we don't put this function to Salsa is because
// the block node comes with block body and can be huge.
// Once the body changed (even block type is unchanged), memoization will be skipped.
// Also, Salsa requires the ownership of GreenNode,
// which means we must clone the whole huge block green node.
pub(crate) fn get_block_sig(
    service: &LanguageService,
    uri: InternUri,
    node: &SyntaxNode,
) -> Option<Signature> {
    support::child::<BlockType>(node)
        .and_then(|block_type| block_type.type_use())
        .and_then(|type_use| {
            let node = type_use.syntax();
            service.get_type_use_sig(uri, SyntaxNodePtr::new(node), node.green().into())
        })
}
