use crate::{
    binder::{SymbolItemKind, SymbolTable},
    helpers,
    idx::IdentsCtx,
    LanguageService,
};
use line_index::LineIndex;
use lsp_types::{Diagnostic, DiagnosticSeverity};

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
            SymbolItemKind::Module
            | SymbolItemKind::Func
            | SymbolItemKind::Param
            | SymbolItemKind::Local
            | SymbolItemKind::Type
            | SymbolItemKind::GlobalDef
            | SymbolItemKind::MemoryDef
            | SymbolItemKind::TableDef
            | SymbolItemKind::BlockDef => false,
            SymbolItemKind::Call
            | SymbolItemKind::TypeUse
            | SymbolItemKind::GlobalRef
            | SymbolItemKind::MemoryRef
            | SymbolItemKind::TableRef => symbol_table
                .find_defs(&symbol.key)
                .is_none_or(|defs| defs.count() == 0),
            SymbolItemKind::LocalRef => symbol_table.find_param_or_local_def(&symbol.key).is_none(),
            SymbolItemKind::BlockRef => !symbol_table.blocks.iter().any(|block| {
                block.ref_key == symbol.key && symbol.idx.is_defined_by(&block.def_idx)
            }),
        })
        .map(|symbol| Diagnostic {
            range: helpers::rowan_range_to_lsp_range(line_index, symbol.key.ptr.text_range()),
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("wat".into()),
            message: format!(
                "cannot find `{}` in this scope",
                symbol
                    .idx
                    .name
                    .map(|name| service.lookup_ident(name))
                    .or_else(|| symbol.idx.num.map(|num| num.to_string()))
                    .unwrap_or_default()
            ),
            ..Default::default()
        });
    diags.extend(diagnostics);
}
