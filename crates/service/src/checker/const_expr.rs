use crate::helpers;
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use rowan::ast::{AstNode, support};
use wat_syntax::{SyntaxNode, ast::Instr};

const DIAGNOSTIC_CODE: &str = "const-expr";

pub fn check(diagnostics: &mut Vec<Diagnostic>, line_index: &LineIndex, node: &SyntaxNode) {
    diagnostics.extend(
        support::children::<Instr>(node)
            .filter(|instr| match instr {
                Instr::Plain(plain) => plain.instr_name().is_some_and(|instr_name| {
                    !matches!(
                        instr_name.text().split_once('.'),
                        Some((_, "const"))
                            | Some(("i32" | "i64", "add" | "sub" | "mul"))
                            | Some(("global", "get"))
                            | Some(("ref", "null" | "i31" | "func"))
                            | Some(("struct" | "array", "new" | "new_default"))
                            | Some(("array", "new_fixed"))
                            | Some(("any", "convert_extern"))
                            | Some(("extern", "convert_any"))
                    )
                }),
                Instr::Block(..) => true,
            })
            .map(|instr| Diagnostic {
                range: helpers::rowan_range_to_lsp_range(line_index, instr.syntax().text_range()),
                severity: Some(DiagnosticSeverity::Error),
                source: Some("wat".into()),
                code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                message: "expression must be constant".into(),
                ..Default::default()
            }),
    );
}
