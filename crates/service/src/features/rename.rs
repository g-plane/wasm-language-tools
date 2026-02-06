use crate::{
    LanguageService,
    binder::{SymbolKey, SymbolKind, SymbolTable},
    document::Document,
    helpers::LineIndexExt,
    idx::InternIdent,
};
use lspt::{PrepareRenameParams, PrepareRenameResult, RenameParams, TextEdit, WorkspaceEdit};
use rowan::ast::support;
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;
use wat_parser::is_id_char;
use wat_syntax::{SyntaxKind, SyntaxToken};

const ERR_INVALID_IDENTIFIER: &str = "not a valid identifier";
const ERR_CANT_BE_RENAMED: &str = "This can't be renamed.";

impl LanguageService {
    /// Handler for `textDocument/prepareRename` request.
    pub fn prepare_rename(&self, params: PrepareRenameParams) -> Option<PrepareRenameResult> {
        let document = self.get_document(params.text_document.uri)?;
        let line_index = document.line_index(self);
        let root = document.root_tree(self);
        let token = super::find_meaningful_token(self, document, &root, params.position)
            .filter(|token| token.kind() == SyntaxKind::IDENT)?;
        let range = line_index.convert(token.text_range());
        Some(PrepareRenameResult::A(range))
    }

    /// Handler for `textDocument/rename` request.
    pub fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>, String> {
        if !params
            .new_name
            .strip_prefix('$')
            .is_some_and(|rest| !rest.is_empty() && rest.chars().all(is_id_char))
        {
            return Err(format!("Invalid name `{}`: {ERR_INVALID_IDENTIFIER}.", params.new_name));
        }

        let Some(document) = self.get_document(&params.text_document.uri) else {
            return Ok(None);
        };
        // We can't assume client supports "prepareRename" so we need to check the token again.
        let token = super::find_meaningful_token(self, document, &document.root_tree(self), params.position)
            .filter(|token| token.kind() == SyntaxKind::IDENT)
            .ok_or_else(|| ERR_CANT_BE_RENAMED.to_owned())?;
        Ok(self.rename_impl(params, document, token))
    }

    fn rename_impl(&self, params: RenameParams, document: Document, ident_token: SyntaxToken) -> Option<WorkspaceEdit> {
        let line_index = document.line_index(self);
        let root = document.root_tree(self);
        let symbol_table = SymbolTable::of(self, document);

        let old_name = InternIdent::new(self, ident_token.text());
        let symbol_key = SymbolKey::new(&ident_token.parent()?);
        let symbol = symbol_table.symbols.get(&symbol_key)?;
        let text_edits = symbol_table
            .symbols
            .values()
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
                | SymbolKind::FieldRef
                | SymbolKind::TagDef
                | SymbolKind::TagRef
                | SymbolKind::DataDef
                | SymbolKind::DataRef => {
                    sym.region == symbol.region
                        && sym.idx_kind == symbol.idx_kind
                        && sym.idx.name.is_some_and(|name| name == old_name)
                }
                SymbolKind::BlockDef => {
                    symbol == *sym
                        || symbol_table
                            .resolved
                            .get(&symbol_key)
                            .is_some_and(|def_key| *def_key == sym.key)
                }
                SymbolKind::BlockRef => {
                    if symbol.kind == SymbolKind::BlockDef {
                        symbol_table
                            .resolved
                            .get(&sym.key)
                            .is_some_and(|def_key| *def_key == symbol_key)
                    } else if let (Some(a), Some(b)) = (
                        symbol_table.resolved.get(&symbol_key),
                        symbol_table.resolved.get(&sym.key),
                    ) {
                        a == b
                    } else {
                        false
                    }
                }
                SymbolKind::Module => false,
            })
            .filter_map(|sym| support::token(&sym.key.to_node(&root), SyntaxKind::IDENT))
            .map(|token| TextEdit {
                range: line_index.convert(token.text_range()),
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
