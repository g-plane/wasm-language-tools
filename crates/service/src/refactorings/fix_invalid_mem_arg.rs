use crate::{files::FilesCtx, helpers, InternUri, LanguageService};
use line_index::LineIndex;
use lsp_types::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};
use rowan::{ast::AstNode, SyntaxElementChildren, TextRange};
use std::collections::HashMap;
use wat_syntax::{ast::Operand, SyntaxElement, SyntaxKind, SyntaxNode, WatLanguage};

pub fn act(
    service: &LanguageService,
    uri: InternUri,
    line_index: &LineIndex,
    node: &SyntaxNode,
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

    #[expect(clippy::mutable_key_type)]
    if text_edits.is_empty() {
        None
    } else {
        let mut changes = HashMap::with_capacity(1);
        changes.insert(service.lookup_uri(uri), text_edits);
        Some(CodeAction {
            title: "Fix invalid memory argument".into(),
            kind: Some(CodeActionKind::QUICKFIX),
            edit: Some(WorkspaceEdit {
                changes: Some(changes),
                ..Default::default()
            }),
            is_preferred: Some(true),
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
    children
        .next()
        .and_then(SyntaxElement::into_node)
        .and_then(Operand::cast)
        .and_then(|operand| operand.int())
        .and(range)
}
