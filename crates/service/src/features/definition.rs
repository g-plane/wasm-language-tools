use super::find_meaningful_token;
use crate::{
    binder::{SymbolItem, SymbolTablesCtx},
    files::FilesCtx,
    helpers, LanguageService,
};
use line_index::LineIndex;
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location};
use rowan::ast::{
    support::{child, token},
    AstNode,
};
use wat_syntax::{ast::TypeUse, SyntaxKind, SyntaxNode};

impl LanguageService {
    pub fn goto_definition(&self, params: GotoDefinitionParams) -> Option<GotoDefinitionResponse> {
        let uri = self.ctx.uri(
            params
                .text_document_position_params
                .text_document
                .uri
                .clone(),
        );
        let token = find_meaningful_token(
            &self.ctx,
            uri,
            params.text_document_position_params.position,
        )?;

        let parent = token.parent()?;
        if !matches!(parent.kind(), SyntaxKind::OPERAND | SyntaxKind::INDEX) {
            return None;
        }

        let root = self.ctx.root(uri);
        let symbol_table = self.ctx.symbol_table(uri);
        let line_index = self.ctx.line_index(uri);
        let key = parent.clone().into();
        symbol_table
            .find_func_defs(&key)
            .map(|symbols| {
                GotoDefinitionResponse::Array(
                    symbols
                        .map(|symbol| {
                            create_location_by_symbol(&params, &line_index, symbol, &root)
                        })
                        .collect(),
                )
            })
            .or_else(|| {
                symbol_table
                    .find_param_def(&key)
                    .or_else(|| symbol_table.find_local_def(&key))
                    .map(|symbol| {
                        GotoDefinitionResponse::Scalar(create_location_by_symbol(
                            &params,
                            &line_index,
                            symbol,
                            &root,
                        ))
                    })
            })
            .or_else(|| {
                symbol_table.find_type_use_defs(&key).map(|symbols| {
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
                symbol_table.find_global_defs(&key).map(|symbols| {
                    GotoDefinitionResponse::Array(
                        symbols
                            .map(|symbol| {
                                create_location_by_symbol(&params, &line_index, symbol, &root)
                            })
                            .collect(),
                    )
                })
            })
    }

    pub fn goto_type_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Option<GotoDefinitionResponse> {
        let uri = self.ctx.uri(
            params
                .text_document_position_params
                .text_document
                .uri
                .clone(),
        );
        let line_index = self.ctx.line_index(uri);
        let root = self.ctx.root(uri);
        let symbol_table = self.ctx.symbol_table(uri);
        let token = find_meaningful_token(
            &self.ctx,
            uri,
            params.text_document_position_params.position,
        )?;

        let parent = token.parent()?;
        let grand = parent.parent()?;
        match grand.kind() {
            SyntaxKind::PLAIN_INSTR => symbol_table.find_func_defs(&parent.into()).map(|symbols| {
                GotoDefinitionResponse::Array(
                    symbols
                        .filter_map(|symbol| {
                            symbol_table.find_type_use_defs(
                                &child::<TypeUse>(&symbol.key.ptr.to_node(&root))?
                                    .index()?
                                    .syntax()
                                    .clone()
                                    .into(),
                            )
                        })
                        .flatten()
                        .map(|symbol| {
                            create_location_by_symbol(&params, &line_index, symbol, &root)
                        })
                        .collect(),
                )
            }),
            _ => None,
        }
    }

    /// Only available for function calls currently. This behaves same as "Goto Definition".
    pub fn goto_declaration(&self, params: GotoDefinitionParams) -> Option<GotoDefinitionResponse> {
        let uri = self.ctx.uri(
            params
                .text_document_position_params
                .text_document
                .uri
                .clone(),
        );
        let line_index = self.ctx.line_index(uri);
        let root = self.ctx.root(uri);
        let symbol_table = self.ctx.symbol_table(uri);
        let token = find_meaningful_token(
            &self.ctx,
            uri,
            params.text_document_position_params.position,
        )?;
        let parent = token.parent()?;
        if parent.kind() == SyntaxKind::OPERAND {
            symbol_table
                .find_func_defs(&parent.clone().into())
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
    symbol: &SymbolItem,
    root: &SyntaxNode,
) -> Location {
    let node = symbol.key.ptr.to_node(root);
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
