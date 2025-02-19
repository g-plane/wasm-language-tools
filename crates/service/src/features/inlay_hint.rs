use crate::{
    binder::{SymbolKind, SymbolTablesCtx},
    helpers,
    idx::IdentsCtx,
    syntax_tree::SyntaxTreeCtx,
    types_analyzer::TypesAnalyzerCtx,
    uri::UrisCtx,
    LanguageService,
};
use lspt::{InlayHint, InlayHintKind, InlayHintParams, Union2};
use wat_syntax::{SyntaxKind, SyntaxNode};

impl LanguageService {
    /// Handler for `textDocument/inlayHint` request.
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
                SymbolKind::LocalRef => {
                    if !range.contains_range(symbol.key.text_range()) {
                        return None;
                    }
                    let param_or_local = symbol_table.find_param_or_local_def(symbol.key)?;
                    let ty = self.extract_type(param_or_local.green.clone())?;
                    Some(InlayHint {
                        position: helpers::rowan_pos_to_lsp_pos(
                            &line_index,
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
                SymbolKind::GlobalRef => {
                    if !range.contains_range(symbol.key.text_range()) {
                        return None;
                    }
                    let global = symbol_table.find_def(symbol.key)?;
                    let ty = self.extract_global_type(global.green.clone())?;
                    Some(InlayHint {
                        position: helpers::rowan_pos_to_lsp_pos(
                            &line_index,
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
                SymbolKind::Func => {
                    let func = symbol.key.to_node(&root);
                    func.last_child_or_token()
                        .map(|last| last.text_range())
                        .filter(|last| range.contains_range(*last))
                        .zip(symbol.idx.name)
                        .map(|(last, name)| InlayHint {
                            position: helpers::rowan_pos_to_lsp_pos(&line_index, last.end()),
                            label: Union2::A(format!("(func {})", self.lookup_ident(name))),
                            kind: None,
                            text_edits: None,
                            tooltip: None,
                            padding_left: Some(true),
                            padding_right: None,
                            data: None,
                        })
                }
                SymbolKind::BlockDef => {
                    let block = symbol.key.to_node(&root);
                    block
                        .last_child_or_token()
                        .map(|last| last.text_range())
                        .filter(|last| range.contains_range(*last))
                        .zip(symbol.idx.name)
                        .map(|(last, name)| InlayHint {
                            position: helpers::rowan_pos_to_lsp_pos(&line_index, last.end()),
                            label: Union2::A(format!(
                                "({} {})",
                                match symbol.key.kind() {
                                    SyntaxKind::BLOCK_IF => "if",
                                    SyntaxKind::BLOCK_LOOP => "loop",
                                    _ => "block",
                                },
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
