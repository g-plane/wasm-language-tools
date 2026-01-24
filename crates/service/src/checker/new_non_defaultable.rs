use super::{Diagnostic, RelatedInformation};
use crate::{
    binder::{SymbolKey, SymbolKind, SymbolTable},
    document::Document,
    types_analyzer::{self, CompositeType, FieldType, Fields, StorageType},
};
use rowan::ast::support;
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "new-non-defaultable";

pub fn check(
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) -> Option<Diagnostic> {
    let def_types = types_analyzer::get_def_types(db, document);
    let immediate = node.first_child()?;
    let instr_name = support::token(node, SyntaxKind::INSTR_NAME)?;
    if !instr_name.text().ends_with(".new_default") {
        return None;
    }
    let def_symbol = symbol_table.find_def(SymbolKey::new(&immediate))?;
    match &def_types.get(&def_symbol.key)?.comp {
        CompositeType::Struct(Fields(fields)) => {
            let non_defaultables = fields
                .iter()
                .filter_map(|field| match field {
                    (
                        FieldType {
                            storage: StorageType::Val(ty),
                            ..
                        },
                        idx,
                    ) if !ty.defaultable() => Some(idx),
                    _ => None,
                })
                .collect::<Vec<_>>();
            if non_defaultables.is_empty() {
                None
            } else {
                Some(Diagnostic {
                    range: immediate.text_range(),
                    code: DIAGNOSTIC_CODE.into(),
                    message: format!("struct type `{}` is not defaultable", def_symbol.idx.render(db)),
                    related_information: Some(
                        non_defaultables
                            .into_iter()
                            .filter_map(|idx| {
                                symbol_table
                                    .symbols
                                    .values()
                                    .find(|symbol| {
                                        symbol.kind == SymbolKind::FieldDef
                                            && symbol.region == def_symbol.key
                                            && &symbol.idx == idx
                                    })
                                    .map(|symbol| RelatedInformation {
                                        range: symbol.key.text_range(),
                                        message: format!("field type `{}` is not defaultable", symbol.idx.render(db)),
                                    })
                            })
                            .collect(),
                    ),
                    ..Default::default()
                })
            }
        }
        CompositeType::Array(Some(FieldType {
            storage: StorageType::Val(ty),
            ..
        })) if !ty.defaultable() => Some(Diagnostic {
            range: immediate.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: format!("array type `{}` is not defaultable", def_symbol.idx.render(db)),
            ..Default::default()
        }),
        _ => None,
    }
}
