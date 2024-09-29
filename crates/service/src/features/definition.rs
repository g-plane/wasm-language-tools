use super::{find_meaningful_token, is_call, locate_module};
use crate::{
    binder::{SymbolItem, SymbolItemKind, SymbolTable, SymbolTablesCtx},
    files::FilesCtx,
    helpers, LanguageService,
};
use either::Either;
use line_index::LineIndex;
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location};
use rowan::ast::{support::child, AstNode};
use wat_syntax::{ast::TypeUse, SyntaxKind, SyntaxToken};

impl LanguageService {
    pub fn goto_definition(&self, params: GotoDefinitionParams) -> Option<GotoDefinitionResponse> {
        let uri = self.ctx.uri(
            params
                .text_document_position_params
                .text_document
                .uri
                .clone(),
        );
        let line_index = self.ctx.line_index(uri);
        let symbol_table = self.ctx.symbol_table(uri);
        let token = find_meaningful_token(
            &self.ctx,
            uri,
            params.text_document_position_params.position,
        )?;

        let grand = token.parent()?.parent()?;
        match grand.kind() {
            SyntaxKind::PLAIN_INSTR => find_func_def(&symbol_table, token).map(|symbols| {
                GotoDefinitionResponse::Array(
                    symbols
                        .map(|symbol| create_location_by_symbol(&params, &line_index, symbol))
                        .collect(),
                )
            }),
            SyntaxKind::TYPE_USE => find_type_use_def(&symbol_table, token).map(|symbols| {
                GotoDefinitionResponse::Array(
                    symbols
                        .map(|symbol| create_location_by_symbol(&params, &line_index, symbol))
                        .collect(),
                )
            }),
            _ => None,
        }
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

        let grand = token.parent()?.parent()?;
        match grand.kind() {
            SyntaxKind::PLAIN_INSTR => find_func_def(&symbol_table, token).map(|symbols| {
                GotoDefinitionResponse::Array(
                    symbols
                        .filter_map(|symbol| {
                            let token = child::<TypeUse>(&symbol.key.ptr.to_node(&root))?
                                .index()?
                                .syntax()
                                .first_token()?;
                            find_type_use_def(&symbol_table, token)
                        })
                        .flatten()
                        .map(|symbol| create_location_by_symbol(&params, &line_index, symbol))
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
        let symbol_table = self.ctx.symbol_table(uri);
        let token = find_meaningful_token(
            &self.ctx,
            uri,
            params.text_document_position_params.position,
        )?;

        let grand = token.parent()?.parent()?;
        match grand.kind() {
            SyntaxKind::PLAIN_INSTR => find_func_def(&symbol_table, token).map(|symbols| {
                GotoDefinitionResponse::Array(
                    symbols
                        .map(|symbol| create_location_by_symbol(&params, &line_index, symbol))
                        .collect(),
                )
            }),
            _ => None,
        }
    }
}

fn find_func_def(
    symbol_table: &SymbolTable,
    token: SyntaxToken,
) -> Option<Either<impl Iterator<Item = &SymbolItem>, impl Iterator<Item = &SymbolItem>>> {
    let grand = token.parent()?.parent()?;
    if grand.kind() == SyntaxKind::PLAIN_INSTR {
        if !is_call(&grand) {
            return None;
        }
        let module = locate_module(symbol_table, token.parent_ancestors())?;
        match token.kind() {
            SyntaxKind::IDENT => Some(Either::Left(symbol_table.symbols.iter().filter(
                move |symbol| {
                    if let SymbolItemKind::Func(func) = &symbol.kind {
                        symbol
                            .parent
                            .as_ref()
                            .is_some_and(|parent| parent == &module.key)
                            && func
                                .name
                                .as_deref()
                                .is_some_and(|name| name == token.text())
                    } else {
                        false
                    }
                },
            ))),
            SyntaxKind::INT => {
                let num: usize = token.text().parse().ok()?;
                Some(Either::Right(symbol_table.symbols.iter().filter(
                    move |symbol| {
                        if let SymbolItemKind::Func(func) = &symbol.kind {
                            symbol
                                .parent
                                .as_ref()
                                .is_some_and(|parent| parent == &module.key)
                                && func.num == num
                        } else {
                            false
                        }
                    },
                )))
            }
            _ => None,
        }
    } else {
        None
    }
}

fn find_type_use_def(
    symbol_table: &SymbolTable,
    token: SyntaxToken,
) -> Option<Either<impl Iterator<Item = &SymbolItem>, impl Iterator<Item = &SymbolItem>>> {
    let module = locate_module(symbol_table, token.parent_ancestors())?;
    match token.kind() {
        SyntaxKind::IDENT => Some(Either::Left(symbol_table.symbols.iter().filter(
            move |symbol| {
                if let SymbolItemKind::Type(ty) = &symbol.kind {
                    symbol
                        .parent
                        .as_ref()
                        .is_some_and(|parent| parent == &module.key)
                        && ty.name.as_deref().is_some_and(|name| name == token.text())
                } else {
                    false
                }
            },
        ))),
        SyntaxKind::INT => {
            let num: usize = token.text().parse().ok()?;
            Some(Either::Right(symbol_table.symbols.iter().filter(
                move |symbol| {
                    if let SymbolItemKind::Type(ty) = &symbol.kind {
                        symbol
                            .parent
                            .as_ref()
                            .is_some_and(|parent| parent == &module.key)
                            && ty.num == num
                    } else {
                        false
                    }
                },
            )))
        }
        _ => None,
    }
}

fn create_location_by_symbol(
    params: &GotoDefinitionParams,
    line_index: &LineIndex,
    symbol: &SymbolItem,
) -> Location {
    Location {
        uri: params
            .text_document_position_params
            .text_document
            .uri
            .clone(),
        range: helpers::rowan_range_to_lsp_range(line_index, symbol.key.ptr.text_range()),
    }
}
