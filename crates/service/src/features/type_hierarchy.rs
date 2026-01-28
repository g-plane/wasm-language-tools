use crate::{
    LanguageService,
    binder::{SymbolKind, SymbolTable},
    helpers::{self, LineIndexExt},
    types_analyzer,
};
use lspt::{
    SymbolKind as LspSymbolKind, TypeHierarchyItem, TypeHierarchyPrepareParams, TypeHierarchySubtypesParams,
    TypeHierarchySupertypesParams,
};

impl LanguageService {
    /// Handler for `textDocument/prepareTypeHierarchy` request.
    pub fn prepare_type_hierarchy(&self, params: TypeHierarchyPrepareParams) -> Option<Vec<TypeHierarchyItem>> {
        let document = self.get_document(&params.text_document.uri)?;
        let line_index = document.line_index(self);
        let root = document.root_tree(self);
        let symbol_table = SymbolTable::of(self, document);

        let token = super::find_meaningful_token(self, document, &root, params.position)?;
        let parent_range = token.parent()?.text_range();

        symbol_table.symbols.values().find_map(|symbol| match symbol.kind {
            SymbolKind::Type if symbol.key.text_range() == parent_range => Some(vec![TypeHierarchyItem {
                name: symbol.idx.render(self).to_string(),
                kind: LspSymbolKind::Class,
                tags: None,
                detail: helpers::syntax::infer_type_def_symbol_detail(symbol, &root),
                uri: params.text_document.uri.clone(),
                range: line_index.convert(symbol.key.text_range()),
                selection_range: helpers::create_selection_range(symbol, &root, line_index),
                data: None,
            }]),
            SymbolKind::TypeUse if symbol.key.text_range() == parent_range => {
                symbol_table.find_def(symbol.key).map(|symbol| {
                    vec![TypeHierarchyItem {
                        name: symbol.idx.render(self).to_string(),
                        kind: LspSymbolKind::Class,
                        tags: None,
                        detail: helpers::syntax::infer_type_def_symbol_detail(symbol, &root),
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

    /// Handler for `typeHierarchy/supertypes` request.
    pub fn type_hierarchy_supertypes(&self, params: TypeHierarchySupertypesParams) -> Option<Vec<TypeHierarchyItem>> {
        let document = self.get_document(&params.item.uri)?;
        let root = document.root_tree(self);
        let symbol_table = SymbolTable::of(self, document);
        let def_types = types_analyzer::get_def_types(self, document);

        let line_index = document.line_index(self);
        let type_def_range = line_index.convert(params.item.range)?;
        let type_def = symbol_table
            .symbols
            .values()
            .find(|symbol| symbol.key.text_range() == type_def_range)?;

        def_types
            .get(&type_def.key)
            .and_then(|def_type| def_type.inherits.as_ref())
            .and_then(|inherits| symbol_table.symbols.get(&inherits.symbol))
            .map(|symbol| {
                vec![TypeHierarchyItem {
                    name: symbol.idx.render(self).to_string(),
                    kind: LspSymbolKind::Class,
                    tags: None,
                    detail: helpers::syntax::infer_type_def_symbol_detail(symbol, &root),
                    uri: params.item.uri,
                    range: line_index.convert(symbol.key.text_range()),
                    selection_range: helpers::create_selection_range(symbol, &root, line_index),
                    data: None,
                }]
            })
    }

    /// Handler for `typeHierarchy/subtypes` request.
    pub fn type_hierarchy_subtypes(&self, params: TypeHierarchySubtypesParams) -> Option<Vec<TypeHierarchyItem>> {
        let document = self.get_document(&params.item.uri)?;
        let root = document.root_tree(self);
        let symbol_table = SymbolTable::of(self, document);
        let def_types = types_analyzer::get_def_types(self, document);

        let line_index = document.line_index(self);
        let type_def_range = line_index.convert(params.item.range)?;
        let key = symbol_table
            .symbols
            .values()
            .find(|symbol| symbol.key.text_range() == type_def_range)?
            .key;

        let mut items = def_types
            .iter()
            .filter(|(_, def_type)| {
                def_type
                    .inherits
                    .as_ref()
                    .is_some_and(|inherits| inherits.symbol == key)
            })
            .filter_map(|(key, _)| symbol_table.symbols.get(key))
            .map(|symbol| TypeHierarchyItem {
                name: symbol.idx.render(self).to_string(),
                kind: LspSymbolKind::Class,
                tags: None,
                detail: helpers::syntax::infer_type_def_symbol_detail(symbol, &root),
                uri: params.item.uri.clone(),
                range: line_index.convert(symbol.key.text_range()),
                selection_range: helpers::create_selection_range(symbol, &root, line_index),
                data: None,
            })
            .collect::<Vec<_>>();
        items.sort_unstable_by_key(|item| item.range.start);
        Some(items)
    }
}
