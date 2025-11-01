use crate::helpers;
use line_index::LineIndex;
use lspt::{Diagnostic, DiagnosticSeverity, Range, Union2};
use rowan::ast::AstNode;
use wat_syntax::{SyntaxNode, ast::Instr};

const DIAGNOSTIC_CODE: &str = "const-expr";

pub fn check(line_index: &LineIndex, node: &SyntaxNode) -> Option<Diagnostic> {
    let first = node.first_child_by_kind(&Instr::can_cast)?;
    let last = node
        .children()
        .filter(|child| Instr::can_cast(child.kind()))
        .last()?;
    if node.descendants().filter_map(Instr::cast).all(|instr| {
        if let Instr::Plain(plain) = instr {
            plain.instr_name().is_some_and(|instr_name| {
                matches!(
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
            })
        } else {
            false
        }
    }) {
        None
    } else {
        Some(Diagnostic {
            range: Range {
                start: helpers::rowan_pos_to_lsp_pos(line_index, first.text_range().start()),
                end: helpers::rowan_pos_to_lsp_pos(line_index, last.text_range().end()),
            },
            severity: Some(DiagnosticSeverity::Error),
            source: Some("wat".into()),
            code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
            message: "expression must be constant".into(),
            ..Default::default()
        })
    }
}
