use super::find_meaningful_token;
use crate::{
    binder::{Symbol, SymbolKey, SymbolTablesCtx},
    helpers,
    syntax_tree::SyntaxTreeCtx,
    uri::UrisCtx,
    LanguageService,
};
use line_index::LineIndex;
use lspt::{
    Declaration, DeclarationParams, Definition, DefinitionParams, Location, TypeDefinitionParams,
    Union2,
};
use rowan::ast::{
    support::{child, token},
    AstNode,
};
use wat_syntax::{ast::TypeUse, SyntaxKind, SyntaxNode};

impl LanguageService {
    /// Handler for `textDocument/definition` request.
    pub fn goto_definition(&self, params: DefinitionParams) -> Option<Definition> {
        let uri = self.uri(params.text_document.uri.clone());
        let root = SyntaxNode::new_root(self.root(uri));
        let token = find_meaningful_token(self, uri, &root, params.position)?;

        let parent = token.parent()?;
        if !matches!(parent.kind(), SyntaxKind::IMMEDIATE | SyntaxKind::INDEX) {
            return None;
        }

        let line_index = self.line_index(uri);
        let symbol_table = self.symbol_table(uri);
        let key = SymbolKey::new(&parent);
        symbol_table
            .find_param_or_local_def(key)
            .map(|symbol| {
                Union2::A(create_location_by_symbol(
                    params.text_document.uri.clone(),
                    &line_index,
                    symbol,
                    &root,
                ))
            })
            .or_else(|| {
                symbol_table.find_def(key).map(|symbol| {
                    Union2::A(create_location_by_symbol(
                        params.text_document.uri.clone(),
                        &line_index,
                        symbol,
                        &root,
                    ))
                })
            })
            .or_else(|| {
                symbol_table
                    .find_block_def(key)
                    .and_then(|key| symbol_table.symbols.iter().find(|symbol| symbol.key == key))
                    .map(|symbol| {
                        Union2::A(create_location_by_symbol(
                            params.text_document.uri.clone(),
                            &line_index,
                            symbol,
                            &root,
                        ))
                    })
            })
    }

    /// Handler for `textDocument/typeDefinition` request.
    pub fn goto_type_definition(&self, params: TypeDefinitionParams) -> Option<Definition> {
        let uri = self.uri(params.text_document.uri.clone());
        let line_index = self.line_index(uri);
        let root = SyntaxNode::new_root(self.root(uri));
        let symbol_table = self.symbol_table(uri);
        let token = find_meaningful_token(self, uri, &root, params.position)?;

        let parent = token.parent()?;
        if !matches!(parent.kind(), SyntaxKind::IMMEDIATE | SyntaxKind::INDEX) {
            return None;
        }

        let grand = parent.parent()?;
        match grand.kind() {
            SyntaxKind::PLAIN_INSTR => symbol_table
                .find_def(SymbolKey::new(&parent))
                .and_then(|symbol| {
                    symbol_table.find_def(SymbolKey::new(
                        child::<TypeUse>(&symbol.key.to_node(&root))?
                            .index()?
                            .syntax(),
                    ))
                })
                .map(|symbol| {
                    Union2::A(create_location_by_symbol(
                        params.text_document.uri.clone(),
                        &line_index,
                        symbol,
                        &root,
                    ))
                }),
            _ => None,
        }
    }

    /// Handler for `textDocument/declaration` request.
    ///
    /// Only available for function calls currently. This behaves same as "Goto Definition".
    pub fn goto_declaration(&self, params: DeclarationParams) -> Option<Declaration> {
        let uri = self.uri(params.text_document.uri.clone());
        let line_index = self.line_index(uri);
        let root = SyntaxNode::new_root(self.root(uri));
        let symbol_table = self.symbol_table(uri);
        let token = find_meaningful_token(self, uri, &root, params.position)?;
        let parent = token.parent()?;
        if parent.kind() == SyntaxKind::IMMEDIATE {
            symbol_table
                .find_def(SymbolKey::new(&parent))
                .map(|symbol| {
                    Union2::A(create_location_by_symbol(
                        params.text_document.uri.clone(),
                        &line_index,
                        symbol,
                        &root,
                    ))
                })
        } else {
            None
        }
    }
}

fn create_location_by_symbol(
    uri: String,
    line_index: &LineIndex,
    symbol: &Symbol,
    root: &SyntaxNode,
) -> Location {
    let node = symbol.key.to_node(root);
    let range = token(&node, SyntaxKind::IDENT)
        .or_else(|| token(&node, SyntaxKind::KEYWORD))
        .map(|token| token.text_range())
        .unwrap_or_else(|| node.text_range());
    Location {
        uri,
        range: helpers::rowan_range_to_lsp_range(line_index, range),
    }
}
