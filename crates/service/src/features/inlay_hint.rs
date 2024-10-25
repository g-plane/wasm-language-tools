use crate::{
    binder::{DefIdx, SymbolItemKind, SymbolTablesCtx},
    files::FilesCtx,
    helpers,
    types_analyzer::TypesAnalyzerCtx,
    LanguageService,
};
use lsp_types::{InlayHint, InlayHintKind, InlayHintLabel, InlayHintParams};
use wat_syntax::SyntaxNode;

impl LanguageService {
    pub fn inlay_hint(&self, params: InlayHintParams) -> Option<Vec<InlayHint>> {
        let uri = self.uri(params.text_document.uri);
        let line_index = self.line_index(uri);
        let root = SyntaxNode::new_root(self.root(uri));
        let symbol_table = self.symbol_table(uri);

        let range = helpers::lsp_range_to_rowan_range(&line_index, params.range)?;
        let inlay_hints = symbol_table
            .symbols
            .iter()
            .filter_map(|symbol| match symbol.kind {
                SymbolItemKind::LocalRef(..) => {
                    if !range.contains_range(symbol.key.ptr.text_range()) {
                        return None;
                    }
                    let param_or_local = symbol_table.find_param_or_local_def(&symbol.key)?;
                    let ty = self.extract_type(param_or_local.green.clone())?;
                    Some(InlayHint {
                        position: helpers::rowan_pos_to_lsp_pos(
                            &line_index,
                            symbol.key.ptr.text_range().end(),
                        ),
                        label: InlayHintLabel::String(ty.to_string()),
                        kind: Some(InlayHintKind::TYPE),
                        text_edits: None,
                        tooltip: None,
                        padding_left: Some(true),
                        padding_right: None,
                        data: None,
                    })
                }
                SymbolItemKind::GlobalRef(..) => {
                    if !range.contains_range(symbol.key.ptr.text_range()) {
                        return None;
                    }
                    let global = symbol_table.find_global_defs(&symbol.key)?.next()?;
                    let ty = self.extract_global_type(global.green.clone())?;
                    Some(InlayHint {
                        position: helpers::rowan_pos_to_lsp_pos(
                            &line_index,
                            symbol.key.ptr.text_range().end(),
                        ),
                        label: InlayHintLabel::String(ty.to_string()),
                        kind: Some(InlayHintKind::TYPE),
                        text_edits: None,
                        tooltip: None,
                        padding_left: Some(true),
                        padding_right: None,
                        data: None,
                    })
                }
                SymbolItemKind::Func(DefIdx {
                    name: Some(name), ..
                }) => {
                    let func = symbol.key.ptr.to_node(&root);
                    func.last_child_or_token()
                        .map(|last| last.text_range())
                        .filter(|last| range.contains_range(*last))
                        .map(|last| InlayHint {
                            position: helpers::rowan_pos_to_lsp_pos(&line_index, last.end()),
                            label: InlayHintLabel::String(format!(
                                "(func {})",
                                self.lookup_ident(name)
                            )),
                            kind: None,
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
