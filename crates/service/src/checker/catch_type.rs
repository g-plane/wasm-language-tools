use crate::{
    LanguageService,
    binder::{SymbolKey, SymbolTable},
    document::Document,
    helpers,
    types_analyzer::{self, HeapType, RefType, ValType},
};
use itertools::Itertools;
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use rowan::ast::AstNode;
use wat_syntax::{SyntaxNode, ast::Cat};

const DIAGNOSTIC_CODE: &str = "catch-type";

pub fn check(
    service: &LanguageService,
    document: Document,
    line_index: &LineIndex,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
    module_id: u32,
    node: SyntaxNode,
) -> Option<Diagnostic> {
    let (label_index, results) = match Cat::cast(node)? {
        Cat::Catch(catch) => {
            let tag = symbol_table.find_def(SymbolKey::new(catch.tag_index()?.syntax()))?;
            let mut results = types_analyzer::get_func_sig(service, document, *tag.key, &tag.green)
                .params
                .into_iter()
                .map(|(ty, _)| ty)
                .collect::<Vec<_>>();
            if catch.keyword()?.text() == "catch_ref" {
                results.push(ValType::Ref(RefType {
                    heap_ty: HeapType::Exn,
                    nullable: false,
                }));
            }
            (catch.label_index()?, results)
        }
        Cat::CatchAll(catch_all) => {
            let results = match catch_all.keyword()?.text() {
                "catch_all" => vec![],
                "catch_all_ref" => vec![ValType::Ref(RefType {
                    heap_ty: HeapType::Exn,
                    nullable: false,
                })],
                _ => unreachable!(),
            };
            (catch_all.label_index()?, results)
        }
    };
    let ref_key = SymbolKey::new(label_index.syntax());
    let block = symbol_table.find_def(ref_key)?;
    let block_sig = types_analyzer::get_block_sig(service, document, &block.key.to_node(root));
    if results.len() != block_sig.results.len()
        || !results
            .iter()
            .zip(block_sig.results.iter())
            .all(|(a, b)| a.matches(b, service, document, module_id))
    {
        Some(Diagnostic {
            range: helpers::rowan_range_to_lsp_range(line_index, label_index.syntax().text_range()),
            severity: Some(DiagnosticSeverity::Error),
            source: Some("wat".into()),
            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
            message: format!(
                "result type [{}] should match result type of block `{}`",
                results.iter().map(|ty| ty.render(service)).join(", "),
                symbol_table.symbols.get(&ref_key)?.idx.render(service),
            ),
            ..Default::default()
        })
    } else {
        None
    }
}
