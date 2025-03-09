use super::find_meaningful_token;
use crate::{
    binder::{SymbolKey, SymbolKind, SymbolTablesCtx},
    helpers,
    idx::IdentsCtx,
    syntax_tree::SyntaxTreeCtx,
    uri::UrisCtx,
    LanguageService,
};
use lspt::{PrepareRenameParams, PrepareRenameResult, RenameParams, TextEdit, WorkspaceEdit};
use rowan::ast::support;
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;
use wat_parser::is_id_char;
use wat_syntax::{SyntaxKind, SyntaxNode, SyntaxToken};

const ERR_INVALID_IDENTIFIER: &str = "not a valid identifier";
const ERR_CANT_BE_RENAMED: &str = "This can't be renamed.";

impl LanguageService {
    /// Handler for `textDocument/prepareRename` request.
    pub fn prepare_rename(&self, params: PrepareRenameParams) -> Option<PrepareRenameResult> {
        let uri = self.uri(params.text_document.uri);
        let line_index = self.line_index(uri);
        let root = SyntaxNode::new_root(self.root(uri));
        let token = find_meaningful_token(self, uri, &root, params.position)
            .filter(|token| token.kind() == SyntaxKind::IDENT)?;
        let range = helpers::rowan_range_to_lsp_range(&line_index, token.text_range());
        Some(PrepareRenameResult::A(range))
    }

    /// Handler for `textDocument/rename` request.
    pub fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>, String> {
        if !params
            .new_name
            .strip_prefix('$')
            .is_some_and(|rest| !rest.is_empty() && rest.chars().all(is_id_char))
        {
            return Err(format!(
                "Invalid name `{}`: {ERR_INVALID_IDENTIFIER}.",
                params.new_name
            ));
        }

        let uri = self.uri(params.text_document.uri.clone());
        // We can't assume client supports "prepareRename" so we need to check the token again.
        let token = find_meaningful_token(
            self,
            uri,
            &SyntaxNode::new_root(self.root(uri)),
            params.position,
        )
        .filter(|token| token.kind() == SyntaxKind::IDENT)
        .ok_or_else(|| ERR_CANT_BE_RENAMED.to_owned())?;
        Ok(self.rename_impl(params, token))
    }

    fn rename_impl(&self, params: RenameParams, ident_token: SyntaxToken) -> Option<WorkspaceEdit> {
        let uri = self.uri(params.text_document.uri.clone());
        let line_index = self.line_index(uri);
        let root = SyntaxNode::new_root(self.root(uri));
        let symbol_table = self.symbol_table(uri);

        let old_name = self.ident(ident_token.text().into());
        let symbol_key = SymbolKey::new(&ident_token.parent()?);
        let symbol = symbol_table
            .symbols
            .iter()
            .find(|symbol| symbol.key == symbol_key)?;
        let text_edits = symbol_table
            .symbols
            .iter()
            .filter(|sym| match &sym.kind {
                SymbolKind::Func
                | SymbolKind::Call
                | SymbolKind::Param
                | SymbolKind::Local
                | SymbolKind::LocalRef
                | SymbolKind::Type
                | SymbolKind::TypeUse
                | SymbolKind::GlobalDef
                | SymbolKind::GlobalRef
                | SymbolKind::MemoryDef
                | SymbolKind::MemoryRef
                | SymbolKind::TableDef
                | SymbolKind::TableRef
                | SymbolKind::FieldDef
                | SymbolKind::FieldRef => {
                    sym.region == symbol.region && sym.idx.name.is_some_and(|name| name == old_name)
                }
                SymbolKind::BlockDef => {
                    symbol == *sym
                        || symbol_table
                            .blocks
                            .iter()
                            .any(|block| symbol_key == block.ref_key && block.def_key == sym.key)
                }
                SymbolKind::BlockRef => {
                    symbol == *sym
                        || symbol_table
                            .blocks
                            .iter()
                            .any(|block| sym.key == block.ref_key && block.def_key == symbol.key)
                        || symbol_table
                            .find_block_def(symbol_key)
                            .is_some_and(|def_key| {
                                symbol_table.blocks.iter().any(|block| {
                                    sym.key == block.ref_key && block.def_key == def_key
                                })
                            })
                }
                SymbolKind::Module => false,
            })
            .filter_map(|sym| support::token(&sym.key.to_node(&root), SyntaxKind::IDENT))
            .map(|token| TextEdit {
                range: helpers::rowan_range_to_lsp_range(&line_index, token.text_range()),
                new_text: params.new_name.clone(),
            })
            .collect();

        let mut changes = HashMap::with_capacity_and_hasher(1, FxBuildHasher);
        changes.insert(params.text_document.uri, text_edits);
        Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        })
    }
}
