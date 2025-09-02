use crate::{LanguageService, binder::SymbolTable, document::Document, helpers, types_analyzer};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use rowan::ast::AstNode;
use wat_syntax::{SyntaxNode, ast::TypeDef};

const DIAGNOSTIC_CODE: &str = "subtyping";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    service: &LanguageService,
    document: Document,
    line_index: &LineIndex,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
) {
    let def_types = types_analyzer::get_def_types(service, document);
    diagnostics.extend(
        def_types
            .iter()
            .filter_map(|sub_type| {
                let inherits = sub_type.inherits.as_ref()?;
                let super_type = def_types
                    .iter()
                    .find(|def_type| def_type.key == inherits.symbol)?;
                let module_key = symbol_table
                    .symbols
                    .iter()
                    .find(|symbol| symbol.key == sub_type.key)?
                    .region;
                let module_id = symbol_table
                    .symbols
                    .iter()
                    .find(|symbol| symbol.key == module_key)?
                    .idx
                    .num?;

                if super_type
                    .idx
                    .num
                    .zip(sub_type.idx.num)
                    .is_some_and(|(sup, sub)| sup >= sub)
                {
                    Some((
                        sub_type.key,
                        format!(
                            "typeidx of super type `{}` must be smaller than type `{}`",
                            super_type.idx.render(service),
                            sub_type.idx.render(service),
                        ),
                    ))
                } else if super_type.is_final {
                    Some((
                        sub_type.key,
                        format!("type `{}` is final", super_type.idx.render(service)),
                    ))
                } else if !sub_type
                    .comp
                    .matches(&super_type.comp, service, document, module_id)
                {
                    Some((
                        sub_type.key,
                        format!(
                            "type of `{}` doesn't match its super type `{}`",
                            sub_type.idx.render(service),
                            super_type.idx.render(service),
                        ),
                    ))
                } else {
                    None
                }
            })
            .filter_map(|(key, message)| {
                let index = TypeDef::cast(key.to_node(root))?
                    .sub_type()?
                    .indexes()
                    .next()?;
                Some(Diagnostic {
                    range: helpers::rowan_range_to_lsp_range(
                        line_index,
                        index.syntax().text_range(),
                    ),
                    severity: Some(DiagnosticSeverity::Error),
                    source: Some("wat".into()),
                    code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                    message,
                    ..Default::default()
                })
            }),
    );
}
