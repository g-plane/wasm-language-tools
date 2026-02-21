use super::{Diagnostic, DiagnosticCtx};
use crate::{
    binder::SymbolKey,
    types_analyzer::{self, HeapType, RefType, ValType},
};
use itertools::Itertools;
use wat_syntax::{AmberNode, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "catch-type";

pub fn check(ctx: &DiagnosticCtx, node: AmberNode) -> Option<Diagnostic> {
    let (label_index, results) = match node.kind() {
        SyntaxKind::CATCH => {
            let mut indexes = node.children_by_kind(SyntaxKind::INDEX);
            let tag = ctx.symbol_table.find_def(indexes.next()?.to_ptr().into())?;
            let mut results = types_analyzer::get_func_sig(ctx.db, ctx.document, tag.key, &tag.green)
                .params
                .into_iter()
                .map(|(ty, _)| ty)
                .collect::<Vec<_>>();
            if node.tokens_by_kind(SyntaxKind::KEYWORD).next()?.text() == "catch_ref" {
                results.push(ValType::Ref(RefType {
                    heap_ty: HeapType::Exn,
                    nullable: false,
                }));
            }
            (indexes.next()?, results)
        }
        SyntaxKind::CATCH_ALL => {
            let results = match node.tokens_by_kind(SyntaxKind::KEYWORD).next()?.text() {
                "catch_all" => vec![],
                "catch_all_ref" => vec![ValType::Ref(RefType {
                    heap_ty: HeapType::Exn,
                    nullable: false,
                })],
                _ => unreachable!(),
            };
            (node.children_by_kind(SyntaxKind::INDEX).next()?, results)
        }
        _ => return None,
    };
    let ref_key = SymbolKey::from(label_index.to_ptr());
    let block = ctx.symbol_table.find_def(ref_key)?;
    let block_sig = types_analyzer::get_func_sig(ctx.db, ctx.document, block.key, &block.green);
    if results.len() != block_sig.results.len()
        || !results
            .iter()
            .zip(block_sig.results.iter())
            .all(|(a, b)| a.matches(b, ctx.db, ctx.document, ctx.module_id))
    {
        Some(Diagnostic {
            range: label_index.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: format!(
                "result type [{}] should match result type of block `{}`",
                results.iter().map(|ty| ty.render(ctx.db)).join(", "),
                ctx.symbol_table.symbols.get(&ref_key)?.idx.render(ctx.db),
            ),
            ..Default::default()
        })
    } else {
        None
    }
}
