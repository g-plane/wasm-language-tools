use super::find_meaningful_token;
use crate::{
    binder::{RefIdx, SymbolItemKind, SymbolTablesCtx},
    files::FilesCtx,
    helpers, LanguageService,
};
use lsp_types::{
    PrepareRenameResponse, RenameParams, TextDocumentPositionParams, TextEdit, WorkspaceEdit,
};
use rowan::ast::support;
use std::collections::HashMap;
use wat_parser::is_id_char;
use wat_syntax::{SyntaxKind, SyntaxNode, SyntaxToken};

const ERR_INVALID_IDENTIFIER: &str = "not a valid identifier";
const ERR_CANT_BE_RENAMED: &str = "This can't be renamed.";

impl LanguageService {
    pub fn prepare_rename(
        &self,
        params: TextDocumentPositionParams,
    ) -> Option<PrepareRenameResponse> {
        let uri = self.uri(params.text_document.uri);
        let line_index = self.line_index(uri);
        let root = SyntaxNode::new_root(self.root(uri));
        let token = find_meaningful_token(self, uri, &root, params.position)
            .filter(|token| token.kind() == SyntaxKind::IDENT)?;
        let range = helpers::rowan_range_to_lsp_range(&line_index, token.text_range());
        Some(PrepareRenameResponse::Range(range))
    }

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

        let uri = self.uri(params.text_document_position.text_document.uri.clone());
        // We can't assume client supports "prepareRename" so we need to check the token again.
        let token = find_meaningful_token(
            self,
            uri,
            &SyntaxNode::new_root(self.root(uri)),
            params.text_document_position.position,
        )
        .filter(|token| token.kind() == SyntaxKind::IDENT)
        .ok_or_else(|| ERR_CANT_BE_RENAMED.to_owned())?;
        Ok(self.rename_impl(params, token))
    }

    #[expect(clippy::mutable_key_type)]
    fn rename_impl(&self, params: RenameParams, ident_token: SyntaxToken) -> Option<WorkspaceEdit> {
        let uri = self.uri(params.text_document_position.text_document.uri.clone());
        let line_index = self.line_index(uri);
        let root = SyntaxNode::new_root(self.root(uri));
        let symbol_table = self.symbol_table(uri);

        let old_name = self.ident(ident_token.text().to_string());
        let symbol_key = ident_token.parent()?.into();
        let symbol = symbol_table
            .symbols
            .iter()
            .find(|symbol| symbol.key == symbol_key)?;
        let text_edits = symbol_table
            .symbols
            .iter()
            .filter(|sym| {
                symbol.region == sym.region
                    && match &sym.kind {
                        SymbolItemKind::Func(idx)
                        | SymbolItemKind::Param(idx)
                        | SymbolItemKind::Local(idx)
                        | SymbolItemKind::Type(idx)
                        | SymbolItemKind::GlobalDef(idx)
                        | SymbolItemKind::MemoryDef(idx) => {
                            idx.name.is_some_and(|name| name == old_name)
                        }
                        SymbolItemKind::Call(idx)
                        | SymbolItemKind::LocalRef(idx)
                        | SymbolItemKind::TypeUse(idx)
                        | SymbolItemKind::GlobalRef(idx)
                        | SymbolItemKind::MemoryRef(idx) => {
                            if let RefIdx::Name(name) = idx {
                                *name == old_name
                            } else {
                                false
                            }
                        }
                        SymbolItemKind::Module => false,
                    }
            })
            .filter_map(|sym| support::token(&sym.key.ptr.to_node(&root), SyntaxKind::IDENT))
            .map(|token| TextEdit {
                range: helpers::rowan_range_to_lsp_range(&line_index, token.text_range()),
                new_text: params.new_name.clone(),
            })
            .collect();

        let mut changes = HashMap::new();
        changes.insert(params.text_document_position.text_document.uri, text_edits);
        Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        })
    }
}
