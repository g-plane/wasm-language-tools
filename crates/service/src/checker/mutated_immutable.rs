use crate::{
    LanguageService,
    binder::{SymbolKind, SymbolTable},
    document::Document,
    helpers, mutability,
    uri::InternUri,
};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, Union2};

const DIAGNOSTIC_CODE: &str = "mutated-immutable";

pub fn check(
    service: &LanguageService,
    diagnostics: &mut Vec<Diagnostic>,
    uri: InternUri,
    document: Document,
    line_index: &LineIndex,
    symbol_table: &SymbolTable,
) {
    let mutabilities = mutability::get_mutabilities(service, document);
    let mutation_actions = mutability::get_mutation_actions(service, document);
    diagnostics.extend(
        mutation_actions
            .iter()
            .filter(|(_, action)| action.kind == mutability::MutationActionKind::Set)
            .filter_map(|(key, action)| {
                action
                    .target
                    .and_then(|target| mutabilities.get_key_value(&target))
                    .filter(|(_, mutability)| mutability.mut_keyword.is_none())
                    .zip(
                        symbol_table
                            .symbols
                            .iter()
                            .find(|symbol| symbol.key == *key),
                    )
                    .map(|((def_key, _), ref_symbol)| {
                        let kind = match ref_symbol.kind {
                            SymbolKind::GlobalRef => "global",
                            SymbolKind::TypeUse => "array",
                            SymbolKind::FieldRef => "field",
                            _ => unreachable!(),
                        };
                        Diagnostic {
                            range: helpers::rowan_range_to_lsp_range(line_index, key.text_range()),
                            severity: Some(DiagnosticSeverity::Error),
                            source: Some("wat".into()),
                            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                            message: format!(
                                "mutating the immutable {kind} `{}` is not allowed",
                                ref_symbol.idx.render(service)
                            ),
                            related_information: Some(vec![DiagnosticRelatedInformation {
                                location: Location {
                                    uri: uri.raw(service),
                                    range: helpers::rowan_range_to_lsp_range(
                                        line_index,
                                        def_key.text_range(),
                                    ),
                                },
                                message: format!("immutable {kind} defined here"),
                            }]),
                            ..Default::default()
                        }
                    })
            }),
    );
}
