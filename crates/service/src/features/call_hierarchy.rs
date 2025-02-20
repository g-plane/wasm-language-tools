use crate::{
    binder::{SymbolKey, SymbolKind, SymbolTablesCtx},
    helpers,
    syntax_tree::SyntaxTreeCtx,
    types_analyzer::TypesAnalyzerCtx,
    uri::UrisCtx,
    LanguageService,
};
use lspt::{
    CallHierarchyIncomingCall, CallHierarchyIncomingCallsParams, CallHierarchyItem,
    CallHierarchyOutgoingCall, CallHierarchyOutgoingCallsParams, CallHierarchyPrepareParams,
    SymbolKind as LspSymbolKind,
};
use wat_syntax::{SyntaxKind, SyntaxNode};

impl LanguageService {
    /// Handler for `textDocument/prepareCallHierarchy` request.
    pub fn prepare_call_hierarchy(
        &self,
        params: CallHierarchyPrepareParams,
    ) -> Option<Vec<CallHierarchyItem>> {
        let uri = self.uri(params.text_document.uri.clone());
        let line_index = self.line_index(uri);
        let root = SyntaxNode::new_root(self.root(uri));
        let symbol_table = self.symbol_table(uri);

        let token = super::find_meaningful_token(self, uri, &root, params.position)?;
        let parent_range = token.parent()?.text_range();

        symbol_table
            .symbols
            .iter()
            .find_map(|symbol| match symbol.kind {
                SymbolKind::Func if symbol.key.text_range() == parent_range => {
                    Some(vec![CallHierarchyItem {
                        name: symbol.idx.render(self).to_string(),
                        kind: LspSymbolKind::Function,
                        tags: None,
                        detail: Some(self.render_func_header(
                            symbol.idx.name,
                            self.get_func_sig(uri, symbol.key, symbol.green.clone()),
                        )),
                        uri: params.text_document.uri.clone(),
                        range: helpers::rowan_range_to_lsp_range(
                            &line_index,
                            symbol.key.text_range(),
                        ),
                        selection_range: helpers::create_selection_range(
                            symbol,
                            &root,
                            &line_index,
                        ),
                        data: None,
                    }])
                }
                SymbolKind::Call if symbol.key.text_range() == parent_range => {
                    symbol_table.find_def(symbol.key).map(|symbol| {
                        vec![CallHierarchyItem {
                            name: symbol.idx.render(self).to_string(),
                            kind: LspSymbolKind::Function,
                            tags: None,
                            detail: Some(self.render_func_header(
                                symbol.idx.name,
                                self.get_func_sig(uri, symbol.key, symbol.green.clone()),
                            )),
                            uri: params.text_document.uri.clone(),
                            range: helpers::rowan_range_to_lsp_range(
                                &line_index,
                                symbol.key.text_range(),
                            ),
                            selection_range: helpers::create_selection_range(
                                symbol,
                                &root,
                                &line_index,
                            ),
                            data: None,
                        }]
                    })
                }
                _ => None,
            })
    }

    /// Handler for `callHierarchy/incomingCalls` request.
    pub fn call_hierarchy_incoming_calls(
        &self,
        params: CallHierarchyIncomingCallsParams,
    ) -> Option<Vec<CallHierarchyIncomingCall>> {
        let uri = self.uri(params.item.uri.clone());
        let root = SyntaxNode::new_root(self.root(uri));
        let symbol_table = self.symbol_table(uri);

        let line_index = self.line_index(uri);
        let callee_def_range = helpers::lsp_range_to_rowan_range(&line_index, params.item.range)?;
        let callee_def = symbol_table
            .symbols
            .iter()
            .find(|symbol| symbol.key.text_range() == callee_def_range)?;
        let items = symbol_table
            .symbols
            .iter()
            .filter(|symbol| {
                symbol.kind == SymbolKind::Call
                    && symbol.idx.is_defined_by(&callee_def.idx)
                    && callee_def.region == symbol.region
            })
            .filter_map(|call_symbol| {
                call_symbol
                    .key
                    .to_node(&root)
                    .ancestors()
                    .find(|node| node.kind() == SyntaxKind::MODULE_FIELD_FUNC)
                    .and_then(|node| {
                        let key = SymbolKey::new(&node);
                        symbol_table.symbols.iter().find_map(move |symbol| {
                            (symbol.kind == SymbolKind::Func && symbol.key == key)
                                .then_some((call_symbol, symbol))
                        })
                    })
            })
            .map(|(call_symbol, func_symbol)| {
                let plain_instr_range =
                    call_symbol.key.to_node(&root).parent().map(|call| {
                        helpers::rowan_range_to_lsp_range(&line_index, call.text_range())
                    });
                CallHierarchyIncomingCall {
                    from: CallHierarchyItem {
                        name: func_symbol.idx.render(self).to_string(),
                        kind: LspSymbolKind::Function,
                        tags: None,
                        detail: Some(self.render_func_header(
                            func_symbol.idx.name,
                            self.get_func_sig(uri, func_symbol.key, func_symbol.green.clone()),
                        )),
                        uri: params.item.uri.clone(),
                        range: helpers::rowan_range_to_lsp_range(
                            &line_index,
                            func_symbol.key.text_range(),
                        ),
                        selection_range: helpers::create_selection_range(
                            func_symbol,
                            &root,
                            &line_index,
                        ),
                        data: None,
                    },
                    from_ranges: plain_instr_range.into_iter().collect(),
                }
            })
            .collect();
        Some(items)
    }

    /// Handler for `callHierarchy/outgoingCalls` request.
    pub fn call_hierarchy_outgoing_calls(
        &self,
        params: CallHierarchyOutgoingCallsParams,
    ) -> Option<Vec<CallHierarchyOutgoingCall>> {
        let uri = self.uri(params.item.uri.clone());
        let root = SyntaxNode::new_root(self.root(uri));
        let symbol_table = self.symbol_table(uri);

        let line_index = self.line_index(uri);
        let call_def_range = helpers::lsp_range_to_rowan_range(&line_index, params.item.range)?;
        let call_def_symbol = symbol_table
            .symbols
            .iter()
            .find(|symbol| symbol.key.text_range() == call_def_range)?;
        let func = call_def_symbol.key.to_node(&root);
        let items = func
            .descendants()
            .filter(|node| node.kind() == SyntaxKind::PLAIN_INSTR && helpers::ast::is_call(node))
            .flat_map(|node| {
                let plain_instr_range =
                    helpers::rowan_range_to_lsp_range(&line_index, node.text_range());
                node.children()
                    .filter(|child| child.kind() == SyntaxKind::IMMEDIATE)
                    .filter_map(|immediate| symbol_table.find_def(SymbolKey::new(&immediate)))
                    .filter(|symbol| symbol.kind == SymbolKind::Func)
                    .map(move |func_symbol| {
                        let line_index = self.line_index(uri);
                        CallHierarchyOutgoingCall {
                            to: CallHierarchyItem {
                                name: func_symbol.idx.render(self).to_string(),
                                kind: LspSymbolKind::Function,
                                tags: None,
                                detail: Some(self.render_func_header(
                                    func_symbol.idx.name,
                                    self.get_func_sig(
                                        uri,
                                        func_symbol.key,
                                        func_symbol.green.clone(),
                                    ),
                                )),
                                uri: self.lookup_uri(uri),
                                range: helpers::rowan_range_to_lsp_range(
                                    &line_index,
                                    func_symbol.key.text_range(),
                                ),
                                selection_range: helpers::create_selection_range(
                                    func_symbol,
                                    &SyntaxNode::new_root(self.root(uri)),
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
