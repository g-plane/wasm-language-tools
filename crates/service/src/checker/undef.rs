use crate::{
    LanguageService,
    binder::{SymbolKind, SymbolTable},
    helpers,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};

const DIAGNOSTIC_CODE: &str = "undef";

pub fn check(
    service: &LanguageService,
    diagnostics: &mut Vec<Diagnostic>,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
) {
    diagnostics.extend(
        symbol_table
            .symbols
            .values()
            .filter(|symbol| match symbol.kind {
                SymbolKind::Module
                | SymbolKind::Func
                | SymbolKind::Param
                | SymbolKind::Local
                | SymbolKind::Type
                | SymbolKind::GlobalDef
                | SymbolKind::MemoryDef
                | SymbolKind::TableDef
                | SymbolKind::BlockDef
                | SymbolKind::FieldDef => false,
                SymbolKind::Call
                | SymbolKind::TypeUse
                | SymbolKind::GlobalRef
                | SymbolKind::MemoryRef
                | SymbolKind::TableRef
                | SymbolKind::FieldRef => symbol_table.find_def(symbol.key).is_none(),
                SymbolKind::LocalRef => symbol_table.find_param_or_local_def(symbol.key).is_none(),
                SymbolKind::BlockRef => !symbol_table.blocks.iter().any(|block| {
                    block.ref_key == symbol.key && symbol.idx.is_defined_by(&block.def_idx)
                }),
            })
            .map(|symbol| {
                let kind = match symbol.kind {
                    SymbolKind::Call => "func",
                    SymbolKind::LocalRef => "param or local",
                    SymbolKind::TypeUse => "type",
                    SymbolKind::GlobalRef => "global",
                    SymbolKind::MemoryRef => "memory",
                    SymbolKind::TableRef => "table",
                    SymbolKind::BlockRef => "label",
                    SymbolKind::FieldRef => "field",
                    _ => unreachable!(),
                };
                Diagnostic {
                    range: helpers::rowan_range_to_lsp_range(line_index, symbol.key.text_range()),
                    severity: Some(DiagnosticSeverity::Error),
                    source: Some("wat".into()),
                    code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                    message: format!(
                        "cannot find {kind} `{}` in this scope",
                        symbol.idx.render(service)
                    ),
                    ..Default::default()
                }
            }),
    );
}
