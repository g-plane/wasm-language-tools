use crate::{helpers, syntax_tree::SyntaxTreeCtx, uri::InternUri, LanguageService};
use line_index::{LineIndex, TextSize};
use lspt::{Diagnostic, DiagnosticSeverity, Position, Range, Union2};
use wat_parser::Message;
use wat_syntax::{SyntaxElement, SyntaxKind, SyntaxNode};

const DIAGNOSTIC_CODE: &str = "syntax";

pub fn check(
    service: &LanguageService,
    diagnostics: &mut Vec<Diagnostic>,
    uri: InternUri,
    line_index: &LineIndex,
    root: &SyntaxNode,
) {
    diagnostics.extend(service.parse(uri).1.iter().map(|error| {
        let start = line_index.line_col(TextSize::new(error.start as u32));
        let end = line_index.line_col(TextSize::new(error.end as u32));
        Diagnostic {
            range: Range {
                start: Position {
                    line: start.line,
                    character: start.col,
                },
                end: Position {
                    line: end.line,
                    character: end.col,
                },
            },
            severity: Some(DiagnosticSeverity::Error),
            source: Some("wat".into()),
            code: if let Message::Name(name) = error.message {
                Some(Union2::B(format!(
                    "{DIAGNOSTIC_CODE}/{}",
                    name.replace(' ', "-")
                )))
            } else {
                Some(Union2::B(DIAGNOSTIC_CODE.into()))
            },
            message: format!("syntax error: {}", error.message),
            ..Default::default()
        }
    }));
    diagnostics.extend(
        root.children_with_tokens()
            .filter_map(|element| match element {
                SyntaxElement::Token(token) if token.kind() == SyntaxKind::ERROR => Some(token),
                _ => None,
            })
            .map(|token| Diagnostic {
                range: helpers::rowan_range_to_lsp_range(line_index, token.text_range()),
                severity: Some(DiagnosticSeverity::Error),
                source: Some("wat".into()),
                code: Some(Union2::B(DIAGNOSTIC_CODE.into())),
                message: "syntax error: unexpected token".into(),
                ..Default::default()
            }),
    );
}
