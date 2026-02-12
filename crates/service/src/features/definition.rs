use crate::{
    LanguageService,
    binder::{SymbolKey, SymbolTable},
    helpers::LineIndexExt,
};
use lspt::{Declaration, DeclarationParams, Definition, DefinitionParams, Location, TypeDefinitionParams, Union2};
use wat_syntax::SyntaxKind;

impl LanguageService {
    /// Handler for `textDocument/definition` request.
    pub fn goto_definition(&self, params: DefinitionParams) -> Option<Definition> {
        let document = self.get_document(&params.text_document.uri)?;
        let root = document.root_tree(self);
        let token = super::find_meaningful_token(self, document, &root, params.position)?;

        let parent = token.parent();
        if !matches!(parent.kind(), SyntaxKind::IMMEDIATE | SyntaxKind::INDEX) {
            return None;
        }
        let line_index = document.line_index(self);
        let symbol_table = SymbolTable::of(self, document);
        let key = SymbolKey::new(&parent);
        symbol_table
            .resolved
            .get(&key)
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
        let root = document.root_tree(self);
        let symbol_table = SymbolTable::of(self, document);
        let token = super::find_meaningful_token(self, document, &root, params.position)?;

        let parent = token.parent();
        if !matches!(parent.kind(), SyntaxKind::IMMEDIATE | SyntaxKind::INDEX) {
            return None;
        }
        symbol_table
            .resolved
            .get(&SymbolKey::new(&parent))
            .and_then(|key| {
                key.try_to_node(&root)?
                    .children()
                    .find_map(|child| match child.kind() {
                        SyntaxKind::TYPE_USE | SyntaxKind::HEAP_TYPE => {
                            child.first_child_by_kind(|kind| kind == SyntaxKind::INDEX)
                        }
                        SyntaxKind::REF_TYPE => child
                            .first_child_by_kind(|kind| kind == SyntaxKind::HEAP_TYPE)
                            .and_then(|node| node.first_child_by_kind(|kind| kind == SyntaxKind::INDEX)),
                        SyntaxKind::GLOBAL_TYPE => child
                            .first_child_by_kind(|kind| kind == SyntaxKind::REF_TYPE)
                            .and_then(|node| node.first_child_by_kind(|kind| kind == SyntaxKind::HEAP_TYPE))
                            .and_then(|node| node.first_child_by_kind(|kind| kind == SyntaxKind::INDEX)),
                        _ => None,
                    })
                    .and_then(|type_idx| symbol_table.resolved.get(&SymbolKey::new(&type_idx)))
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
