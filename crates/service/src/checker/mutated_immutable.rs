use super::{Diagnostic, RelatedInformation};
use crate::{
    binder::{SymbolKind, SymbolTable},
    document::Document,
    mutability,
};

const DIAGNOSTIC_CODE: &str = "mutated-immutable";

pub fn check(
    db: &dyn salsa::Database,
    diagnostics: &mut Vec<Diagnostic>,
    document: Document,
    symbol_table: &SymbolTable,
) {
    let mutabilities = mutability::get_mutabilities(db, document);
    let mutation_actions = mutability::get_mutation_actions(db, document);
    diagnostics.extend(
        mutation_actions
            .iter()
            .filter(|(_, action)| action.kind == mutability::MutationActionKind::Set)
            .filter_map(|(key, action)| {
                action
                    .target
                    .and_then(|target| mutabilities.get_key_value(&target))
                    .filter(|(_, mutability)| mutability.mut_keyword.is_none())
                    .zip(symbol_table.symbols.get(key))
                    .map(|((def_key, _), ref_symbol)| {
                        let kind = match ref_symbol.kind {
                            SymbolKind::GlobalRef => "global",
                            SymbolKind::TypeUse => "array",
                            SymbolKind::FieldRef => "field",
                            _ => unreachable!(),
                        };
                        Diagnostic {
                            range: key.text_range(),
                            code: DIAGNOSTIC_CODE.into(),
                            message: format!(
                                "mutating the immutable {kind} `{}` is not allowed",
                                ref_symbol.idx.render(db)
                            ),
                            related_information: Some(vec![RelatedInformation {
                                range: def_key.text_range(),
                                message: format!("immutable {kind} defined here"),
                            }]),
                            ..Default::default()
                        }
                    })
            }),
    );
}
