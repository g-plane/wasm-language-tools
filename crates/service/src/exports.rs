use crate::{
    binder::{SymbolKey, SymbolTable},
    document::Document,
};
use rowan::{TextRange, ast::AstNode};
use rustc_hash::FxHashMap;
use wat_syntax::{SyntaxKind, SyntaxNodePtr, ast::ExternIdx};

#[salsa::tracked(returns(ref))]
pub(crate) fn get_exports<'db>(
    db: &'db dyn salsa::Database,
    document: Document,
) -> FxHashMap<SyntaxNodePtr, Vec<Export>> {
    let symbol_table = SymbolTable::of(db, document);
    document
        .root_tree(db)
        .children()
        .map(|module| {
            (
                SyntaxNodePtr::new(&module),
                module
                    .children()
                    .filter_map(|module_field| {
                        if module_field.kind() == SyntaxKind::MODULE_FIELD_EXPORT {
                            let name = module_field
                                .first_child_by_kind(&|kind| kind == SyntaxKind::NAME)?;
                            module_field
                                .first_child_by_kind(&ExternIdx::can_cast)
                                .and_then(|extern_idx| {
                                    extern_idx
                                        .first_child_by_kind(&|kind| kind == SyntaxKind::INDEX)
                                })
                                .and_then(|index| {
                                    symbol_table.resolved.get(&SymbolKey::new(&index))
                                })
                                .map(|def_key| Export {
                                    def_key: *def_key,
                                    name: name.to_string(),
                                    range: name.text_range(),
                                })
                        } else {
                            module_field
                                .first_child_by_kind(&|kind| kind == SyntaxKind::EXPORT)
                                .and_then(|export| {
                                    export.first_child_by_kind(&|kind| kind == SyntaxKind::NAME)
                                })
                                .map(|name| Export {
                                    def_key: SymbolKey::new(&module_field),
                                    name: name.to_string(),
                                    range: name.text_range(),
                                })
                        }
                    })
                    .collect(),
            )
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
