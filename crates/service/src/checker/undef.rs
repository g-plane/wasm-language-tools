use crate::{
    binder::{RefIdx, SymbolItemKind, SymbolTable, SymbolTablesCtx},
    helpers, LanguageServiceCtx,
};
use line_index::LineIndex;
use lsp_types::{Diagnostic, DiagnosticSeverity};

pub fn check(
    diags: &mut Vec<Diagnostic>,
    ctx: &LanguageServiceCtx,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
) {
    let diagnostics = symbol_table
        .symbols
        .iter()
        .filter_map(|symbol| match &symbol.kind {
            SymbolItemKind::Module
            | SymbolItemKind::Func(..)
            | SymbolItemKind::Param(..)
            | SymbolItemKind::Local(..)
            | SymbolItemKind::Type(..)
            | SymbolItemKind::GlobalDef(..)
            | SymbolItemKind::MemoryDef(..) => None,
            SymbolItemKind::Call(ref_idx) => {
                if symbol_table.symbols.iter().any(|sym| {
                    if let SymbolItemKind::Func(def_idx) = &sym.kind {
                        def_idx == ref_idx && symbol.region == sym.region
                    } else {
                        false
                    }
                }) {
                    None
                } else {
                    Some((symbol, ref_idx))
                }
            }
            SymbolItemKind::LocalRef(ref_idx) => {
                if symbol_table.symbols.iter().any(|sym| {
                    if let SymbolItemKind::Param(def_idx) | SymbolItemKind::Local(def_idx) =
                        &sym.kind
                    {
                        def_idx == ref_idx && symbol.region == sym.region
                    } else {
                        false
                    }
                }) {
                    None
                } else {
                    Some((symbol, ref_idx))
                }
            }
            SymbolItemKind::TypeUse(ref_idx) => {
                if symbol_table.symbols.iter().any(|sym| {
                    if let SymbolItemKind::Type(def_idx) = &sym.kind {
                        def_idx == ref_idx && symbol.region == sym.region
                    } else {
                        false
                    }
                }) {
                    None
                } else {
                    Some((symbol, ref_idx))
                }
            }
            SymbolItemKind::GlobalRef(ref_idx) => {
                if symbol_table.symbols.iter().any(|sym| {
                    if let SymbolItemKind::GlobalDef(def_idx) = &sym.kind {
                        def_idx == ref_idx && symbol.region == sym.region
                    } else {
                        false
                    }
                }) {
                    None
                } else {
                    Some((symbol, ref_idx))
                }
            }
            SymbolItemKind::MemoryRef(ref_idx) => {
                if symbol_table.symbols.iter().any(|sym| {
                    if let SymbolItemKind::MemoryDef(def_idx) = &sym.kind {
                        def_idx == ref_idx && symbol.region == sym.region
                    } else {
                        false
                    }
                }) {
                    None
                } else {
                    Some((symbol, ref_idx))
                }
            }
        })
        .map(|(symbol, idx)| Diagnostic {
            range: helpers::rowan_range_to_lsp_range(line_index, symbol.key.ptr.text_range()),
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("wat".into()),
            message: format!(
                "cannot find `{}` in this scope",
                match idx {
                    RefIdx::Num(num) => num.to_string(),
                    RefIdx::Name(name) => ctx.lookup_ident(*name),
                }
            ),
            ..Default::default()
        });
    diags.extend(diagnostics);
}
