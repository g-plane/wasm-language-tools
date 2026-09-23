use crate::{
    LanguageService,
    binder::{SymbolKey, SymbolTable},
    helpers::{self, LineIndexExt},
};
use lspt::{Declaration, DeclarationParams, Definition, DefinitionParams, Location, TypeDefinitionParams};
use wat_syntax::{SyntaxKind, SyntaxNode};

impl LanguageService {
    /// Handler for `textDocument/definition` request.
    pub fn goto_definition(&self, params: DefinitionParams) -> Option<Definition> {
        let document = self.get_document(&params.text_document.uri)?;
        let line_index = document.line_index(self);
        let root = SyntaxNode::new_root(document.root(self));
        let symbol_table = SymbolTable::of(self, document);
        let parent = super::find_meaningful_token(self, document, &root, params.position)?.parent();
        symbol_table
            .find_def(SymbolKey::from(&parent))
            .and_then(|symbol| line_index.convert(helpers::syntax::infer_def_poi(symbol.amber())))
            .map(|range| {
                Definition::Location(Location {
                    uri: params.text_document.uri,
                    range,
                })
            })
    }

    /// Handler for `textDocument/typeDefinition` request.
    pub fn goto_type_definition(&self, params: TypeDefinitionParams) -> Option<Definition> {
        let document = self.get_document(&params.text_document.uri)?;
        let line_index = document.line_index(self);
        let root = SyntaxNode::new_root(document.root(self));
        let symbol_table = SymbolTable::of(self, document);
        let parent = super::find_meaningful_token(self, document, &root, params.position)?.parent();
        symbol_table
            .find_def(SymbolKey::from(&parent))
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
                    .and_then(|type_idx| symbol_table.find_def(type_idx.into()))
            })
            .and_then(|symbol| line_index.convert(helpers::syntax::infer_def_poi(symbol.amber())))
            .map(|range| {
                Definition::Location(Location {
                    uri: params.text_document.uri.clone(),
                    range,
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
        .map(|definition| match definition {
            Definition::Location(location) => Declaration::Location(location),
            Definition::List(locations) => Declaration::List(locations),
        })
    }
}
