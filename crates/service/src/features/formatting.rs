use super::FilesCtx;
use crate::{helpers, LanguageService};
use lsp_types::{DocumentFormattingParams, TextEdit};
use rowan::ast::AstNode;
use wat_formatter::config::{FormatOptions, LayoutOptions};
use wat_syntax::{ast::Root, SyntaxNode};

impl LanguageService {
    pub fn formatting(&self, params: DocumentFormattingParams) -> Option<Vec<TextEdit>> {
        let uri = self.uri(params.text_document.uri);
        let line_index = self.line_index(uri);
        let root = Root::cast(SyntaxNode::new_root(self.root(uri)))?;
        let formatted = wat_formatter::format(
            &root,
            &FormatOptions {
                layout: LayoutOptions {
                    indent_width: params.options.tab_size as usize,
                    use_tabs: !params.options.insert_spaces,
                    ..Default::default()
                },
                language: Default::default(),
            },
        );
        let text_edit = TextEdit {
            range: helpers::rowan_range_to_lsp_range(&line_index, root.syntax().text_range()),
            new_text: formatted,
        };
        Some(vec![text_edit])
    }
}
