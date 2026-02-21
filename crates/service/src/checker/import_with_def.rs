use super::{Diagnostic, DiagnosticCtx, RelatedInformation};
use wat_syntax::{AmberNode, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "import-with-def";

pub fn check(ctx: &mut DiagnosticCtx, node: AmberNode) -> Option<Diagnostic> {
    if !ctx.imports.contains(&node.to_ptr().into()) {
        return None;
    }
    let first = node
        .children_by_kind(|kind| {
            !matches!(
                kind,
                SyntaxKind::EXPORT
                    | SyntaxKind::IMPORT
                    | SyntaxKind::TYPE_USE
                    | SyntaxKind::GLOBAL_TYPE
                    | SyntaxKind::MEM_TYPE
                    | SyntaxKind::TABLE_TYPE
            )
        })
        .next()?;
    let last = node.children().next_back()?;
    Some(Diagnostic {
        range: first.text_range().cover(last.text_range()),
        code: DIAGNOSTIC_CODE.into(),
        message: "imported item can't contain definition".into(),
        related_information: node.children_by_kind(SyntaxKind::IMPORT).next().map(|import| {
            vec![RelatedInformation {
                range: import.text_range(),
                message: "import declared here".into(),
            }]
        }),
        ..Default::default()
    })
}
