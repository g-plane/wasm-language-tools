use crate::document::Document;
use rowan::TextRange;
use smallvec::SmallVec;
use wat_syntax::SyntaxKind;

#[salsa::tracked(returns(ref))]
pub(crate) fn get_exports<'db>(
    db: &'db dyn salsa::Database,
    document: Document,
) -> SmallVec<[Vec<Export>; 1]> {
    document
        .root_tree(db)
        .children()
        .map(|module| {
            module
                .children()
                .filter_map(|module_field| {
                    if module_field.kind() == SyntaxKind::MODULE_FIELD_EXPORT {
                        Some(module_field)
                    } else {
                        module_field
                            .children()
                            .find(|child| child.kind() == SyntaxKind::EXPORT)
                    }
                })
                .filter_map(|node| {
                    node.children()
                        .find(|child| child.kind() == SyntaxKind::NAME)
                })
                .map(|name| Export {
                    name: name.to_string(),
                    range: name.text_range(),
                })
                .collect()
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub(crate) struct Export {
    pub name: String, // including double quotes
    pub range: TextRange,
}
