use self::{
    def_type::{create_def_types, create_recursive_types, RecTypeGroup},
    extractor::{extract_global_type, extract_sig, extract_type},
    renderer::{render_block_header, render_compact_sig, render_func_header, render_sig},
    signature::{get_func_sig, get_type_use_sig, Signature},
};
pub(crate) use self::{
    def_type::{CompositeType, DefType},
    resolver::{resolve_array_type_with_idx, resolve_br_types, resolve_param_types},
    signature::{get_block_sig, ResolvedSig},
    types::{HeapType, OperandType, RefType, ValType},
};
use crate::{
    binder::SymbolTablesCtx, idx::InternIdent, syntax_tree::SyntaxTreeCtx, uri::InternUri,
};
use rowan::GreenNode;
use std::sync::Arc;
use wat_syntax::{SyntaxKind, SyntaxNodePtr};

mod def_type;
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

    #[salsa::memoized]
    #[salsa::invoke(create_def_types)]
    fn def_types(&self, uri: InternUri) -> Arc<Vec<DefType>>;
    #[salsa::memoized]
    #[salsa::invoke(create_recursive_types)]
    fn rec_type_groups(&self, uri: InternUri) -> Arc<Vec<RecTypeGroup>>;

    #[salsa::memoized]
    fn operand_type_matches(
        &self,
        uri: InternUri,
        module_id: u32,
        sub: OperandType,
        sup: OperandType,
    ) -> bool;
}

fn operand_type_matches(
    db: &dyn TypesAnalyzerCtx,
    uri: InternUri,
    module_id: u32,
    sub: OperandType,
    sup: OperandType,
) -> bool {
    match (sub, sup) {
        (OperandType::Val(sub), OperandType::Val(sup)) => sub.matches(&sup, db, uri, module_id),
        (OperandType::Any, _) | (_, OperandType::Any) => true,
        (OperandType::PackedI8, OperandType::PackedI8) => true,
        (OperandType::PackedI16, OperandType::PackedI16) => true,
        _ => false,
    }
}
