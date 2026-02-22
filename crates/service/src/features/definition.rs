use crate::{LanguageService, binder::SymbolTable, helpers::LineIndexExt};
use lspt::{Declaration, DeclarationParams, Definition, DefinitionParams, Location, TypeDefinitionParams, Union2};
use wat_syntax::SyntaxKind;

impl LanguageService {
    /// Handler for `textDocument/definition` request.
    pub fn goto_definition(&self, params: DefinitionParams) -> Option<Definition> {
        let document = self.get_document(&params.text_document.uri)?;
        let line_index = document.line_index(self);
        let position = line_index.convert(params.position)?;
        let symbol_table = SymbolTable::of(self, document);
        symbol_table
            .resolved
            .iter()
            .find_map(|(ref_key, def_key)| {
                if ref_key.text_range().contains_inclusive(position) {
                    Some(def_key)
                } else {
                    None
                }
            })
            .and_then(|key| symbol_table.def_poi.get(key))
            .map(|range| {
                Union2::A(Location {
                    uri: params.text_document.uri.clone(),
                    range: line_index.convert(*range),
                })
            })
    }

    /// Handler for `textDocument/typeDefinition` request.
    pub fn goto_type_definition(&self, params: TypeDefinitionParams) -> Option<Definition> {
        let document = self.get_document(&params.text_document.uri)?;
        let line_index = document.line_index(self);
        let position = line_index.convert(params.position)?;
        let symbol_table = SymbolTable::of(self, document);
        symbol_table
            .resolved
            .iter()
            .find_map(|(ref_key, def_key)| {
                if ref_key.text_range().contains_inclusive(position) {
                    symbol_table.symbols.get(def_key)
                } else {
                    None
                }
            })
            .and_then(|symbol| {
                symbol
                    .amber()
                    .children()
                    .find_map(|child| match child.kind() {
                        SyntaxKind::TYPE_USE | SyntaxKind::HEAP_TYPE => {
                            child.children_by_kind(SyntaxKind::INDEX).next()
                        }
                        SyntaxKind::REF_TYPE => child
                            .children_by_kind(SyntaxKind::HEAP_TYPE)
                            .next()
                            .and_then(|node| node.children_by_kind(SyntaxKind::INDEX).next()),
                        SyntaxKind::GLOBAL_TYPE => child
                            .children_by_kind(SyntaxKind::REF_TYPE)
                            .next()
                            .and_then(|node| node.children_by_kind(SyntaxKind::HEAP_TYPE).next())
                            .and_then(|node| node.children_by_kind(SyntaxKind::INDEX).next()),
                        _ => None,
                    })
                    .and_then(|type_idx| symbol_table.resolved.get(&type_idx.to_ptr().into()))
            })
            .and_then(|key| symbol_table.def_poi.get(key))
            .map(|range| {
                Union2::A(Location {
                    uri: params.text_document.uri.clone(),
                    range: line_index.convert(*range),
                })
            })
    }

    /// Handler for `textDocument/declaration` request.
    pub fn goto_declaration(&self, params: DeclarationParams) -> Option<Declaration> {
        self.goto_definition(DefinitionParams {
            text_document: params.text_document,
            position: params.position,
            work_done_token: params.work_done_token,
            partial_result_token: params.partial_result_token,
        })
    }
}
