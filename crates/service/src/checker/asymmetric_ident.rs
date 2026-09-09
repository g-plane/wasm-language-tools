use super::{Diagnostic, RelatedInformation};
use wat_syntax::{AmberNode, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "asymmetric-ident";

pub fn check(node: AmberNode) -> Option<Diagnostic> {
    let close = node
        .children_by_kind(SyntaxKind::END_DELIM)
        .next()?
        .tokens_by_kind(SyntaxKind::IDENT)
        .next()?;
    if let Some(open) = node.tokens_by_kind(SyntaxKind::IDENT).next() {
        if open.text() == close.text() {
            None
        } else {
            let block_kind = display_block_kind(node);
            Some(Diagnostic {
                range: close.text_range(),
                code: DIAGNOSTIC_CODE.into(),
                message: format!(
                    "mismatched ident whose enclosing `{block_kind}` is labeled as `{}`",
                    open.text(),
                ),
                related_information: Some(vec![RelatedInformation {
                    range: open.text_range(),
                    message: format!("{block_kind} `{}` defined here", open.text()),
                }]),
                ..Default::default()
            })
        }
    } else {
        let block_kind = display_block_kind(node);
        Some(Diagnostic {
            range: close.text_range(),
            code: DIAGNOSTIC_CODE.into(),
            message: format!("unexpected ident whose enclosing `{block_kind}` is not labeled"),
            related_information: Some(vec![RelatedInformation {
                range: node
                    .tokens_by_kind(SyntaxKind::KEYWORD)
                    .next()
                    .map(|token| token.text_range())
                    .unwrap_or_else(|| node.text_range()),
                message: format!("{block_kind} defined here"),
            }]),
            ..Default::default()
        })
    }
}

fn display_block_kind(node: AmberNode) -> &'static str {
    match node.kind() {
        SyntaxKind::BLOCK_BLOCK => "block",
        SyntaxKind::BLOCK_LOOP => "loop",
        SyntaxKind::BLOCK_IF => "if",
        SyntaxKind::BLOCK_TRY_TABLE => "try_table",
        _ => unreachable!(),
    }
}
