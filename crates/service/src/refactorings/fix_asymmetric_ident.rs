use crate::helpers::LineIndexExt;
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionContext, CodeActionKind, Diagnostic, NumberOrString, TextEdit, WorkspaceEdit};
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;
use wat_syntax::{SyntaxKind, SyntaxNode, TextRange};

pub fn act(uri: &str, line_index: &LineIndex, node: &SyntaxNode, context: &CodeActionContext) -> Option<CodeAction> {
    let close = node
        .children_by_kind(SyntaxKind::END_DELIM)
        .next()?
        .tokens_by_kind(SyntaxKind::IDENT)
        .next()?;
    if let Some(open) = node.tokens_by_kind(SyntaxKind::IDENT).next() {
        if open.text() == close.text() {
            None
        } else {
            let text_edits = vec![TextEdit {
                range: line_index.convert(close.text_range())?,
                new_text: open.text().into(),
            }];
            let mut changes = HashMap::with_capacity_and_hasher(1, FxBuildHasher);
            changes.insert(uri.to_owned(), text_edits);
            Some(CodeAction {
                title: format!("Fix mismatched identifier `{}` to `{}`", close.text(), open.text()),
                kind: Some(CodeActionKind::QuickFix),
                edit: Some(WorkspaceEdit {
                    changes: Some(changes),
                    ..Default::default()
                }),
                is_preferred: Some(true),
                diagnostics: pick_diagnostic(&context.diagnostics, line_index, close.text_range()),
                ..Default::default()
            })
        }
    } else {
        let range = if let Some(token) = close
            .prev_consecutive_tokens()
            .find(|token| token.kind() == SyntaxKind::WHITESPACE)
        {
            token.text_range().cover(close.text_range())
        } else {
            close.text_range()
        };
        let text_edits = vec![TextEdit {
            range: line_index.convert(range)?,
            new_text: String::new(),
        }];
        let mut changes = HashMap::with_capacity_and_hasher(1, FxBuildHasher);
        changes.insert(uri.to_owned(), text_edits);
        Some(CodeAction {
            title: format!("Remove identifier `{}`", close.text()),
            kind: Some(CodeActionKind::QuickFix),
            edit: Some(WorkspaceEdit {
                changes: Some(changes),
                ..Default::default()
            }),
            is_preferred: Some(true),
            diagnostics: pick_diagnostic(&context.diagnostics, line_index, close.text_range()),
            ..Default::default()
        })
    }
}

fn pick_diagnostic(diagnostics: &[Diagnostic], line_index: &LineIndex, range: TextRange) -> Option<Vec<Diagnostic>> {
    diagnostics
        .iter()
        .find(|diagnostic| match &diagnostic.code {
            Some(NumberOrString::String(code)) => {
                code == "asymmetric-ident" && line_index.convert(diagnostic.range) == Some(range)
            }
            _ => false,
        })
        .map(|diagnostic| vec![diagnostic.clone()])
}
