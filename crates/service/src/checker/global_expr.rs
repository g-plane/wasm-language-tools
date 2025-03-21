use crate::helpers;
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Union2};
use rowan::ast::{support, AstNode};
use wat_syntax::{ast::Instr, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "global-expr";

pub fn check(diagnostics: &mut Vec<Diagnostic>, line_index: &LineIndex, node: &SyntaxNode) {
    diagnostics.extend(
        support::children::<Instr>(node)
            .filter(|instr| match instr {
                Instr::Plain(plain) => {
                    let Some(instr_name) = plain.instr_name() else {
                        return false;
                    };
                    let instr_name = instr_name.text();
                    !instr_name.ends_with(".const")
                        && !matches!(
                            instr_name,
                            "global.get"
                                | "ref.null"
                                | "ref.i31"
                                | "ref.func"
                                | "struct.new"
                                | "struct.new_default"
                                | "array.new"
                                | "array.new_default"
                                | "array.new_fixed"
                                | "any.convert_extern"
                                | "extern.convert_any"
                        )
                }
                Instr::Block(..) => true,
            })
            .map(|instr| Diagnostic {
                range: helpers::rowan_range_to_lsp_range(line_index, instr.syntax().text_range()),
                severity: Some(DiagnosticSeverity::Error),
                source: Some("wat".into()),
                code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                message: "expression must be constant in global".into(),
                ..Default::default()
            }),
    );
}
