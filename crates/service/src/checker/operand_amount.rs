use crate::{data_set, helpers};
use line_index::LineIndex;
use lsp_types::{Diagnostic, DiagnosticSeverity};
use rowan::ast::support::token;
use wat_syntax::{SyntaxKind, SyntaxNode};

pub fn check(diags: &mut Vec<Diagnostic>, line_index: &LineIndex, node: &SyntaxNode) {
    let Some(instr_name) = token(node, SyntaxKind::INSTR_NAME) else {
        return;
    };
    let Some(meta) = data_set::INSTR_METAS.get(instr_name.text()) else {
        return;
    };

    let expected = if token(node, SyntaxKind::L_PAREN).is_some() {
        meta.operands_count + meta.params.len()
    } else {
        meta.operands_count
    };
    let found = node
        .children()
        .filter(|child| child.kind() == SyntaxKind::OPERAND)
        .count();
    if expected != found {
        diags.push(Diagnostic {
            range: helpers::rowan_range_to_lsp_range(line_index, node.text_range()),
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("wat".into()),
            message: format!("expected {}, found {found}", pluralize(expected)),
            ..Default::default()
        });
    }
}

fn pluralize(count: usize) -> String {
    if count == 1 {
        "1 operand".into()
    } else {
        format!("{count} operands")
    }
}
