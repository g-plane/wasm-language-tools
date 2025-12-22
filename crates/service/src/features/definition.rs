use crate::{
    LanguageService,
    binder::{SymbolKey, SymbolTable},
    helpers,
};
use lspt::{
    Declaration, DeclarationParams, Definition, DefinitionParams, TypeDefinitionParams, Union2,
};
use rowan::ast::{AstNode, support};
use wat_syntax::{SyntaxKind, ast::TypeUse};

impl LanguageService {
    /// Handler for `textDocument/definition` request.
    pub fn goto_definition(&self, params: DefinitionParams) -> Option<Definition> {
        let document = self.get_document(&params.text_document.uri)?;
        let root = document.root_tree(self);
        let token = super::find_meaningful_token(self, document, &root, params.position)?;

        let parent = token
            .parent()
            .filter(|parent| matches!(parent.kind(), SyntaxKind::IMMEDIATE | SyntaxKind::INDEX))?;
        let line_index = document.line_index(self);
        let symbol_table = SymbolTable::of(self, document);
        let key = SymbolKey::new(&parent);
        symbol_table.resolved.get(&key).map(|key| {
            Union2::A(helpers::create_location_by_symbol(
                params.text_document.uri.clone(),
                line_index,
                *key,
                &root,
            ))
        })
    }

    /// Handler for `textDocument/typeDefinition` request.
    pub fn goto_type_definition(&self, params: TypeDefinitionParams) -> Option<Definition> {
        let document = self.get_document(&params.text_document.uri)?;
        let line_index = document.line_index(self);
        let root = document.root_tree(self);
        let symbol_table = SymbolTable::of(self, document);
        let token = super::find_meaningful_token(self, document, &root, params.position)?;

        let parent = token
            .parent()
            .filter(|parent| matches!(parent.kind(), SyntaxKind::IMMEDIATE | SyntaxKind::INDEX))?;
        let grand = parent.parent()?;
        match grand.kind() {
            SyntaxKind::PLAIN_INSTR => symbol_table
                .resolved
                .get(&SymbolKey::new(&parent))
                .and_then(|key| {
                    symbol_table.resolved.get(&SymbolKey::new(
                        support::child::<TypeUse>(&key.to_node(&root))?
                            .index()?
                            .syntax(),
                    ))
                })
                .map(|key| {
                    Union2::A(helpers::create_location_by_symbol(
                        params.text_document.uri.clone(),
                        line_index,
                        *key,
                        &root,
                    ))
                }),
            _ => None,
        }
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
