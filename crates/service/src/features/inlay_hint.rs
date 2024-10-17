use crate::{
    binder::{SymbolItemKind, SymbolTablesCtx},
    files::FilesCtx,
    helpers,
    types_analyzer::TypesAnalyzerCtx,
    LanguageService,
};
use lsp_types::{InlayHint, InlayHintKind, InlayHintLabel, InlayHintParams};
use rowan::NodeOrToken;
use wat_syntax::SyntaxKind;

impl LanguageService {
    pub fn inlay_hint(&self, params: InlayHintParams) -> Option<Vec<InlayHint>> {
        let uri = self.ctx.uri(params.text_document.uri);
        let line_index = self.ctx.line_index(uri);
        let range = helpers::lsp_range_to_rowan_range(&line_index, params.range)?;
        let symbol_table = self.ctx.symbol_table(uri);

        let inlay_hints = symbol_table
            .symbols
            .iter()
            .filter(|symbol| range.contains_range(symbol.key.ptr.text_range()))
            .filter_map(|symbol| match &symbol.kind {
                SymbolItemKind::LocalRef(..) => {
                    let param_or_local = symbol_table.find_param_or_local_def(&symbol.key)?;
                    let types = self.ctx.extract_types(param_or_local.key.green.clone());
                    Some(InlayHint {
                        position: helpers::rowan_pos_to_lsp_pos(
                            &line_index,
                            symbol.key.ptr.text_range().end(),
                        ),
                        label: InlayHintLabel::String(types.first()?.to_string()),
                        kind: Some(InlayHintKind::TYPE),
                        text_edits: None,
                        tooltip: None,
                        padding_left: Some(true),
                        padding_right: None,
                        data: None,
                    })
                }
                SymbolItemKind::GlobalRef(..) => {
                    let global = symbol_table.find_global_defs(&symbol.key)?.next()?;
                    let types = self.ctx.extract_types(
                        global
                            .key
                            .green
                            .children()
                            .find_map(|child| match child {
                                NodeOrToken::Node(node)
                                    if node.kind() == SyntaxKind::GLOBAL_TYPE.into() =>
                                {
                                    Some(node)
                                }
                                _ => None,
                            })?
                            .to_owned(),
                    );
                    Some(InlayHint {
                        position: helpers::rowan_pos_to_lsp_pos(
                            &line_index,
                            symbol.key.ptr.text_range().end(),
                        ),
                        label: InlayHintLabel::String(types.first()?.to_string()),
                        kind: Some(InlayHintKind::TYPE),
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
