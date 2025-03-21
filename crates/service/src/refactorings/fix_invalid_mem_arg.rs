use crate::{
    helpers,
    uri::{InternUri, UrisCtx},
    LanguageService,
};
use line_index::LineIndex;
use lspt::{CodeAction, CodeActionContext, CodeActionKind, TextEdit, Union2, WorkspaceEdit};
use rowan::{SyntaxElementChildren, TextRange};
use rustc_hash::FxBuildHasher;
use std::collections::HashMap;
use wat_syntax::{SyntaxElement, SyntaxKind, SyntaxNode, WatLanguage};

pub fn act(
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    node: &SyntaxNode,
    context: &CodeActionContext,
) -> Option<CodeAction> {
    let mut text_edits = vec![];

    let mut children = node.children_with_tokens();
    while let Some(element) = children.next() {
        if let SyntaxElement::Token(token) = element {
            if token.kind() != SyntaxKind::ERROR {
                continue;
            }
            let text = token.text();
            let rest = text
                .strip_prefix("offset")
                .or_else(|| text.strip_prefix("align"));
            match rest {
                Some("") => {
                    let range = children.next().and_then(|element| match element {
                        SyntaxElement::Token(token) if token.kind() == SyntaxKind::WHITESPACE => {
                            Some(token.text_range())
                        }
                        _ => None,
                    });
                    if let Some(range) = children
                        .next()
                        .filter(|element| {
                            if let SyntaxElement::Token(token) = element {
                                token.kind() == SyntaxKind::ERROR
                                    && token.text().strip_prefix('=').is_some_and(|rest| {
                                        rest.chars().all(|c| c.is_ascii_digit())
                                    })
                            } else {
                                false
                            }
                        })
                        .and(range)
                    {
                        text_edits.push(TextEdit {
                            range: helpers::rowan_range_to_lsp_range(line_index, range),
                            new_text: String::new(),
                        });
                        if let Some(range) = check_after_eq(&mut children) {
                            text_edits.push(TextEdit {
                                range: helpers::rowan_range_to_lsp_range(line_index, range),
                                new_text: String::new(),
                            });
                        }
                        break;
                    }
                }
                Some("=") => {
                    if let Some(range) = check_after_eq(&mut children) {
                        text_edits.push(TextEdit {
                            range: helpers::rowan_range_to_lsp_range(line_index, range),
                            new_text: String::new(),
                        });
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    if text_edits.is_empty() {
        None
    } else {
        let mut changes = HashMap::with_capacity_and_hasher(1, FxBuildHasher);
        changes.insert(service.lookup_uri(uri), text_edits);
        Some(CodeAction {
            title: "Fix invalid memory argument".into(),
            kind: Some(CodeActionKind::QuickFix),
            edit: Some(WorkspaceEdit {
                changes: Some(changes),
                ..Default::default()
            }),
            is_preferred: Some(true),
            diagnostics: Some(
                context
                    .diagnostics
                    .iter()
                    .filter(|diagnostic| {
                        if let Some(Union2::B(s)) = &diagnostic.code {
                            s.starts_with("syntax/")
                        } else {
                            false
                        }
                    })
                    .cloned()
                    .collect(),
            ),
            ..Default::default()
        })
    }
}

fn check_after_eq(children: &mut SyntaxElementChildren<WatLanguage>) -> Option<TextRange> {
    let range = children.next().and_then(|element| match element {
        SyntaxElement::Token(token) if token.kind() == SyntaxKind::WHITESPACE => {
            Some(token.text_range())
        }
        _ => None,
    });
    match children.next() {
        Some(SyntaxElement::Token(token)) if token.kind() == SyntaxKind::ERROR => range,
        _ => None,
    }
}
