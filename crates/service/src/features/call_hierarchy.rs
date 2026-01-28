use crate::{
    LanguageService,
    binder::{SymbolKey, SymbolKind, SymbolTable},
    helpers::{self, LineIndexExt},
    types_analyzer,
};
use lspt::{
    CallHierarchyIncomingCall, CallHierarchyIncomingCallsParams, CallHierarchyItem, CallHierarchyOutgoingCall,
    CallHierarchyOutgoingCallsParams, CallHierarchyPrepareParams, SymbolKind as LspSymbolKind,
};
use wat_syntax::SyntaxKind;

impl LanguageService {
    /// Handler for `textDocument/prepareCallHierarchy` request.
    pub fn prepare_call_hierarchy(&self, params: CallHierarchyPrepareParams) -> Option<Vec<CallHierarchyItem>> {
        let document = self.get_document(&params.text_document.uri)?;
        let line_index = document.line_index(self);
        let root = document.root_tree(self);
        let symbol_table = SymbolTable::of(self, document);

        let token = super::find_meaningful_token(self, document, &root, params.position)?;
        let parent_range = token.parent()?.text_range();

        symbol_table.symbols.values().find_map(|symbol| match symbol.kind {
            SymbolKind::Func if symbol.key.text_range() == parent_range => Some(vec![CallHierarchyItem {
                name: symbol.idx.render(self).to_string(),
                kind: LspSymbolKind::Function,
                tags: None,
                detail: Some(types_analyzer::render_func_header(
                    self,
                    symbol.idx.name,
                    types_analyzer::get_func_sig(self, document, *symbol.key, &symbol.green),
                )),
                uri: params.text_document.uri.clone(),
                range: line_index.convert(symbol.key.text_range()),
                selection_range: helpers::create_selection_range(symbol, &root, line_index),
                data: None,
            }]),
            SymbolKind::Call if symbol.key.text_range() == parent_range => {
                symbol_table.find_def(symbol.key).map(|symbol| {
                    vec![CallHierarchyItem {
                        name: symbol.idx.render(self).to_string(),
                        kind: LspSymbolKind::Function,
                        tags: None,
                        detail: Some(types_analyzer::render_func_header(
                            self,
                            symbol.idx.name,
                            types_analyzer::get_func_sig(self, document, *symbol.key, &symbol.green),
                        )),
                        uri: params.text_document.uri.clone(),
                        range: line_index.convert(symbol.key.text_range()),
                        selection_range: helpers::create_selection_range(symbol, &root, line_index),
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
        let document = self.get_document(&params.item.uri)?;
        let root = document.root_tree(self);
        let symbol_table = SymbolTable::of(self, document);

        let line_index = document.line_index(self);
        let callee_def_range = line_index.convert(params.item.range)?;
        let callee_def = symbol_table
            .symbols
            .values()
            .find(|symbol| symbol.key.text_range() == callee_def_range)?;
        let items = symbol_table
            .symbols
            .values()
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
                    .and_then(|node| symbol_table.symbols.get(&SymbolKey::new(&node)))
                    .map(|symbol| (call_symbol, symbol))
            })
            .map(|(call_symbol, func_symbol)| {
                let plain_instr_range = call_symbol
                    .key
                    .to_node(&root)
                    .parent()
                    .map(|call| line_index.convert(call.text_range()));
                CallHierarchyIncomingCall {
                    from: CallHierarchyItem {
                        name: func_symbol.idx.render(self).to_string(),
                        kind: LspSymbolKind::Function,
                        tags: None,
                        detail: Some(types_analyzer::render_func_header(
                            self,
                            func_symbol.idx.name,
                            types_analyzer::get_func_sig(self, document, *func_symbol.key, &func_symbol.green),
                        )),
                        uri: params.item.uri.clone(),
                        range: line_index.convert(func_symbol.key.text_range()),
                        selection_range: helpers::create_selection_range(func_symbol, &root, line_index),
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
        let document = self.get_document(&params.item.uri)?;
        let root = &document.root_tree(self);
        let symbol_table = SymbolTable::of(self, document);

        let line_index = document.line_index(self);
        let call_def_range = line_index.convert(params.item.range)?;
        let call_def_symbol = symbol_table
            .symbols
            .values()
            .find(|symbol| symbol.key.text_range() == call_def_range)?;
        let func = call_def_symbol.key.to_node(root);
        let items = func
            .descendants()
            .filter(|node| node.kind() == SyntaxKind::PLAIN_INSTR && helpers::syntax::is_call(node))
            .flat_map(|node| {
                let plain_instr_range = line_index.convert(node.text_range());
                let uri = &params.item.uri;
                node.children()
                    .filter(|child| child.kind() == SyntaxKind::IMMEDIATE)
                    .filter_map(|immediate| symbol_table.find_def(SymbolKey::new(&immediate)))
                    .filter(|symbol| symbol.kind == SymbolKind::Func)
                    .map(move |func_symbol| CallHierarchyOutgoingCall {
                        to: CallHierarchyItem {
                            name: func_symbol.idx.render(self).to_string(),
                            kind: LspSymbolKind::Function,
                            tags: None,
                            detail: Some(types_analyzer::render_func_header(
                                self,
                                func_symbol.idx.name,
                                types_analyzer::get_func_sig(self, document, *func_symbol.key, &func_symbol.green),
                            )),
                            uri: uri.clone(),
                            range: line_index.convert(func_symbol.key.text_range()),
                            selection_range: helpers::create_selection_range(func_symbol, root, line_index),
                            data: None,
                        },
                        from_ranges: vec![plain_instr_range],
                    })
            })
            .collect();
        Some(items)
    }
}
