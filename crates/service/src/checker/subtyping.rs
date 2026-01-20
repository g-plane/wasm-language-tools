use crate::{binder::SymbolTable, document::Document, helpers, types_analyzer};
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use rowan::ast::AstNode;
use wat_syntax::{SyntaxNode, ast::TypeDef};

const DIAGNOSTIC_CODE: &str = "subtyping";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
    line_index: &LineIndex,
    root: &SyntaxNode,
    symbol_table: &SymbolTable,
) {
    let def_types = types_analyzer::get_def_types(db, document);
    diagnostics.extend(
        def_types
            .iter()
            .filter_map(|(key, sub_type)| {
                let inherits = sub_type.inherits.as_ref()?;
                let super_type = def_types.get(&inherits.symbol)?;
                let module_key = symbol_table.symbols.get(key)?.region;
                let module_id = symbol_table.symbols.get(&module_key)?.idx.num?;

                if super_type
                    .idx
                    .num
                    .zip(sub_type.idx.num)
                    .is_some_and(|(sup, sub)| sup >= sub)
                {
                    Some((
                        key,
                        format!(
                            "typeidx of super type `{}` must be smaller than type `{}`",
                            super_type.idx.render(db),
                            sub_type.idx.render(db),
                        ),
                    ))
                } else if super_type.is_final {
                    Some((
                        key,
                        format!("type `{}` is final", super_type.idx.render(db)),
                    ))
                } else if !sub_type
                    .comp
                    .matches(&super_type.comp, db, document, module_id)
                {
                    Some((
                        key,
                        format!(
                            "type of `{}` doesn't match its super type `{}`",
                            sub_type.idx.render(db),
                            super_type.idx.render(db),
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
