use crate::{
    binder::{SymbolKind, SymbolTable},
    config::LintLevel,
    helpers, LanguageService,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, DiagnosticTag, Union2};
use rowan::ast::{support, AstNode};
use wat_syntax::{
    ast::{GlobalType, ModuleFieldGlobal, PlainInstr},
    SyntaxKind, SyntaxNode,
};

const DIAGNOSTIC_CODE: &str = "needless-mut";

pub fn check(
    service: &LanguageService,
    diagnostics: &mut Vec<Diagnostic>,
    lint_level: LintLevel,
    line_index: &LineIndex,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
) {
    let severity = match lint_level {
        LintLevel::Allow => return,
        LintLevel::Hint => DiagnosticSeverity::Hint,
        LintLevel::Warn => DiagnosticSeverity::Warning,
        LintLevel::Deny => DiagnosticSeverity::Error,
    };

    let mut mutable_globals = symbol_table
        .symbols
        .iter()
        .filter_map(|symbol| {
            if symbol.kind == SymbolKind::GlobalDef {
                let node = symbol.key.to_node(root);
                let global_type = support::child::<GlobalType>(&node);
                if ModuleFieldGlobal::cast(node)
                    .and_then(|global| global.export())
                    .is_some()
                {
                    None
                } else {
                    global_type
                        .and_then(|global_type| global_type.mut_keyword())
                        .map(|keyword| (symbol, keyword, /* mutated */ false))
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    symbol_table
        .symbols
        .iter()
        .filter(|symbol| symbol.kind == SymbolKind::GlobalRef)
        .for_each(|symbol| {
            let parent = symbol.key.to_node(root).parent();
            if parent
                .as_ref()
                .is_some_and(|parent| parent.kind() == SyntaxKind::EXPORT_DESC_GLOBAL)
                || parent
                    .and_then(PlainInstr::cast)
                    .and_then(|instr| instr.instr_name())
                    .is_some_and(|name| name.text() == "global.set")
            {
                if let Some((.., mutated)) = mutable_globals.iter_mut().find(|(def_symbol, ..)| {
                    symbol.region == def_symbol.region && symbol.idx.is_defined_by(&def_symbol.idx)
                }) {
                    *mutated = true;
                };
            }
        });
    diagnostics.extend(
        mutable_globals
            .into_iter()
            .filter(|(.., mutated)| !mutated)
            .map(|(symbol, keyword, _)| Diagnostic {
                range: helpers::rowan_range_to_lsp_range(line_index, keyword.text_range()),
                severity: Some(severity),
                source: Some("wat".into()),
                code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                message: format!("`{}` is unnecessarily mutable", symbol.idx.render(service)),
                tags: Some(vec![DiagnosticTag::Unnecessary]),
                ..Default::default()
            }),
    );
}
