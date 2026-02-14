use super::{Diagnostic, FastPlainInstr, RelatedInformation};
use crate::{
    binder::{Symbol, SymbolKey, SymbolTable},
    document::Document,
    types_analyzer::{self, CompositeType, DefTypes, FieldType, Fields, StorageType},
};
use wat_syntax::SyntaxKind;

const DIAGNOSTIC_CODE: &str = "packing";

pub fn check(
    db: &dyn salsa::Database,
    document: Document,
    symbol_table: &SymbolTable,
    instr: &FastPlainInstr,
) -> Option<Diagnostic> {
    match instr.name.text() {
        "struct.get" => {
            let def_types = types_analyzer::get_def_types(db, document);
            if let Some((_, symbol)) =
                find_struct_field(symbol_table, def_types, instr).filter(|(ty, _)| ty.is_packed())
            {
                Some(Diagnostic {
                    range: symbol.key.text_range(),
                    code: DIAGNOSTIC_CODE.into(),
                    message: format!("field `{}` is packed", symbol.idx.render(db)),
                    related_information: Some(vec![RelatedInformation {
                        range: instr.name.text_range(),
                        message: "use `struct.get_s` or `struct.get_u` instead".into(),
                    }]),
                    ..Default::default()
                })
            } else {
                None
            }
        }
        "struct.get_s" | "struct.get_u" => {
            let def_types = types_analyzer::get_def_types(db, document);
            if let Some((_, symbol)) =
                find_struct_field(symbol_table, def_types, instr).filter(|(ty, _)| !ty.is_packed())
            {
                Some(Diagnostic {
                    range: symbol.key.text_range(),
                    code: DIAGNOSTIC_CODE.into(),
                    message: format!("field `{}` is unpacked", symbol.idx.render(db)),
                    related_information: Some(vec![RelatedInformation {
                        range: instr.name.text_range(),
                        message: "use `struct.get` instead".into(),
                    }]),
                    ..Default::default()
                })
            } else {
                None
            }
        }
        "array.get" => {
            let def_types = types_analyzer::get_def_types(db, document);
            if let Some((_, symbol)) = find_array(symbol_table, def_types, instr).filter(|(ty, _)| ty.is_packed()) {
                Some(Diagnostic {
                    range: symbol.key.text_range(),
                    code: DIAGNOSTIC_CODE.into(),
                    message: format!("array `{}` is packed", symbol.idx.render(db)),
                    related_information: Some(vec![RelatedInformation {
                        range: instr.name.text_range(),
                        message: "use `array.get_s` or `array.get_u` instead".into(),
                    }]),
                    ..Default::default()
                })
            } else {
                None
            }
        }
        "array.get_s" | "array.get_u" => {
            let def_types = types_analyzer::get_def_types(db, document);
            if let Some((_, symbol)) = find_array(symbol_table, def_types, instr).filter(|(ty, _)| !ty.is_packed()) {
                Some(Diagnostic {
                    range: symbol.key.text_range(),
                    code: DIAGNOSTIC_CODE.into(),
                    message: format!("array `{}` is unpacked", symbol.idx.render(db)),
                    related_information: Some(vec![RelatedInformation {
                        range: instr.name.text_range(),
                        message: "use `array.get` instead".into(),
                    }]),
                    ..Default::default()
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

fn find_struct_field<'db>(
    symbol_table: &'db SymbolTable<'db>,
    def_types: &'db DefTypes<'db>,
    instr: &FastPlainInstr,
) -> Option<(&'db StorageType<'db>, &'db Symbol<'db>)> {
    let mut immediates = instr
        .amber
        .children()
        .filter(|child| child.kind() == SyntaxKind::IMMEDIATE);
    let struct_def_key = symbol_table.resolved.get(&immediates.next()?.to_ptr().into())?;
    let field_ref_symbol = symbol_table
        .symbols
        .get(&SymbolKey::from(immediates.next()?.to_ptr()))?;
    if let Some(CompositeType::Struct(Fields(fields))) = def_types.get(struct_def_key).map(|def_type| &def_type.comp) {
        fields
            .iter()
            .find(|(_, idx)| field_ref_symbol.idx.is_defined_by(idx))
            .map(|(ty, _)| (&ty.storage, field_ref_symbol))
    } else {
        None
    }
}

fn find_array<'db>(
    symbol_table: &'db SymbolTable<'db>,
    def_types: &'db DefTypes<'db>,
    instr: &FastPlainInstr,
) -> Option<(&'db StorageType<'db>, &'db Symbol<'db>)> {
    let ref_key = instr
        .amber
        .children()
        .find(|child| child.kind() == SyntaxKind::IMMEDIATE)?
        .to_ptr()
        .into();
    let ref_symbol = symbol_table.symbols.get(&ref_key)?;
    if let Some(CompositeType::Array(Some(FieldType { storage, .. }))) = def_types
        .get(symbol_table.resolved.get(&ref_key)?)
        .map(|def_type| &def_type.comp)
    {
        Some((storage, ref_symbol))
    } else {
        None
    }
}
