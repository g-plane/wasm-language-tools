use self::{
    extractor::{extract_global_type, extract_sig, extract_type},
    renderer::{render_block_header, render_compact_sig, render_func_header, render_sig},
    signature::{get_func_sig, get_type_use_sig, Signature},
};
pub(crate) use self::{
    resolver::{resolve_br_types, resolve_param_types},
    signature::{get_block_sig, ResolvedSig},
    types::{HeapType, OperandType, RefType, ValType},
};
use crate::{
    binder::{Symbol, SymbolTablesCtx},
    idx::InternIdent,
    syntax_tree::SyntaxTreeCtx,
    uri::InternUri,
};
use rowan::GreenNode;
use std::{
    fmt::{self, Debug},
    hash::Hash,
    ops::Deref,
};
use wat_syntax::{SyntaxKind, SyntaxNodePtr};

mod extractor;
mod renderer;
mod resolver;
mod signature;
mod types;

#[salsa::query_group(TypesAnalyzer)]
pub(crate) trait TypesAnalyzerCtx: SyntaxTreeCtx + SymbolTablesCtx {
    #[salsa::memoized]
    fn extract_type(&self, node: GreenNode) -> Option<ValType>;
    #[salsa::memoized]
    fn extract_global_type(&self, node: GreenNode) -> Option<ValType>;
    #[salsa::memoized]
    fn extract_sig(&self, node: GreenNode) -> Signature;

    #[salsa::memoized]
    fn get_func_sig(
        &self,
        uri: InternUri,
        ptr: SyntaxNodePtr,
        green: GreenNode,
    ) -> Option<Signature>;
    #[salsa::memoized]
    fn get_type_use_sig(
        &self,
        uri: InternUri,
        ptr: SyntaxNodePtr,
        type_use: GreenNode,
    ) -> Option<Signature>;
    #[salsa::memoized]
    fn render_sig(&self, signature: Signature) -> String;
    #[salsa::memoized]
    fn render_compact_sig(&self, signature: Signature) -> String;
    #[salsa::memoized]
    fn render_func_header(&self, name: Option<InternIdent>, signature: Option<Signature>)
        -> String;
    #[salsa::memoized]
    fn render_block_header(
        &self,
        kind: SyntaxKind,
        name: Option<InternIdent>,
        signature: Option<Signature>,
    ) -> String;
}

#[derive(Clone)]
pub(crate) struct SymbolWithGreenEq(Symbol);
impl PartialEq for SymbolWithGreenEq {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.0.green == other.0.green
    }
}
impl Eq for SymbolWithGreenEq {}
impl Hash for SymbolWithGreenEq {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.0.green.hash(state);
    }
}
impl From<Symbol> for SymbolWithGreenEq {
    fn from(symbol: Symbol) -> Self {
        SymbolWithGreenEq(symbol)
    }
}
impl Deref for SymbolWithGreenEq {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Debug for SymbolWithGreenEq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
