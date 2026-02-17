use super::Diagnostic;
use wat_syntax::{AmberNode, SyntaxKind};

const DIAGNOSTIC_CODE: &str = "multiple-starts";

pub fn check(diagnostics: &mut Vec<Diagnostic>, module: AmberNode) {
    diagnostics.extend(
        module
            .children_by_kind(SyntaxKind::MODULE_FIELD_START)
            .skip(1)
            .map(|start| Diagnostic {
                range: start.text_range(),
                code: DIAGNOSTIC_CODE.into(),
                message: "only one start section is allowed".into(),
                ..Default::default()
            }),
    );
}
