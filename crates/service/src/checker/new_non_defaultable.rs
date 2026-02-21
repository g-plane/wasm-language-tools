use super::{Diagnostic, DiagnosticCtx, RelatedInformation};
use crate::{
    binder::SymbolKind,
    types_analyzer::{CompositeType, FieldType, Fields, StorageType},
};
use wat_syntax::{AmberNode, AmberToken, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "new-non-defaultable";

pub fn check(ctx: &DiagnosticCtx, node: AmberNode, instr_name: AmberToken) -> Option<Diagnostic> {
    if !instr_name.text().ends_with(".new_default") {
        return None;
    }
    let immediate = node.children_by_kind(SyntaxKind::IMMEDIATE).next()?;
    let def_symbol = ctx.symbol_table.find_def(immediate.to_ptr().into())?;
    match &ctx.def_types.get(&def_symbol.key)?.comp {
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
                    message: format!("struct type `{}` is not defaultable", def_symbol.idx.render(ctx.db)),
                    related_information: Some(
                        non_defaultables
                            .into_iter()
                            .filter_map(|idx| {
                                ctx.symbol_table
                                    .symbols
                                    .values()
                                    .find(|symbol| {
                                        symbol.kind == SymbolKind::FieldDef
                                            && symbol.region == def_symbol.key
                                            && &symbol.idx == idx
                                    })
                                    .map(|symbol| RelatedInformation {
                                        range: symbol.key.text_range(),
                                        message: format!(
                                            "field type `{}` is not defaultable",
                                            symbol.idx.render(ctx.db)
                                        ),
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
            message: format!("array type `{}` is not defaultable", def_symbol.idx.render(ctx.db)),
            ..Default::default()
        }),
        _ => None,
    }
}
