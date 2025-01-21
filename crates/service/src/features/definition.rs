use super::find_meaningful_token;
use crate::{
    binder::{Symbol, SymbolKey, SymbolTablesCtx},
    helpers,
    syntax_tree::SyntaxTreeCtx,
    uri::UrisCtx,
    LanguageService,
};
use line_index::LineIndex;
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location};
use rowan::ast::{
    support::{child, token},
    AstNode,
};
use wat_syntax::{ast::TypeUse, SyntaxKind, SyntaxNode};

impl LanguageService {
    /// Handler for `textDocument/definition` request.
    pub fn goto_definition(&self, params: GotoDefinitionParams) -> Option<GotoDefinitionResponse> {
        let uri = self.uri(
            params
                .text_document_position_params
                .text_document
                .uri
                .clone(),
        );
        let root = SyntaxNode::new_root(self.root(uri));
        let token = find_meaningful_token(
            self,
            uri,
            &root,
            params.text_document_position_params.position,
        )?;

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
                GotoDefinitionResponse::Scalar(create_location_by_symbol(
                    &params,
                    &line_index,
                    symbol,
                    &root,
                ))
            })
            .or_else(|| {
                symbol_table.find_defs(key).map(|symbols| {
                    GotoDefinitionResponse::Array(
                        symbols
                            .map(|symbol| {
                                create_location_by_symbol(&params, &line_index, symbol, &root)
                            })
                            .collect(),
                    )
                })
            })
            .or_else(|| {
                symbol_table
                    .find_block_def(key)
                    .and_then(|key| symbol_table.symbols.iter().find(|symbol| symbol.key == key))
                    .map(|symbol| {
                        GotoDefinitionResponse::Scalar(create_location_by_symbol(
                            &params,
                            &line_index,
                            symbol,
                            &root,
                        ))
                    })
            })
    }

    /// Handler for `textDocument/typeDefinition` request.
    pub fn goto_type_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Option<GotoDefinitionResponse> {
        let uri = self.uri(
            params
                .text_document_position_params
                .text_document
                .uri
                .clone(),
        );
        let line_index = self.line_index(uri);
        let root = SyntaxNode::new_root(self.root(uri));
        let symbol_table = self.symbol_table(uri);
        let token = find_meaningful_token(
            self,
            uri,
            &root,
            params.text_document_position_params.position,
        )?;

        let parent = token.parent()?;
        if !matches!(parent.kind(), SyntaxKind::IMMEDIATE | SyntaxKind::INDEX) {
            return None;
        }

        let grand = parent.parent()?;
        match grand.kind() {
            SyntaxKind::PLAIN_INSTR => {
                symbol_table
                    .find_defs(SymbolKey::new(&parent))
                    .map(|symbols| {
                        GotoDefinitionResponse::Array(
                            symbols
                                .filter_map(|symbol| {
                                    symbol_table.find_defs(SymbolKey::new(
                                        child::<TypeUse>(&symbol.key.to_node(&root))?
                                            .index()?
                                            .syntax(),
                                    ))
                                })
                                .flatten()
                                .map(|symbol| {
                                    create_location_by_symbol(&params, &line_index, symbol, &root)
                                })
                                .collect(),
                        )
                    })
            }
            _ => None,
        }
    }

    /// Handler for `textDocument/declaration` request.
    ///
    /// Only available for function calls currently. This behaves same as "Goto Definition".
    pub fn goto_declaration(&self, params: GotoDefinitionParams) -> Option<GotoDefinitionResponse> {
        let uri = self.uri(
            params
                .text_document_position_params
                .text_document
                .uri
                .clone(),
        );
        let line_index = self.line_index(uri);
        let root = SyntaxNode::new_root(self.root(uri));
        let symbol_table = self.symbol_table(uri);
        let token = find_meaningful_token(
            self,
            uri,
            &root,
            params.text_document_position_params.position,
        )?;
        let parent = token.parent()?;
        if parent.kind() == SyntaxKind::IMMEDIATE {
            symbol_table
                .find_defs(SymbolKey::new(&parent))
                .map(|symbols| {
                    GotoDefinitionResponse::Array(
                        symbols
                            .map(|symbol| {
                                create_location_by_symbol(&params, &line_index, symbol, &root)
                            })
                            .collect(),
                    )
                })
        } else {
            None
        }
    }
}

fn create_location_by_symbol(
    params: &GotoDefinitionParams,
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
        uri: params
            .text_document_position_params
            .text_document
            .uri
            .clone(),
        range: helpers::rowan_range_to_lsp_range(line_index, range),
    }
}
