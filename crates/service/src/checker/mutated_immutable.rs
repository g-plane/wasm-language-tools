use crate::{
    binder::{SymbolKind, SymbolTable},
    helpers,
    mutability::{MutabilitiesCtx, MutationActionKind},
    uri::{InternUri, UrisCtx},
    LanguageService,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, Union2};

const DIAGNOSTIC_CODE: &str = "mutated-immutable";

pub fn check(
    service: &LanguageService,
    diagnostics: &mut Vec<Diagnostic>,
    uri: InternUri,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
) {
    let mutabilities = service.mutabilities(uri);
    let mutation_actions = service.mutation_actions(uri);
    diagnostics.extend(
        mutation_actions
            .iter()
            .filter(|(_, action)| action.kind == MutationActionKind::Set)
            .filter_map(|(key, action)| {
                action
                    .target
                    .and_then(|target| mutabilities.get_key_value(&target))
                    .filter(|(_, mutability)| mutability.mut_keyword.is_none())
                    .map(|(def_key, _)| {
                        let kind = match symbol_table
                            .symbols
                            .iter()
                            .find(|symbol| symbol.key == *def_key)
                            .map(|symbol| symbol.kind)
                        {
                            Some(SymbolKind::GlobalDef) => "global",
                            Some(SymbolKind::Type) => "array",
                            Some(SymbolKind::FieldDef) => "field",
                            _ => unreachable!(),
                        };
                        Diagnostic {
                            range: helpers::rowan_range_to_lsp_range(line_index, key.text_range()),
                            severity: Some(DiagnosticSeverity::Error),
                            source: Some("wat".into()),
                            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                            message: format!("mutating an immutable {kind} is not allowed"),
                            related_information: Some(vec![DiagnosticRelatedInformation {
                                location: Location {
                                    uri: service.lookup_uri(uri),
                                    range: helpers::rowan_range_to_lsp_range(
                                        line_index,
                                        def_key.text_range(),
                                    ),
                                },
                                message: format!("immutable {kind}"),
                            }]),
                            ..Default::default()
                        }
                    })
            }),
    );
}
