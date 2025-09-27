use crate::{
    LanguageService,
    binder::{Symbol, SymbolKey, SymbolTable},
    document::Document,
    helpers,
    types_analyzer::{self, CompositeType, DefTypes, FieldType, Fields, StorageType},
    uri::InternUri,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, Union2};
use rowan::ast::support;
use wat_syntax::{SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "packing";

pub fn check(
    service: &LanguageService,
    diagnostics: &mut Vec<Diagnostic>,
    uri: InternUri,
    document: Document,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
    node: &SyntaxNode,
) -> Option<()> {
    let instr_name = support::token(node, SyntaxKind::INSTR_NAME)?;
    match instr_name.text() {
        "struct.get" => {
            let def_types = types_analyzer::get_def_types(service, document);
            if let Some((_, symbol)) =
                find_struct_field(symbol_table, def_types, node).filter(|(ty, _)| ty.is_packed())
            {
                diagnostics.push(Diagnostic {
                    range: helpers::rowan_range_to_lsp_range(line_index, symbol.key.text_range()),
                    severity: Some(DiagnosticSeverity::Error),
                    source: Some("wat".into()),
                    code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                    message: format!("field `{}` is packed", symbol.idx.render(service)),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            uri: uri.raw(service),
                            range: helpers::rowan_range_to_lsp_range(
                                line_index,
                                instr_name.text_range(),
                            ),
                        },
                        message: "use `struct.get_s` or `struct.get_u` instead".into(),
                    }]),
                    ..Default::default()
                });
            }
        }
        "struct.get_s" | "struct.get_u" => {
            let def_types = types_analyzer::get_def_types(service, document);
            if let Some((_, symbol)) =
                find_struct_field(symbol_table, def_types, node).filter(|(ty, _)| !ty.is_packed())
            {
                diagnostics.push(Diagnostic {
                    range: helpers::rowan_range_to_lsp_range(line_index, symbol.key.text_range()),
                    severity: Some(DiagnosticSeverity::Error),
                    source: Some("wat".into()),
                    code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                    message: format!("field `{}` is unpacked", symbol.idx.render(service)),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            uri: uri.raw(service),
                            range: helpers::rowan_range_to_lsp_range(
                                line_index,
                                instr_name.text_range(),
                            ),
                        },
                        message: "use `struct.get` instead".into(),
                    }]),
                    ..Default::default()
                });
            }
        }
        "array.get" => {
            let def_types = types_analyzer::get_def_types(service, document);
            if let Some((_, symbol)) =
                find_array(symbol_table, def_types, node).filter(|(ty, _)| ty.is_packed())
            {
                diagnostics.push(Diagnostic {
                    range: helpers::rowan_range_to_lsp_range(line_index, symbol.key.text_range()),
                    severity: Some(DiagnosticSeverity::Error),
                    source: Some("wat".into()),
                    code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                    message: format!("array `{}` is packed", symbol.idx.render(service)),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            uri: uri.raw(service),
                            range: helpers::rowan_range_to_lsp_range(
                                line_index,
                                instr_name.text_range(),
                            ),
                        },
                        message: "use `array.get_s` or `array.get_u` instead".into(),
                    }]),
                    ..Default::default()
                });
            }
        }
        "array.get_s" | "array.get_u" => {
            let def_types = types_analyzer::get_def_types(service, document);
            if let Some((_, symbol)) =
                find_array(symbol_table, def_types, node).filter(|(ty, _)| !ty.is_packed())
            {
                diagnostics.push(Diagnostic {
                    range: helpers::rowan_range_to_lsp_range(line_index, symbol.key.text_range()),
                    severity: Some(DiagnosticSeverity::Error),
                    source: Some("wat".into()),
                    code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                    message: format!("array `{}` is unpacked", symbol.idx.render(service)),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            uri: uri.raw(service),
                            range: helpers::rowan_range_to_lsp_range(
                                line_index,
                                instr_name.text_range(),
                            ),
                        },
                        message: "use `array.get` instead".into(),
                    }]),
                    ..Default::default()
                });
            }
        }
        _ => {}
    }
    Some(())
}

fn find_struct_field<'db>(
    symbol_table: &'db SymbolTable<'db>,
    def_types: &'db DefTypes<'db>,
    node: &SyntaxNode,
) -> Option<(&'db StorageType<'db>, &'db Symbol<'db>)> {
    let mut immediates = node
        .children()
        .filter(|child| child.kind() == SyntaxKind::IMMEDIATE);
    let struct_def_key = symbol_table
        .resolved
        .get(&SymbolKey::new(&immediates.next()?))?;
    let field_ref_symbol = symbol_table
        .symbols
        .get(&SymbolKey::new(&immediates.next()?))?;
    if let Some(CompositeType::Struct(Fields(fields))) =
        def_types.get(struct_def_key).map(|def_type| &def_type.comp)
    {
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
    node: &SyntaxNode,
) -> Option<(&'db StorageType<'db>, &'db Symbol<'db>)> {
    let ref_key = SymbolKey::new(
        &node
            .children()
            .find(|child| child.kind() == SyntaxKind::IMMEDIATE)?,
    );
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
