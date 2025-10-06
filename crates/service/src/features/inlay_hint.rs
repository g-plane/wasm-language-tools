use crate::{
    LanguageService,
    binder::{SymbolKind, SymbolTable},
    helpers, types_analyzer,
};
use lspt::{InlayHint, InlayHintKind, InlayHintParams, Union2};
use wat_syntax::SyntaxKind;

impl LanguageService {
    /// Handler for `textDocument/inlayHint` request.
    pub fn inlay_hint(&self, params: InlayHintParams) -> Option<Vec<InlayHint>> {
        let document = self.get_document(params.text_document.uri)?;
        let line_index = document.line_index(self);
        let root = document.root_tree(self);
        let symbol_table = SymbolTable::of(self, document);
        let config = &self.get_config(document).inlay_hint;

        let range = helpers::lsp_range_to_rowan_range(line_index, params.range)?;
        let inlay_hints = symbol_table
            .symbols
            .values()
            .filter_map(|symbol| match symbol.kind {
                SymbolKind::LocalRef if config.types => {
                    if !range.contains_range(symbol.key.text_range()) {
                        return None;
                    }
                    let param_or_local = symbol_table.find_def(symbol.key)?;
                    let ty =
                        types_analyzer::extract_type(self, document, param_or_local.green.clone())?;
                    Some(InlayHint {
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
                    })
                }
                SymbolKind::GlobalRef if config.types => {
                    if !range.contains_range(symbol.key.text_range()) {
                        return None;
                    }
                    let global = symbol_table.find_def(symbol.key)?;
                    let ty =
                        types_analyzer::extract_global_type(self, document, global.green.clone())?;
                    Some(InlayHint {
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
                    })
                }
                SymbolKind::Func if config.ending => {
                    let func = symbol.key.to_node(&root);
                    func.last_child_or_token()
                        .map(|last| last.text_range())
                        .filter(|last| range.contains_range(*last))
                        .zip(symbol.idx.name)
                        .map(|(last, name)| InlayHint {
                            position: helpers::rowan_pos_to_lsp_pos(line_index, last.end()),
                            label: Union2::A(format!("(func {})", name.ident(self))),
                            kind: None,
                            text_edits: None,
                            tooltip: None,
                            padding_left: Some(true),
                            padding_right: None,
                            data: None,
                        })
                }
                SymbolKind::BlockDef if config.ending => {
                    let block = symbol.key.to_node(&root);
                    block
                        .last_child_or_token()
                        .map(|last| last.text_range())
                        .filter(|last| range.contains_range(*last))
                        .zip(symbol.idx.name)
                        .map(|(last, name)| InlayHint {
                            position: helpers::rowan_pos_to_lsp_pos(line_index, last.end()),
                            label: Union2::A(format!(
                                "({} {})",
                                match symbol.key.kind() {
                                    SyntaxKind::BLOCK_IF => "if",
                                    SyntaxKind::BLOCK_LOOP => "loop",
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
                        })
                }
                SymbolKind::FieldRef if config.types => {
                    if !range.contains_range(symbol.key.text_range()) {
                        return None;
                    }
                    types_analyzer::resolve_field_type(self, document, symbol.key, symbol.region)
                        .map(|ty| InlayHint {
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
                        })
                }
                _ => None,
            })
            .collect();
        Some(inlay_hints)
    }
}
