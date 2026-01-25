use crate::{
    LanguageService,
    binder::{IdxKind, SymbolKind, SymbolTable},
    helpers::{self, LineIndexExt},
};
use lspt::{CodeLens, CodeLensParams, Command};
use serde::{Deserialize, Serialize};

impl LanguageService {
    /// Handler for `textDocument/codeLens` request.
    pub fn code_lens(&self, params: CodeLensParams) -> Option<Vec<CodeLens>> {
        let document = self.get_document(&params.text_document.uri)?;
        self.with_db(|db| {
            let line_index = document.line_index(db);
            let symbol_table = SymbolTable::of(db, document);
            symbol_table
                .symbols
                .values()
                .filter(|symbol| {
                    matches!(
                        symbol.kind,
                        SymbolKind::Func
                            | SymbolKind::Type
                            | SymbolKind::GlobalDef
                            | SymbolKind::MemoryDef
                            | SymbolKind::TableDef
                            | SymbolKind::TagDef
                    )
                })
                .map(|symbol| CodeLens {
                    range: line_index.convert(symbol.key.text_range()),
                    command: None,
                    data: serde_json::to_value(CodeLensData {
                        uri: params.text_document.uri.clone(),
                        kind: symbol.idx_kind,
                    })
                    .ok(),
                })
                .collect()
        })
    }

    /// Handler for `codeLens/resolve` request.
    pub fn code_lens_resolve(&self, params: CodeLens) -> CodeLens {
        self.code_lens_resolve_impl(params.clone()).unwrap_or(params)
    }

    fn code_lens_resolve_impl(&self, params: CodeLens) -> Option<CodeLens> {
        let data = serde_json::from_value::<CodeLensData>(params.data?).ok()?;
        let document = self.get_document(&data.uri)?;
        let line_index = document.line_index(self);
        let root = document.root_tree(self);
        let symbol_table = SymbolTable::of(self, document);

        let range = line_index.convert(params.range)?;
        let def_symbol = symbol_table
            .symbols
            .values()
            .find(|symbol| symbol.idx_kind == data.kind && symbol.key.text_range() == range)?;
        let locations = symbol_table
            .find_references_on_def(def_symbol, false)
            .map(|symbol| helpers::create_location_by_symbol(data.uri.clone(), line_index, symbol.key, &root))
            .collect::<Vec<_>>();
        Some(CodeLens {
            range: params.range,
            command: Some(Command {
                title: if locations.len() == 1 {
                    "1 reference".into()
                } else {
                    format!("{} references", locations.len())
                },
                command: "wasmLanguageTools.showReferences".into(),
                arguments: Some(vec![
                    serde_json::to_value(&data.uri).ok()?,
                    serde_json::to_value(params.range.start).ok()?,
                    serde_json::to_value(locations).ok()?,
                ]),
            }),
            data: None,
        })
    }
}

#[derive(Serialize, Deserialize)]
struct CodeLensData {
    uri: String,
    kind: IdxKind,
}
