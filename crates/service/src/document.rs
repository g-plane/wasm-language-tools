use crate::{
    LanguageService,
    config::ConfigState,
    helpers::LineIndexExt,
    uri::{InternUri, IntoInternUri},
};
use line_index::LineIndex;
use lspt::{
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, TextDocumentContentChangeEvent,
};
use salsa::Setter;
use std::{cmp::Ordering, ops::Range};
use wat_syntax::{GreenNode, SyntaxNode, TextSize};

#[salsa::input(debug)]
pub(crate) struct Document {
    pub uri: InternUri,
    pub text: String,
    #[returns(ref)]
    pub line_index: LineIndex,
    pub root: GreenNode,
    #[returns(ref)]
    pub syntax_errors: Vec<wat_parser::SyntaxError>,
}

impl Document {
    pub(crate) fn root_tree(self, db: &dyn salsa::Database) -> SyntaxNode {
        SyntaxNode::new_root(self.root(db))
    }
}

impl LanguageService {
    #[inline]
    /// Commit a document to the service.
    pub fn commit(&mut self, uri: impl AsRef<str>, text: String) {
        let uri = InternUri::new(self, uri.as_ref());
        let line_index = LineIndex::new(&text);
        let (green, errors) = wat_parser::parse(&text);
        if let Some(document) = self.get_document(uri) {
            document.set_text(self).to(text);
            document.set_line_index(self).to(line_index);
            document.set_root(self).to(green);
            document.set_syntax_errors(self).to(errors);
        } else {
            self.documents
                .write()
                .insert(uri, Document::new(self, uri, text, line_index, green, errors));
            if !self.support_pull_config {
                self.configs.write().insert(uri, ConfigState::Inherit);
            }
        };
    }

    /// Handler for `textDocument/didOpen` notification.
    pub fn did_open(&mut self, params: DidOpenTextDocumentParams) {
        let uri = InternUri::new(self, params.text_document.uri);
        let line_index = LineIndex::new(&params.text_document.text);
        let (green, errors) = wat_parser::parse(&params.text_document.text);
        self.documents.write().insert(
            uri,
            Document::new(self, uri, params.text_document.text, line_index, green, errors),
        );
        if !self.support_pull_config {
            self.configs.write().insert(uri, ConfigState::Inherit);
        }
    }

    /// Handler for `textDocument/didChange` notification.
    pub fn did_change(&mut self, params: DidChangeTextDocumentParams) {
        let Some(document) = self.get_document(params.text_document.uri) else {
            return;
        };
        'single: {
            // only do incremental parsing for single change
            if params.content_changes.len() == 1 {
                match &*params.content_changes {
                    [TextDocumentContentChangeEvent::A(partial)] => {
                        if partial.text.starts_with(';') {
                            // user may be typing a comment
                            break 'single;
                        }
                        let Some(range) = document.line_index(self).convert(partial.range) else {
                            break 'single;
                        };
                        // search the module field where code is changed
                        let Some(node) = document
                            .root_tree(self)
                            .children()
                            .find(|module| module.text_range().contains_range(range))
                            .and_then(|module| module.child_at_range(range))
                            .filter(|node| {
                                let node_range = node.text_range();
                                node_range.start() <= range.start() && node_range.end() > range.end()
                            })
                        else {
                            break 'single;
                        };
                        let mut text = document.text(self);
                        let old_start = usize::from(range.start());
                        let old_end = usize::from(range.end());
                        // apply change to source text
                        text.replace_range(old_start..old_end, &partial.text);
                        // parse that module field by specifying offset with changed code
                        let node_range = node.text_range();
                        let Some((green, mut partial_errors)) =
                            wat_parser::parse_partial(&text, node_range.start().into())
                        else {
                            break 'single;
                        };

                        let mut all_errors = document.syntax_errors(self).clone();
                        all_errors.retain_mut(|error| {
                            match (
                                node_range.start().cmp(&error.range.start()),
                                node_range.end().cmp(&error.range.end()),
                            ) {
                                // parser has returned new syntax errors about that module field,
                                // so we need to remove old errors that belongs to that module field
                                (Ordering::Less | Ordering::Equal, Ordering::Greater | Ordering::Equal) => false,
                                // for syntax errors after that module field,
                                // we need to adjust their locations
                                (Ordering::Less | Ordering::Equal, _) => {
                                    error.range = error.range + TextSize::of(&partial.text) - range.len();
                                    true
                                }
                                _ => true,
                            }
                        });
                        all_errors.append(&mut partial_errors);

                        let line_index = LineIndex::new(&text);
                        document.set_text(self).to(text);
                        document.set_line_index(self).to(line_index);
                        document.set_root(self).to(node.replace_with(green));
                        document.set_syntax_errors(self).to(all_errors);
                    }
                    [TextDocumentContentChangeEvent::B(whole)] => {
                        let (green, errors) = wat_parser::parse(&whole.text);
                        document.set_text(self).to(whole.text.clone());
                        document.set_line_index(self).to(LineIndex::new(&whole.text));
                        document.set_root(self).to(green);
                        document.set_syntax_errors(self).to(errors);
                    }
                    _ => break 'single,
                }
                return;
            }
        }

        let mut line_index = document.line_index(self).clone();
        let mut text = document.text(self);
        params.content_changes.into_iter().for_each(|change| match change {
            TextDocumentContentChangeEvent::A(partial) => {
                if let Some(range) = line_index.convert(partial.range) {
                    text.replace_range::<Range<usize>>(range.start().into()..range.end().into(), &partial.text);
                    line_index = LineIndex::new(&text);
                }
            }
            TextDocumentContentChangeEvent::B(whole) => {
                line_index = LineIndex::new(&whole.text);
                text = whole.text;
            }
        });

        let (green, errors) = wat_parser::parse(&text);
        document.set_text(self).to(text);
        document.set_line_index(self).to(line_index);
        document.set_root(self).to(green);
        document.set_syntax_errors(self).to(errors);
    }

    /// Handler for `textDocument/didClose` notification.
    pub fn did_close(&mut self, params: DidCloseTextDocumentParams) {
        let uri = InternUri::new(self, params.text_document.uri);
        self.documents.write().remove(&uri);
        self.configs.write().remove(&uri);
    }

    #[inline]
    /// Get URIs of all opened documents.
    pub fn get_opened_uris(&self) -> Vec<String> {
        self.documents.read().keys().map(|uri| uri.raw(self)).collect()
    }

    pub(crate) fn get_document(&self, uri: impl IntoInternUri) -> Option<Document> {
        self.documents.read().get(&uri.into_intern_uri(self)).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lspt::{
        Definition, DefinitionParams, DocumentDiagnosticParams, DocumentHighlightParams, Hover, HoverParams, Location,
        MarkupContent, MarkupKind, Position, Range, TextDocumentContentChangePartial,
        TextDocumentContentChangeWholeDocument, TextDocumentIdentifier, TextDocumentItem, Union2, Union3,
        VersionedTextDocumentIdentifier,
    };

    #[test]
    fn get_opened_uris() {
        let mut service = LanguageService::default();
        assert!(service.get_opened_uris().is_empty());

        let a = "untitled://a.wat".to_string();
        service.commit(&a, "".into());
        assert_eq!(service.get_opened_uris().first(), Some(&a));

        let b = "untitled://b.wat".to_string();
        service.did_open(DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: b,
                language_id: lspt::LanguageKind::Custom_("wat".into()),
                version: 1,
                text: "".into(),
            },
        });
        assert_eq!(service.get_opened_uris().len(), 2);
    }

    #[test]
    fn single_cursor_in_module_field() {
        let uri = "untitled:test".to_string();
        let mut service = LanguageService::default();
        service.commit(&uri, "".into());
        service.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 1,
            },
            content_changes: vec![TextDocumentContentChangeEvent::B(
                TextDocumentContentChangeWholeDocument {
                    text: "(module
  (type ) (start)
)"
                    .into(),
                },
            )],
        });
        service.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 2,
            },
            content_changes: vec![TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                range: Range {
                    start: Position { line: 1, character: 8 },
                    end: Position { line: 1, character: 8 },
                },
                text: "(func (param $x) (param i32))".into(),
            })],
        });
        let mut diagnostics = service
            .pull_diagnostics(DocumentDiagnosticParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                identifier: None,
                previous_result_id: None,
                work_done_token: None,
                partial_result_token: None,
            })
            .items;
        diagnostics.sort_unstable_by_key(|diagnostic| diagnostic.range.start);
        let mut diagnostics = diagnostics.into_iter().filter(|diagnostic| {
            if let Some(Union2::B(code)) = &diagnostic.code {
                code == "syntax" || code.starts_with("syntax/")
            } else {
                false
            }
        });
        assert_eq!(
            diagnostics.next().unwrap().range,
            Range {
                start: Position { line: 1, character: 23 },
                end: Position { line: 1, character: 24 },
            }
        );
        assert_eq!(
            diagnostics.next().unwrap().range,
            Range {
                start: Position { line: 1, character: 45 },
                end: Position { line: 1, character: 46 },
            }
        );
    }

    #[test]
    fn single_cursor_to_invalid_module_field() {
        let uri = "untitled:test".to_string();
        let mut service = LanguageService::default();
        service.commit(&uri, "".into());
        service.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 1,
            },
            content_changes: vec![TextDocumentContentChangeEvent::B(
                TextDocumentContentChangeWholeDocument {
                    text: "(module
  (start 0) (type (func))
)"
                    .into(),
                },
            )],
        });
        service.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 2,
            },
            content_changes: vec![TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                range: Range {
                    start: Position { line: 1, character: 7 },
                    end: Position { line: 1, character: 8 },
                },
                text: "".into(),
            })],
        });
        let mut diagnostics = service
            .pull_diagnostics(DocumentDiagnosticParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                identifier: None,
                previous_result_id: None,
                work_done_token: None,
                partial_result_token: None,
            })
            .items;
        diagnostics.sort_unstable_by_key(|diagnostic| diagnostic.range.start);
        let mut diagnostics = diagnostics.into_iter().filter(|diagnostic| {
            if let Some(Union2::B(code)) = &diagnostic.code {
                code == "syntax"
            } else {
                false
            }
        });
        assert_eq!(
            diagnostics.next().unwrap().range,
            Range {
                start: Position { line: 1, character: 3 },
                end: Position { line: 1, character: 7 },
            }
        );
        assert_eq!(
            diagnostics.next().unwrap().range,
            Range {
                start: Position { line: 1, character: 8 },
                end: Position { line: 1, character: 9 },
            }
        );
        assert_eq!(
            diagnostics.next().unwrap().range,
            Range {
                start: Position { line: 1, character: 9 },
                end: Position { line: 1, character: 10 },
            }
        );
    }

    #[test]
    fn single_cursor_out_of_module_field() {
        let uri = "untitled:test".to_string();
        let mut service = LanguageService::default();
        service.commit(&uri, "".into());
        service.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 1,
            },
            content_changes: vec![TextDocumentContentChangeEvent::B(
                TextDocumentContentChangeWholeDocument {
                    text: "(module
  (func (result
    f32
    i32))
)"
                    .into(),
                },
            )],
        });
        service.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 2,
            },
            content_changes: vec![TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                range: Range {
                    start: Position { line: 3, character: 9 },
                    end: Position { line: 3, character: 9 },
                },
                text: "(type (func))".into(),
            })],
        });
        assert!(
            service
                .pull_diagnostics(DocumentDiagnosticParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    identifier: None,
                    previous_result_id: None,
                    work_done_token: None,
                    partial_result_token: None,
                })
                .items
                .iter()
                .all(|diagnostic| {
                    if let Some(Union2::B(code)) = &diagnostic.code {
                        code != "syntax" && !code.starts_with("syntax/")
                    } else {
                        true
                    }
                })
        );
    }

    #[test]
    fn multi_cursor_asc_insert_and_replace() {
        let uri = "untitled:test".to_string();
        let mut service = LanguageService::default();
        service.commit(&uri, "".into());
        service.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 1,
            },
            content_changes: vec![TextDocumentContentChangeEvent::B(
                TextDocumentContentChangeWholeDocument {
                    text: "(module
  (func (param i32 i32))
  (start 1)
    (func (param i64 i64))
)"
                    .into(),
                },
            )],
        });
        assert!(matches!(
            service
                .goto_definition(DefinitionParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position { line: 2, character: 10 },
                    work_done_token: None,
                    partial_result_token: None
                })
                .unwrap(),
            Definition::A(Location {
                range: Range {
                    start: Position { line: 3, .. },
                    ..
                },
                ..
            }),
        ));

        service.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 2,
            },
            content_changes: vec![
                TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                    range: Range {
                        start: Position { line: 1, character: 18 },
                        end: Position { line: 1, character: 18 },
                    },
                    text: "\n   ".into(),
                }),
                TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                    range: Range {
                        start: Position { line: 4, character: 20 },
                        end: Position { line: 4, character: 20 },
                    },
                    text: "\n     ".into(),
                }),
            ],
        });
        assert!(matches!(
            service
                .goto_definition(DefinitionParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position { line: 3, character: 10 },
                    work_done_token: None,
                    partial_result_token: None
                })
                .unwrap(),
            Definition::A(Location {
                range: Range {
                    start: Position { line: 4, .. },
                    ..
                },
                ..
            }),
        ));

        service.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 3,
            },
            content_changes: vec![
                TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                    range: Range {
                        start: Position { line: 2, character: 3 },
                        end: Position { line: 2, character: 3 },
                    },
                    text: "f".into(),
                }),
                TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                    range: Range {
                        start: Position { line: 5, character: 5 },
                        end: Position { line: 5, character: 5 },
                    },
                    text: "f".into(),
                }),
            ],
        });
        service.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 4,
            },
            content_changes: vec![
                TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                    range: Range {
                        start: Position { line: 2, character: 3 },
                        end: Position { line: 2, character: 4 },
                    },
                    text: "f32".into(),
                }),
                TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                    range: Range {
                        start: Position { line: 5, character: 5 },
                        end: Position { line: 5, character: 6 },
                    },
                    text: "f32".into(),
                }),
            ],
        });
        assert!(matches!(
            service
                .hover(HoverParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position {
                        line: 1,
                        character: 4,
                    },
                    work_done_token: None,
                })
                .unwrap(),
            Hover {
                contents: Union3::A(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value,
                }),
                ..
            } if value.contains("(func (param i32) (param f32) (param i32))"),
        ));
        assert!(matches!(
            service
                .hover(HoverParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position {
                        line: 4,
                        character: 6,
                    },
                    work_done_token: None,
                })
                .unwrap(),
            Hover {
                contents: Union3::A(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value,
                }),
                ..
            } if value.contains("(func (param i64) (param f32) (param i64))"),
        ));
    }

    #[test]
    fn multi_cursor_asc_delete() {
        let uri = "untitled:test".to_string();
        let mut service = LanguageService::default();
        service.commit(&uri, "".into());
        service.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 1,
            },
            content_changes: vec![TextDocumentContentChangeEvent::B(
                TextDocumentContentChangeWholeDocument {
                    text: "(module
  (func (result
    f32
    i32))
  (start 1)
    (func (result
      f64
      i64))
)"
                    .into(),
                },
            )],
        });
        assert!(matches!(
            service
                .goto_definition(DefinitionParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position { line: 4, character: 10 },
                    work_done_token: None,
                    partial_result_token: None
                })
                .unwrap(),
            Definition::A(Location {
                range: Range {
                    start: Position { line: 5, .. },
                    ..
                },
                ..
            }),
        ));

        service.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 2,
            },
            content_changes: vec![
                TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                    range: Range {
                        start: Position { line: 2, character: 0 },
                        end: Position { line: 3, character: 0 },
                    },
                    text: "".into(),
                }),
                TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                    range: Range {
                        start: Position { line: 5, character: 0 },
                        end: Position { line: 6, character: 0 },
                    },
                    text: "".into(),
                }),
            ],
        });
        assert!(matches!(
            service
                .goto_definition(DefinitionParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position { line: 3, character: 10 },
                    work_done_token: None,
                    partial_result_token: None
                })
                .unwrap(),
            Definition::A(Location {
                range: Range {
                    start: Position { line: 4, .. },
                    ..
                },
                ..
            }),
        ));
        assert!(matches!(
            service
                .hover(HoverParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position {
                        line: 1,
                        character: 4,
                    },
                    work_done_token: None,
                })
                .unwrap(),
            Hover {
                contents: Union3::A(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value,
                }),
                ..
            } if value.contains("(func (result i32))"),
        ));
        assert!(matches!(
            service
                .hover(HoverParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position {
                        line: 4,
                        character: 6,
                    },
                    work_done_token: None,
                })
                .unwrap(),
            Hover {
                contents: Union3::A(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value,
                }),
                ..
            } if value.contains("(func (result i64))"),
        ));
    }

    #[test]
    fn multi_cursor_desc_insert_and_replace() {
        let uri = "untitled:test".to_string();
        let mut service = LanguageService::default();
        service.commit(&uri, "".into());
        service.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 1,
            },
            content_changes: vec![TextDocumentContentChangeEvent::B(
                TextDocumentContentChangeWholeDocument {
                    text: "(module
  (func (param i32 i32))
  (start 1)
    (func (param i64 i64))
)"
                    .into(),
                },
            )],
        });
        assert!(matches!(
            service
                .goto_definition(DefinitionParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position { line: 2, character: 10 },
                    work_done_token: None,
                    partial_result_token: None
                })
                .unwrap(),
            Definition::A(Location {
                range: Range {
                    start: Position { line: 3, .. },
                    ..
                },
                ..
            }),
        ));

        service.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 2,
            },
            content_changes: vec![
                TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                    range: Range {
                        start: Position { line: 3, character: 20 },
                        end: Position { line: 3, character: 20 },
                    },
                    text: "\n      ".into(),
                }),
                TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                    range: Range {
                        start: Position { line: 1, character: 18 },
                        end: Position { line: 1, character: 18 },
                    },
                    text: "\n    ".into(),
                }),
            ],
        });
        assert!(matches!(
            service
                .goto_definition(DefinitionParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position { line: 3, character: 10 },
                    work_done_token: None,
                    partial_result_token: None
                })
                .unwrap(),
            Definition::A(Location {
                range: Range {
                    start: Position { line: 4, .. },
                    ..
                },
                ..
            }),
        ));

        service.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 3,
            },
            content_changes: vec![
                TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                    range: Range {
                        start: Position { line: 5, character: 6 },
                        end: Position { line: 5, character: 6 },
                    },
                    text: "f".into(),
                }),
                TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                    range: Range {
                        start: Position { line: 2, character: 4 },
                        end: Position { line: 2, character: 4 },
                    },
                    text: "f".into(),
                }),
            ],
        });
        service.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 4,
            },
            content_changes: vec![
                TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                    range: Range {
                        start: Position { line: 5, character: 6 },
                        end: Position { line: 5, character: 7 },
                    },
                    text: "f32".into(),
                }),
                TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                    range: Range {
                        start: Position { line: 2, character: 4 },
                        end: Position { line: 2, character: 5 },
                    },
                    text: "f32".into(),
                }),
            ],
        });
        assert!(matches!(
            service
                .hover(HoverParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position {
                        line: 1,
                        character: 4,
                    },
                    work_done_token: None,
                })
                .unwrap(),
            Hover {
                contents: Union3::A(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value,
                }),
                ..
            } if value.contains("(func (param i32) (param f32) (param i32))"),
        ));
        assert!(matches!(
            service
                .hover(HoverParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position {
                        line: 4,
                        character: 6,
                    },
                    work_done_token: None,
                })
                .unwrap(),
            Hover {
                contents: Union3::A(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value,
                }),
                ..
            } if value.contains("(func (param i64) (param f32) (param i64))"),
        ));
    }

    #[test]
    fn multi_cursor_desc_delete() {
        let uri = "untitled:test".to_string();
        let mut service = LanguageService::default();
        service.commit(&uri, "".into());
        service.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 1,
            },
            content_changes: vec![TextDocumentContentChangeEvent::B(
                TextDocumentContentChangeWholeDocument {
                    text: "(module
  (func (result
    f32
    i32))
  (start 1)
    (func (result
      f64
      i64))
)"
                    .into(),
                },
            )],
        });
        assert!(matches!(
            service
                .goto_definition(DefinitionParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position { line: 4, character: 10 },
                    work_done_token: None,
                    partial_result_token: None
                })
                .unwrap(),
            Definition::A(Location {
                range: Range {
                    start: Position { line: 5, .. },
                    ..
                },
                ..
            }),
        ));

        service.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 2,
            },
            content_changes: vec![
                TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                    range: Range {
                        start: Position { line: 6, character: 0 },
                        end: Position { line: 7, character: 0 },
                    },
                    text: "".into(),
                }),
                TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                    range: Range {
                        start: Position { line: 2, character: 0 },
                        end: Position { line: 3, character: 0 },
                    },
                    text: "".into(),
                }),
            ],
        });
        assert!(matches!(
            service
                .goto_definition(DefinitionParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position { line: 3, character: 10 },
                    work_done_token: None,
                    partial_result_token: None
                })
                .unwrap(),
            Definition::A(Location {
                range: Range {
                    start: Position { line: 4, .. },
                    ..
                },
                ..
            }),
        ));
        assert!(matches!(
            service
                .hover(HoverParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position {
                        line: 1,
                        character: 4,
                    },
                    work_done_token: None,
                })
                .unwrap(),
            Hover {
                contents: Union3::A(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value,
                }),
                ..
            } if value.contains("(func (result i32))"),
        ));
        assert!(matches!(
            service
                .hover(HoverParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: Position {
                        line: 4,
                        character: 6,
                    },
                    work_done_token: None,
                })
                .unwrap(),
            Hover {
                contents: Union3::A(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value,
                }),
                ..
            } if value.contains("(func (result i64))"),
        ));
    }

    #[test]
    fn insert_semicolon() {
        let uri = "untitled:test".to_string();
        let mut service = LanguageService::default();
        service.commit(
            &uri,
            "
(module
  (func (param i32) (local i32)))
"
            .into(),
        );
        service.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 1,
            },
            content_changes: vec![TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                range: Range {
                    start: Position { line: 2, character: 9 },
                    end: Position { line: 2, character: 9 },
                },
                text: ";".into(),
            })],
        });
        // should not panic
        service.document_highlight(DocumentHighlightParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position { line: 2, character: 10 },
            work_done_token: None,
            partial_result_token: None,
        });
    }

    #[test]
    fn insert_before_module() {
        let uri = "untitled:test".to_string();
        let mut service = LanguageService::default();
        service.commit(
            &uri,
            "
(@metadata.code.call_target )
(module
  (func (param i32) (local i32)))
"
            .into(),
        );
        // should not panic
        service.did_change(DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 1,
            },
            content_changes: vec![TextDocumentContentChangeEvent::A(TextDocumentContentChangePartial {
                range: Range {
                    start: Position { line: 1, character: 27 },
                    end: Position { line: 1, character: 27 },
                },
                text: "s".into(),
            })],
        });
    }
}
