use super::{find_meaningful_token, is_call};
use crate::{
    binder::{SymbolItem, SymbolItemKind, SymbolTablesCtx},
    files::FilesCtx,
    helpers,
    types_analyzer::TypesAnalyzerCtx,
    LanguageService,
};
use line_index::LineIndex;
use lsp_types::{
    CallHierarchyIncomingCall, CallHierarchyIncomingCallsParams, CallHierarchyItem,
    CallHierarchyOutgoingCall, CallHierarchyOutgoingCallsParams, CallHierarchyPrepareParams, Range,
    SymbolKind,
};
use rowan::ast::support::token;
use wat_syntax::{SyntaxKind, SyntaxNode};

impl LanguageService {
    pub fn prepare_call_hierarchy(
        &self,
        params: CallHierarchyPrepareParams,
    ) -> Option<Vec<CallHierarchyItem>> {
        let uri = self.ctx.uri(
            params
                .text_document_position_params
                .text_document
                .uri
                .clone(),
        );
        let line_index = self.ctx.line_index(uri);
        let root = self.build_root(uri);
        let symbol_table = self.ctx.symbol_table(uri);

        let token = find_meaningful_token(
            &self.ctx,
            uri,
            &root,
            params.text_document_position_params.position,
        )?;
        let parent_range = token.parent()?.text_range();

        symbol_table
            .symbols
            .iter()
            .find_map(|symbol| match &symbol.kind {
                SymbolItemKind::Func(idx) if symbol.key.ptr.text_range() == parent_range => {
                    Some(vec![CallHierarchyItem {
                        name: idx
                            .name
                            .map(|name| self.ctx.lookup_ident(name))
                            .unwrap_or_else(|| idx.num.to_string()),
                        kind: SymbolKind::FUNCTION,
                        tags: None,
                        detail: Some(self.ctx.render_func_header(uri, symbol.clone())),
                        uri: params
                            .text_document_position_params
                            .text_document
                            .uri
                            .clone(),
                        range: helpers::rowan_range_to_lsp_range(
                            &line_index,
                            symbol.key.ptr.text_range(),
                        ),
                        selection_range: create_selection_range(symbol, &root, &line_index),
                        data: None,
                    }])
                }
                SymbolItemKind::Call(..) if symbol.key.ptr.text_range() == parent_range => {
                    symbol_table.find_func_defs(&symbol.key).map(|symbols| {
                        symbols
                            .filter_map(|symbol| {
                                if let SymbolItemKind::Func(idx) = &symbol.kind {
                                    Some((symbol, idx))
                                } else {
                                    None
                                }
                            })
                            .map(|(symbol, idx)| CallHierarchyItem {
                                name: idx
                                    .name
                                    .map(|name| self.ctx.lookup_ident(name))
                                    .unwrap_or_else(|| idx.num.to_string()),
                                kind: SymbolKind::FUNCTION,
                                tags: None,
                                detail: Some(self.ctx.render_func_header(uri, symbol.clone())),
                                uri: params
                                    .text_document_position_params
                                    .text_document
                                    .uri
                                    .clone(),
                                range: helpers::rowan_range_to_lsp_range(
                                    &line_index,
                                    symbol.key.ptr.text_range(),
                                ),
                                selection_range: create_selection_range(symbol, &root, &line_index),
                                data: None,
                            })
                            .collect()
                    })
                }
                _ => None,
            })
    }

    pub fn call_hierarchy_incoming_calls(
        &self,
        params: CallHierarchyIncomingCallsParams,
    ) -> Option<Vec<CallHierarchyIncomingCall>> {
        let uri = self.ctx.uri(params.item.uri.clone());
        let root = self.build_root(uri);
        let symbol_table = self.ctx.symbol_table(uri);

        let line_index = self.ctx.line_index(uri);
        let callee_def_range = helpers::lsp_range_to_rowan_range(&line_index, params.item.range)?;
        let (callee_def_symbol, callee_def_idx) =
            symbol_table.symbols.iter().find_map(|symbol| {
                if let SymbolItemKind::Func(idx) = &symbol.kind {
                    if symbol.key.ptr.text_range() == callee_def_range {
                        Some((symbol, idx))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })?;
        let items = symbol_table
            .symbols
            .iter()
            .filter(|symbol| {
                if let SymbolItemKind::Call(idx) = &symbol.kind {
                    idx == callee_def_idx && callee_def_symbol.region == symbol.region
                } else {
                    false
                }
            })
            .filter_map(|call_symbol| {
                call_symbol
                    .key
                    .ptr
                    .to_node(&root)
                    .ancestors()
                    .find(|node| node.kind() == SyntaxKind::MODULE_FIELD_FUNC)
                    .and_then(|node| {
                        let key = node.into();
                        symbol_table
                            .symbols
                            .iter()
                            .find_map(move |symbol| match &symbol.kind {
                                SymbolItemKind::Func(idx) if symbol.key == key => {
                                    Some((call_symbol, symbol, idx))
                                }
                                _ => None,
                            })
                    })
            })
            .map(|(call_symbol, func_symbol, idx)| {
                let plain_instr_range =
                    call_symbol.key.ptr.to_node(&root).parent().map(|call| {
                        helpers::rowan_range_to_lsp_range(&line_index, call.text_range())
                    });
                CallHierarchyIncomingCall {
                    from: CallHierarchyItem {
                        name: idx
                            .name
                            .map(|name| self.ctx.lookup_ident(name))
                            .unwrap_or_else(|| idx.num.to_string()),
                        kind: SymbolKind::FUNCTION,
                        tags: None,
                        detail: Some(self.ctx.render_func_header(uri, func_symbol.clone())),
                        uri: params.item.uri.clone(),
                        range: helpers::rowan_range_to_lsp_range(
                            &line_index,
                            func_symbol.key.ptr.text_range(),
                        ),
                        selection_range: create_selection_range(func_symbol, &root, &line_index),
                        data: None,
                    },
                    from_ranges: plain_instr_range.into_iter().collect(),
                }
            })
            .collect();
        Some(items)
    }

    pub fn call_hierarchy_outgoing_calls(
        &self,
        params: CallHierarchyOutgoingCallsParams,
    ) -> Option<Vec<CallHierarchyOutgoingCall>> {
        let uri = self.ctx.uri(params.item.uri.clone());
        let root = self.build_root(uri);
        let symbol_table = self.ctx.symbol_table(uri);

        let line_index = self.ctx.line_index(uri);
        let call_def_range = helpers::lsp_range_to_rowan_range(&line_index, params.item.range)?;
        let call_def_symbol = symbol_table.symbols.iter().find(|symbol| {
            if let SymbolItemKind::Func(..) = &symbol.kind {
                symbol.key.ptr.text_range() == call_def_range
            } else {
                false
            }
        })?;
        let func = call_def_symbol.key.ptr.to_node(&root);
        let items = func
            .descendants()
            .filter(|node| node.kind() == SyntaxKind::PLAIN_INSTR && is_call(node))
            .flat_map(|node| {
                let plain_instr_range =
                    helpers::rowan_range_to_lsp_range(&line_index, node.text_range());
                node.children()
                    .filter(|child| child.kind() == SyntaxKind::OPERAND)
                    .filter_map(|operand| symbol_table.find_func_defs(&operand.into()))
                    .flatten()
                    .filter_map(|symbol| {
                        if let SymbolItemKind::Func(idx) = &symbol.kind {
                            Some((symbol, idx))
                        } else {
                            None
                        }
                    })
                    .map(move |(func_symbol, idx)| {
                        let line_index = self.ctx.line_index(uri);
                        CallHierarchyOutgoingCall {
                            to: CallHierarchyItem {
                                name: idx
                                    .name
                                    .map(|name| self.ctx.lookup_ident(name))
                                    .unwrap_or_else(|| idx.num.to_string()),
                                kind: SymbolKind::FUNCTION,
                                tags: None,
                                detail: Some(self.ctx.render_func_header(uri, func_symbol.clone())),
                                uri: self.ctx.lookup_uri(uri),
                                range: helpers::rowan_range_to_lsp_range(
                                    &line_index,
                                    func_symbol.key.ptr.text_range(),
                                ),
                                selection_range: create_selection_range(
                                    func_symbol,
                                    &self.build_root(uri),
                                    &line_index,
                                ),
                                data: None,
                            },
                            from_ranges: vec![plain_instr_range],
                        }
                    })
            })
            .collect();
        Some(items)
    }
}

fn create_selection_range(symbol: &SymbolItem, root: &SyntaxNode, line_index: &LineIndex) -> Range {
    let node = symbol.key.ptr.to_node(root);
    let range = token(&node, SyntaxKind::IDENT)
        .or_else(|| token(&node, SyntaxKind::KEYWORD))
        .map(|token| token.text_range())
        .unwrap_or_else(|| node.text_range());
    helpers::rowan_range_to_lsp_range(line_index, range)
}
