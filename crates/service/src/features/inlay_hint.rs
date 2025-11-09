use crate::{
    LanguageService,
    binder::{SymbolKind, SymbolTable},
    helpers, types_analyzer,
};
use lspt::{InlayHint, InlayHintKind, InlayHintParams, Union2};
use rowan::ast::support;
use wat_syntax::SyntaxKind;

impl LanguageService {
    /// Handler for `textDocument/inlayHint` request.
    pub fn inlay_hint(&self, params: InlayHintParams) -> Option<Vec<InlayHint>> {
        let document = self.get_document(params.text_document.uri)?;
        // Avoid inlay hints flickering if client supports pulling config and config is not ready.
        // This is similar to what we do in the checker.
        let config_state = self.configs.get(&document.uri(self))?;
        let config = config_state.get_or_global(self);

        let line_index = document.line_index(self);
        let root = document.root_tree(self);
        let symbol_table = SymbolTable::of(self, document);
        let options = &config.inlay_hint;

        let range = helpers::lsp_range_to_rowan_range(line_index, params.range)?;
        let mut inlay_hints = Vec::new();
        for symbol in symbol_table.symbols.values() {
            match symbol.kind {
                SymbolKind::LocalRef => {
                    if options.types
                        && range.contains_range(symbol.key.text_range())
                        && let Some(ty) = symbol_table.find_def(symbol.key).and_then(|local| {
                            types_analyzer::extract_type(self, document, local.green.clone())
                        })
                    {
                        inlay_hints.push(InlayHint {
                            position: helpers::rowan_pos_to_lsp_pos(
                                line_index,
                                symbol.key.text_range().end(),
                            ),
                            label: Union2::A(ty.render(self).to_string()),
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
                            types_analyzer::extract_global_type(
                                self,
                                document,
                                global.green.clone(),
                            )
                        })
                    {
                        inlay_hints.push(InlayHint {
                            position: helpers::rowan_pos_to_lsp_pos(
                                line_index,
                                symbol.key.text_range().end(),
                            ),
                            label: Union2::A(ty.render(self).to_string()),
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
                            position: helpers::rowan_pos_to_lsp_pos(line_index, last.end()),
                            label: Union2::A(format!("(func {})", name.ident(self))),
                            kind: None,
                            text_edits: None,
                            tooltip: None,
                            padding_left: Some(true),
                            padding_right: None,
                            data: None,
                        });
                    }
                    if options.index
                        && let Some(num) = symbol.idx.num
                        && let Some(keyword) =
                            support::token(&symbol.key.to_node(&root), SyntaxKind::KEYWORD)
                    {
                        inlay_hints.push(InlayHint {
                            position: helpers::rowan_pos_to_lsp_pos(
                                line_index,
                                keyword.text_range().end(),
                            ),
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
                            position: helpers::rowan_pos_to_lsp_pos(line_index, last.end()),
                            label: Union2::A(format!(
                                "({} {})",
                                match symbol.key.kind() {
                                    SyntaxKind::BLOCK_IF => "if",
                                    SyntaxKind::BLOCK_LOOP => "loop",
                                    SyntaxKind::BLOCK_TRY_TABLE => "try_table",
                                    _ => "block",
                                },
                                name.ident(self),
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
                        && let Some(num) = symbol.idx.num
                        && let Some(keyword) =
                            support::token(&symbol.key.to_node(&root), SyntaxKind::KEYWORD)
                    {
                        inlay_hints.push(InlayHint {
                            position: helpers::rowan_pos_to_lsp_pos(
                                line_index,
                                keyword.text_range().end(),
                            ),
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
                        && let Some(num) = symbol.idx.num
                    {
                        let end = support::token(&symbol.key.to_node(&root), SyntaxKind::KEYWORD)
                            .filter(|token| matches!(token.text(), "param" | "local" | "field"))
                            .map(|token| token.text_range().end())
                            .unwrap_or_else(|| symbol.key.text_range().end());
                        inlay_hints.push(InlayHint {
                            position: helpers::rowan_pos_to_lsp_pos(line_index, end),
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
                        && let Some(ty) = types_analyzer::resolve_field_type(
                            self,
                            document,
                            symbol.key,
                            symbol.region,
                        )
                    {
                        inlay_hints.push(InlayHint {
                            position: helpers::rowan_pos_to_lsp_pos(
                                line_index,
                                symbol.key.text_range().end(),
                            ),
                            label: Union2::A(ty.render(self).to_string()),
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
    }
}
