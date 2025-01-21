use crate::{
    binder::{SymbolKind, SymbolTable},
    helpers, LanguageService,
};
use line_index::LineIndex;
use lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString};

const DIAGNOSTIC_CODE: &str = "undef";

pub fn check(
    service: &LanguageService,
    diags: &mut Vec<Diagnostic>,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
) {
    let diagnostics = symbol_table
        .symbols
        .iter()
        .filter(|symbol| match &symbol.kind {
            SymbolKind::Module
            | SymbolKind::Func
            | SymbolKind::Param
            | SymbolKind::Local
            | SymbolKind::Type
            | SymbolKind::GlobalDef
            | SymbolKind::MemoryDef
            | SymbolKind::TableDef
            | SymbolKind::BlockDef => false,
            SymbolKind::Call
            | SymbolKind::TypeUse
            | SymbolKind::GlobalRef
            | SymbolKind::MemoryRef
            | SymbolKind::TableRef => symbol_table
                .find_defs(symbol.key)
                .is_none_or(|defs| defs.count() == 0),
            SymbolKind::LocalRef => symbol_table.find_param_or_local_def(symbol.key).is_none(),
            SymbolKind::BlockRef => !symbol_table.blocks.iter().any(|block| {
                block.ref_key == symbol.key && symbol.idx.is_defined_by(&block.def_idx)
            }),
        })
        .map(|symbol| Diagnostic {
            range: helpers::rowan_range_to_lsp_range(line_index, symbol.key.text_range()),
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("wat".into()),
            code: Some(NumberOrString::String(DIAGNOSTIC_CODE.into())),
            message: format!("cannot find `{}` in this scope", symbol.idx.render(service)),
            ..Default::default()
        });
    diags.extend(diagnostics);
}
