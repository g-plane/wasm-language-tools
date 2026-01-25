use crate::{
    LanguageService,
    binder::{SymbolKey, SymbolKind, SymbolTable},
    helpers,
};
use lspt::{Location, ReferenceParams};
use wat_syntax::SyntaxKind;

impl LanguageService {
    /// Handler for `textDocument/references` request.
    pub fn find_references(&self, params: ReferenceParams) -> Option<Vec<Location>> {
        let uri = params.text_document.uri;
        let document = self.get_document(&uri)?;
        let root = document.root_tree(self);
        let token = super::find_meaningful_token(self, document, &root, params.position)?;
        if !matches!(
            token.kind(),
            SyntaxKind::IDENT
                | SyntaxKind::INT
                | SyntaxKind::UNSIGNED_INT
                | SyntaxKind::TYPE_KEYWORD
                | SyntaxKind::KEYWORD
        ) {
            return None;
        }
        let parent = token.parent()?;
        let grand = parent.parent();
        let current_node = match grand {
            Some(grand) if grand.kind() == SyntaxKind::FIELD_TYPE => grand,
            _ => parent,
        };

        let line_index = document.line_index(self);
        let symbol_table = SymbolTable::of(self, document);

        let key = SymbolKey::new(&current_node);
        let symbol = symbol_table.symbols.get(&key)?;
        match symbol.kind {
            SymbolKind::Module => None,
            SymbolKind::Func
            | SymbolKind::Param
            | SymbolKind::Local
            | SymbolKind::Type
            | SymbolKind::GlobalDef
            | SymbolKind::MemoryDef
            | SymbolKind::TableDef
            | SymbolKind::FieldDef
            | SymbolKind::TagDef => Some(
                symbol_table
                    .find_references_on_def(symbol, params.context.include_declaration)
                    .map(|symbol| helpers::create_location_by_symbol(uri.clone(), line_index, symbol.key, &root))
                    .collect(),
            ),
            SymbolKind::Call
            | SymbolKind::LocalRef
            | SymbolKind::TypeUse
            | SymbolKind::GlobalRef
            | SymbolKind::MemoryRef
            | SymbolKind::TableRef
            | SymbolKind::FieldRef
            | SymbolKind::TagRef => Some(
                symbol_table
                    .find_references_on_ref(symbol, params.context.include_declaration)
                    .map(|symbol| helpers::create_location_by_symbol(uri.clone(), line_index, symbol.key, &root))
                    .collect(),
            ),
            SymbolKind::BlockDef => Some(
                symbol_table
                    .find_block_references(key, params.context.include_declaration)
                    .map(|symbol| helpers::create_location_by_symbol(uri.clone(), line_index, symbol.key, &root))
                    .collect(),
            ),
            SymbolKind::BlockRef => symbol_table.resolved.get(&key).map(|def_key| {
                symbol_table
                    .find_block_references(*def_key, params.context.include_declaration)
                    .map(|symbol| helpers::create_location_by_symbol(uri.clone(), line_index, symbol.key, &root))
                    .collect()
            }),
        }
        .map(|mut references: Vec<_>| {
            references.sort_unstable_by_key(|location| location.range.start);
            references
        })
    }
}
