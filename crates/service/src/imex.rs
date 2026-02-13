use crate::{
    binder::{SymbolKey, SymbolTable},
    document::Document,
    helpers,
};
use rustc_hash::FxHashMap;
use wat_syntax::{SyntaxKind, SyntaxNodePtr, TextRange};

#[salsa::tracked(returns(ref))]
pub(crate) fn get_imports(db: &dyn salsa::Database, document: Document) -> Vec<SymbolKey> {
    SymbolTable::of(db, document)
        .symbols
        .iter()
        .filter(|(_, symbol)| match symbol.key.kind() {
            SyntaxKind::EXTERN_TYPE_FUNC
            | SyntaxKind::EXTERN_TYPE_GLOBAL
            | SyntaxKind::EXTERN_TYPE_MEMORY
            | SyntaxKind::EXTERN_TYPE_TABLE
            | SyntaxKind::EXTERN_TYPE_TAG => true,
            SyntaxKind::MODULE_FIELD_FUNC
            | SyntaxKind::MODULE_FIELD_GLOBAL
            | SyntaxKind::MODULE_FIELD_MEMORY
            | SyntaxKind::MODULE_FIELD_TABLE
            | SyntaxKind::MODULE_FIELD_TAG => symbol.green.children().any(|child| child.kind() == SyntaxKind::IMPORT),
            _ => false,
        })
        .map(|(key, _)| *key)
        .collect()
}

pub(crate) type ExportMap = FxHashMap<SyntaxNodePtr, Vec<Export>>;

#[salsa::tracked(returns(ref))]
pub(crate) fn get_exports(db: &dyn salsa::Database, document: Document) -> ExportMap {
    let symbol_table = SymbolTable::of(db, document);
    document
        .root_tree(db)
        .children()
        .map(|module| {
            let mut exports = Vec::new();
            module.children().for_each(|module_field| {
                if module_field.kind() == SyntaxKind::MODULE_FIELD_EXPORT {
                    if let Some(name) = module_field.first_child_by_kind(|kind| kind == SyntaxKind::NAME)
                        && let Some(def_key) = helpers::syntax::extract_index_from_export(&module_field)
                            .and_then(|index| symbol_table.resolved.get(&SymbolKey::new(&index)))
                    {
                        exports.push(Export {
                            def_key: *def_key,
                            name: name.to_string(),
                            range: name.text_range(),
                        });
                    }
                } else {
                    exports.extend(
                        module_field
                            .children_by_kind(|kind| kind == SyntaxKind::EXPORT)
                            .filter_map(|export| export.first_child_by_kind(|kind| kind == SyntaxKind::NAME))
                            .map(|name| Export {
                                def_key: SymbolKey::new(&module_field),
                                name: name.to_string(),
                                range: name.text_range(),
                            }),
                    );
                }
            });
            (SyntaxNodePtr::new(&module), exports)
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Export {
    pub def_key: SymbolKey,
    /// Export name contains double quotes.
    pub name: String,
    pub range: TextRange,
}
