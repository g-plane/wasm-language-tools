use crate::{
    LanguageService,
    binder::{SymbolKind, SymbolTable},
    helpers::LineIndexExt,
    idx::Idx,
    types_analyzer,
    uri::InternUri,
};
use lspt::{InlayHint, InlayHintKind, InlayHintParams, Union2};
use rowan::ast::support;
use wat_syntax::SyntaxKind;

impl LanguageService {
    /// Handler for `textDocument/inlayHint` request.
    pub fn inlay_hint(&self, params: InlayHintParams) -> Option<Vec<InlayHint>> {
        let uri = InternUri::new(self, params.text_document.uri);
        let document = self.get_document(uri)?;
        // Avoid inlay hints flickering if client supports pulling config and config is not ready.
        // This is similar to what we do in the checker.
        let configs = self.configs.read();
        let config = configs.get(&uri)?.unwrap_or_global(self);
        self.with_db(|db| {
            let line_index = document.line_index(db);
            let root = document.root_tree(db);
            let symbol_table = SymbolTable::of(db, document);
            let options = &config.inlay_hint;

            let range = line_index.convert(params.range)?;
            let mut inlay_hints = Vec::new();
            for symbol in symbol_table.symbols.values() {
                match symbol.kind {
                    SymbolKind::LocalRef => {
                        if options.types
                            && range.contains_range(symbol.key.text_range())
                            && let Some(ty) = symbol_table
                                .find_def(symbol.key)
                                .and_then(|local| types_analyzer::extract_type(db, document, local.green.clone()))
                        {
                            inlay_hints.push(InlayHint {
                                position: line_index.convert(symbol.key.text_range().end()),
                                label: Union2::A(ty.render(db).to_string()),
                                kind: Some(InlayHintKind::Type),
                                text_edits: None,
                                tooltip: None,
                                padding_left: Some(true),
                                padding_right: None,
                                data: None,
                            });
                        }
                    }
                    SymbolKind::GlobalRef => {
                        if options.types
                            && range.contains_range(symbol.key.text_range())
                            && let Some(ty) = symbol_table.find_def(symbol.key).and_then(|global| {
                                types_analyzer::extract_global_type(db, document, global.green.clone())
                            })
                        {
                            inlay_hints.push(InlayHint {
                                position: line_index.convert(symbol.key.text_range().end()),
                                label: Union2::A(ty.render(db).to_string()),
                                kind: Some(InlayHintKind::Type),
                                text_edits: None,
                                tooltip: None,
                                padding_left: Some(true),
                                padding_right: None,
                                data: None,
                            });
                        }
                    }
                    SymbolKind::Func => {
                        if options.ending
                            && let Some((last, name)) = symbol
                                .key
                                .to_node(&root)
                                .last_child_or_token()
                                .map(|last| last.text_range())
                                .filter(|last| range.contains_range(*last))
                                .zip(symbol.idx.name)
                        {
                            inlay_hints.push(InlayHint {
                                position: line_index.convert(last.end()),
                                label: Union2::A(format!("(func {})", name.ident(db))),
                                kind: None,
                                text_edits: None,
                                tooltip: None,
                                padding_left: Some(true),
                                padding_right: None,
                                data: None,
                            });
                        }
                        if options.index
                            && let Idx {
                                num: Some(num),
                                name: None,
                            } = symbol.idx
                            && let Some(range) = symbol_table.def_poi.get(&symbol.key)
                        {
                            inlay_hints.push(InlayHint {
                                position: line_index.convert(range.end()),
                                label: Union2::A(format!("(;{num};)")),
                                kind: None,
                                text_edits: None,
                                tooltip: None,
                                padding_left: Some(true),
                                padding_right: None,
                                data: None,
                            });
                        }
                    }
                    SymbolKind::BlockDef => {
                        if options.ending
                            && let Some((last, name)) = symbol
                                .key
                                .to_node(&root)
                                .last_child_or_token()
                                .map(|last| last.text_range())
                                .filter(|last| range.contains_range(*last))
                                .zip(symbol.idx.name)
                        {
                            inlay_hints.push(InlayHint {
                                position: line_index.convert(last.end()),
                                label: Union2::A(format!(
                                    "({} {})",
                                    match symbol.key.kind() {
                                        SyntaxKind::BLOCK_IF => "if",
                                        SyntaxKind::BLOCK_LOOP => "loop",
                                        SyntaxKind::BLOCK_TRY_TABLE => "try_table",
                                        _ => "block",
                                    },
                                    name.ident(db),
                                )),
                                kind: None,
                                text_edits: None,
                                tooltip: None,
                                padding_left: Some(true),
                                padding_right: None,
                                data: None,
                            });
                        }
                    }
                    SymbolKind::Type
                    | SymbolKind::GlobalDef
                    | SymbolKind::MemoryDef
                    | SymbolKind::TableDef
                    | SymbolKind::TagDef => {
                        if options.index
                            && let Idx {
                                num: Some(num),
                                name: None,
                            } = symbol.idx
                            && let Some(range) = symbol_table.def_poi.get(&symbol.key)
                        {
                            inlay_hints.push(InlayHint {
                                position: line_index.convert(range.end()),
                                label: Union2::A(format!("(;{num};)")),
                                kind: None,
                                text_edits: None,
                                tooltip: None,
                                padding_left: Some(true),
                                padding_right: None,
                                data: None,
                            });
                        }
                    }
                    SymbolKind::Param | SymbolKind::Local | SymbolKind::FieldDef => {
                        if options.index
                            && let Idx {
                                num: Some(num),
                                name: None,
                            } = symbol.idx
                        {
                            let end = support::token(&symbol.key.to_node(&root), SyntaxKind::KEYWORD)
                                .filter(|token| matches!(token.text(), "param" | "local" | "field"))
                                .map(|token| token.text_range().end())
                                .unwrap_or_else(|| symbol.key.text_range().end());
                            inlay_hints.push(InlayHint {
                                position: line_index.convert(end),
                                label: Union2::A(format!("(;{num};)")),
                                kind: None,
                                text_edits: None,
                                tooltip: None,
                                padding_left: Some(true),
                                padding_right: None,
                                data: None,
                            });
                        }
                    }
                    SymbolKind::FieldRef => {
                        if options.types
                            && range.contains_range(symbol.key.text_range())
                            && let Some(ty) =
                                types_analyzer::resolve_field_type(db, document, symbol.key, symbol.region)
                        {
                            inlay_hints.push(InlayHint {
                                position: line_index.convert(symbol.key.text_range().end()),
                                label: Union2::A(ty.render(db).to_string()),
                                kind: Some(InlayHintKind::Type),
                                text_edits: None,
                                tooltip: None,
                                padding_left: Some(true),
                                padding_right: None,
                                data: None,
                            });
                        }
                    }
                    _ => {}
                }
            }
            Some(inlay_hints)
        })
        .flatten()
    }
}
