use super::Diagnostic;
use crate::binder::{SymbolKind, SymbolTable};

const DIAGNOSTIC_CODE: &str = "undef";

pub fn check(db: &dyn salsa::Database, diagnostics: &mut Vec<Diagnostic>, symbol_table: &SymbolTable) {
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
                | SymbolKind::FieldDef
                | SymbolKind::TagDef
                | SymbolKind::DataDef
                | SymbolKind::ElemDef => false,
                SymbolKind::Call
                | SymbolKind::LocalRef
                | SymbolKind::TypeUse
                | SymbolKind::GlobalRef
                | SymbolKind::MemoryRef
                | SymbolKind::TableRef
                | SymbolKind::FieldRef
                | SymbolKind::BlockRef
                | SymbolKind::TagRef
                | SymbolKind::DataRef
                | SymbolKind::ElemRef => !symbol_table.resolved.contains_key(&symbol.key),
            })
            .map(|symbol| Diagnostic {
                range: symbol.key.text_range(),
                code: DIAGNOSTIC_CODE.into(),
                message: format!("cannot find {} `{}` in this scope", symbol.kind, symbol.idx.render(db)),
                ..Default::default()
            }),
    );
}
