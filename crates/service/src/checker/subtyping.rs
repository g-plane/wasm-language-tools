use super::Diagnostic;
use crate::{binder::SymbolTable, document::Document, types_analyzer};
use wat_syntax::{
    SyntaxNode,
    ast::{AstNode, TypeDef},
};

const DIAGNOSTIC_CODE: &str = "subtyping";

pub fn check(
    diagnostics: &mut Vec<Diagnostic>,
    db: &dyn salsa::Database,
    document: Document,
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
                    Some((key, format!("type `{}` is final", super_type.idx.render(db))))
                } else if !sub_type.comp.matches(&super_type.comp, db, document, module_id) {
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
                let index = TypeDef::cast(key.to_node(root))?.sub_type()?.indexes().next()?;
                Some(Diagnostic {
                    range: index.syntax().text_range(),
                    code: DIAGNOSTIC_CODE.into(),
                    message,
                    ..Default::default()
                })
            }),
    );
}
