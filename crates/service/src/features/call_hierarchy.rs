use crate::{
    LanguageService,
    binder::{SymbolKind, SymbolTable},
    deprecation,
    helpers::LineIndexExt,
    types_analyzer,
};
use lspt::{
    CallHierarchyIncomingCall, CallHierarchyIncomingCallsParams, CallHierarchyItem, CallHierarchyOutgoingCall,
    CallHierarchyOutgoingCallsParams, CallHierarchyPrepareParams, SymbolKind as LspSymbolKind, SymbolTag,
};

impl LanguageService {
    /// Handler for `textDocument/prepareCallHierarchy` request.
    pub fn prepare_call_hierarchy(&self, params: CallHierarchyPrepareParams) -> Option<Vec<CallHierarchyItem>> {
        let document = self.get_document(&params.text_document.uri)?;
        let line_index = document.line_index(self);
        let root = document.root_tree(self);
        let symbol_table = SymbolTable::of(self, document);
        let deprecation = deprecation::get_deprecation(self, document);

        let token = super::find_meaningful_token(self, document, &root, params.position)?;
        let parent_range = token.parent().text_range();

        symbol_table.symbols.values().find_map(|symbol| match symbol.kind {
            SymbolKind::Func if symbol.key.text_range() == parent_range => Some(vec![CallHierarchyItem {
                name: symbol.idx.render(self).to_string(),
                kind: LspSymbolKind::Function,
                tags: if deprecation.contains_key(&symbol.key) {
                    Some(vec![SymbolTag::Deprecated])
                } else {
                    None
                },
                detail: Some(types_analyzer::render_func_header(
                    self,
                    symbol.idx.name,
                    types_analyzer::get_func_sig(self, document, symbol.key, &symbol.green),
                )),
                uri: params.text_document.uri.clone(),
                range: line_index.convert(symbol.key.text_range()),
                selection_range: line_index.convert(
                    symbol_table
                        .def_poi
                        .get(&symbol.key)
                        .copied()
                        .unwrap_or_else(|| symbol.key.text_range()),
                ),
                data: None,
            }]),
            SymbolKind::Call if symbol.key.text_range() == parent_range => {
                symbol_table.find_def(symbol.key).map(|symbol| {
                    vec![CallHierarchyItem {
                        name: symbol.idx.render(self).to_string(),
                        kind: LspSymbolKind::Function,
                        tags: if deprecation.contains_key(&symbol.key) {
                            Some(vec![SymbolTag::Deprecated])
                        } else {
                            None
                        },
                        detail: Some(types_analyzer::render_func_header(
                            self,
                            symbol.idx.name,
                            types_analyzer::get_func_sig(self, document, symbol.key, &symbol.green),
                        )),
                        uri: params.text_document.uri.clone(),
                        range: line_index.convert(symbol.key.text_range()),
                        selection_range: line_index.convert(
                            symbol_table
                                .def_poi
                                .get(&symbol.key)
                                .copied()
                                .unwrap_or_else(|| symbol.key.text_range()),
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
        let document = self.get_document(&params.item.uri)?;
        let line_index = document.line_index(self);
        let symbol_table = SymbolTable::of(self, document);
        let deprecation = deprecation::get_deprecation(self, document);
        let callee_def_range = line_index.convert(params.item.range)?;
        let mut items = symbol_table
            .resolved
            .iter()
            .filter_map(|(ref_key, def_key)| {
                if def_key.text_range() == callee_def_range {
                    Some(ref_key)
                } else {
                    None
                }
            })
            .filter_map(|call_key| {
                symbol_table
                    .symbols
                    .values()
                    .find(|symbol| {
                        symbol.kind == SymbolKind::Func && symbol.key.text_range().contains_range(call_key.text_range())
                    })
                    .map(|symbol| CallHierarchyIncomingCall {
                        from: CallHierarchyItem {
                            name: symbol.idx.render(self).to_string(),
                            kind: LspSymbolKind::Function,
                            tags: if deprecation.contains_key(&symbol.key) {
                                Some(vec![SymbolTag::Deprecated])
                            } else {
                                None
                            },
                            detail: Some(types_analyzer::render_func_header(
                                self,
                                symbol.idx.name,
                                types_analyzer::get_func_sig(self, document, symbol.key, &symbol.green),
                            )),
                            uri: params.item.uri.clone(),
                            range: line_index.convert(symbol.key.text_range()),
                            selection_range: line_index.convert(
                                symbol_table
                                    .def_poi
                                    .get(&symbol.key)
                                    .copied()
                                    .unwrap_or_else(|| symbol.key.text_range()),
                            ),
                            data: None,
                        },
                        from_ranges: vec![line_index.convert(call_key.text_range())],
                    })
            })
            .collect::<Vec<_>>();
        items.sort_unstable_by_key(|item| item.from.range.start);
        Some(items)
    }

    /// Handler for `callHierarchy/outgoingCalls` request.
    pub fn call_hierarchy_outgoing_calls(
        &self,
        params: CallHierarchyOutgoingCallsParams,
    ) -> Option<Vec<CallHierarchyOutgoingCall>> {
        let document = self.get_document(&params.item.uri)?;
        let line_index = document.line_index(self);
        let symbol_table = SymbolTable::of(self, document);
        let deprecation = deprecation::get_deprecation(self, document);
        let call_def_range = line_index.convert(params.item.range)?;
        let mut items = symbol_table
            .symbols
            .values()
            .filter(|symbol| symbol.kind == SymbolKind::Call && call_def_range.contains_range(symbol.key.text_range()))
            .filter_map(|symbol| symbol_table.find_def(symbol.key).map(|def_symbol| (def_symbol, symbol)))
            .map(|(def_symbol, ref_symbol)| CallHierarchyOutgoingCall {
                to: CallHierarchyItem {
                    name: def_symbol.idx.render(self).to_string(),
                    kind: LspSymbolKind::Function,
                    tags: if deprecation.contains_key(&def_symbol.key) {
                        Some(vec![SymbolTag::Deprecated])
                    } else {
                        None
                    },
                    detail: Some(types_analyzer::render_func_header(
                        self,
                        def_symbol.idx.name,
                        types_analyzer::get_func_sig(self, document, def_symbol.key, &def_symbol.green),
                    )),
                    uri: params.item.uri.clone(),
                    range: line_index.convert(def_symbol.key.text_range()),
                    selection_range: line_index.convert(
                        symbol_table
                            .def_poi
                            .get(&def_symbol.key)
                            .copied()
                            .unwrap_or_else(|| def_symbol.key.text_range()),
                    ),
                    data: None,
                },
                from_ranges: vec![line_index.convert(ref_symbol.key.text_range())],
            })
            .collect::<Vec<_>>();
        items.sort_unstable_by_key(|item| item.to.range.start);
        Some(items)
    }
}
