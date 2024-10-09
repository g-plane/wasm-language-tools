use super::find_meaningful_token;
use crate::{
    binder::{SymbolItem, SymbolItemKind, SymbolTablesCtx},
    files::FilesCtx,
    helpers, LanguageService,
};
use line_index::LineIndex;
use lsp_types::{Location, ReferenceParams};
use rowan::ast::support::token;
use wat_syntax::{SyntaxKind, SyntaxNode};

impl LanguageService {
    pub fn find_references(&self, params: ReferenceParams) -> Option<Vec<Location>> {
        let uri = self
            .ctx
            .uri(params.text_document_position.text_document.uri.clone());
        let token = find_meaningful_token(&self.ctx, uri, params.text_document_position.position)?;
        let parent = token.parent()?;

        let line_index = self.ctx.line_index(uri);
        let root = self.ctx.root(uri);
        let symbol_table = self.ctx.symbol_table(uri);

        let key = parent.into();
        let current_symbol = symbol_table
            .symbols
            .iter()
            .find(|symbol| symbol.key == key)?;
        Some(match &current_symbol.kind {
            SymbolItemKind::Module => vec![],
            SymbolItemKind::Func(def_idx) => symbol_table
                .symbols
                .iter()
                .filter(|symbol| match &symbol.kind {
                    SymbolItemKind::Func(idx) if params.context.include_declaration => {
                        def_idx == idx && symbol.region == current_symbol.region
                    }
                    SymbolItemKind::Call(idx) => {
                        def_idx == idx && symbol.region == current_symbol.region
                    }
                    _ => false,
                })
                .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                .collect(),
            SymbolItemKind::Param(def_idx) => symbol_table
                .symbols
                .iter()
                .filter(|symbol| match &symbol.kind {
                    SymbolItemKind::Param(idx) if params.context.include_declaration => {
                        def_idx == idx && symbol.region == current_symbol.region
                    }
                    SymbolItemKind::LocalRef(idx) => {
                        def_idx == idx && symbol.region == current_symbol.region
                    }
                    _ => false,
                })
                .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                .collect(),
            SymbolItemKind::Local(def_idx) => symbol_table
                .symbols
                .iter()
                .filter(|symbol| match &symbol.kind {
                    SymbolItemKind::Local(idx) if params.context.include_declaration => {
                        def_idx == idx && symbol.region == current_symbol.region
                    }
                    SymbolItemKind::LocalRef(idx) => {
                        def_idx == idx && symbol.region == current_symbol.region
                    }
                    _ => false,
                })
                .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                .collect(),
            SymbolItemKind::Call(ref_idx) => symbol_table
                .symbols
                .iter()
                .filter(|symbol| match &symbol.kind {
                    SymbolItemKind::Func(idx) if params.context.include_declaration => {
                        ref_idx == idx && symbol.region == current_symbol.region
                    }
                    SymbolItemKind::Call(idx) => {
                        ref_idx == idx && symbol.region == current_symbol.region
                    }
                    _ => false,
                })
                .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                .collect(),
            SymbolItemKind::LocalRef(ref_idx) => symbol_table
                .symbols
                .iter()
                .filter(|symbol| match &symbol.kind {
                    SymbolItemKind::Param(idx) | SymbolItemKind::Local(idx)
                        if params.context.include_declaration =>
                    {
                        ref_idx == idx && symbol.region == current_symbol.region
                    }
                    SymbolItemKind::LocalRef(idx) => {
                        ref_idx == idx && symbol.region == current_symbol.region
                    }
                    _ => false,
                })
                .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                .collect(),
            SymbolItemKind::Type(def_idx) => symbol_table
                .symbols
                .iter()
                .filter(|symbol| match &symbol.kind {
                    SymbolItemKind::Type(idx) if params.context.include_declaration => {
                        def_idx == idx && symbol.region == current_symbol.region
                    }
                    SymbolItemKind::TypeUse(idx) => {
                        def_idx == idx && symbol.region == current_symbol.region
                    }
                    _ => false,
                })
                .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                .collect(),
            SymbolItemKind::TypeUse(ref_idx) => symbol_table
                .symbols
                .iter()
                .filter(|symbol| match &symbol.kind {
                    SymbolItemKind::Type(idx) if params.context.include_declaration => {
                        ref_idx == idx && symbol.region == current_symbol.region
                    }
                    SymbolItemKind::TypeUse(idx) => {
                        ref_idx == idx && symbol.region == current_symbol.region
                    }
                    _ => false,
                })
                .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                .collect(),
            SymbolItemKind::GlobalDef(def_idx) => symbol_table
                .symbols
                .iter()
                .filter(|symbol| match &symbol.kind {
                    SymbolItemKind::GlobalDef(idx) if params.context.include_declaration => {
                        def_idx == idx && symbol.region == current_symbol.region
                    }
                    SymbolItemKind::GlobalRef(idx) => {
                        def_idx == idx && symbol.region == current_symbol.region
                    }
                    _ => false,
                })
                .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                .collect(),
            SymbolItemKind::GlobalRef(ref_idx) => symbol_table
                .symbols
                .iter()
                .filter(|symbol| match &symbol.kind {
                    SymbolItemKind::GlobalDef(idx) if params.context.include_declaration => {
                        ref_idx == idx && symbol.region == current_symbol.region
                    }
                    SymbolItemKind::GlobalRef(idx) => {
                        ref_idx == idx && symbol.region == current_symbol.region
                    }
                    _ => false,
                })
                .map(|symbol| create_location_by_symbol(&params, &line_index, symbol, &root))
                .collect(),
        })
    }
}

fn create_location_by_symbol(
    params: &ReferenceParams,
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
        uri: params.text_document_position.text_document.uri.clone(),
        range: helpers::rowan_range_to_lsp_range(line_index, range),
    }
}