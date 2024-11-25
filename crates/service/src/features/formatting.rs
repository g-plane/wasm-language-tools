use super::FilesCtx;
use crate::{helpers, LanguageService};
use lsp_types::{
    DocumentFormattingParams, DocumentRangeFormattingParams, FormattingOptions, TextEdit,
};
use rowan::ast::AstNode;
use wat_formatter::config::{FormatOptions, LayoutOptions};
use wat_syntax::{ast::Root, SyntaxNode};

impl LanguageService {
    /// Handler for `textDocument/formatting` request.
    pub fn formatting(&self, params: DocumentFormattingParams) -> Option<Vec<TextEdit>> {
        let uri = self.uri(params.text_document.uri);
        let line_index = self.line_index(uri);
        let root = Root::cast(SyntaxNode::new_root(self.root(uri)))?;
        let formatted = wat_formatter::format(&root, &build_options(&params.options));
        let text_edit = TextEdit {
            range: helpers::rowan_range_to_lsp_range(&line_index, root.syntax().text_range()),
            new_text: formatted,
        };
        Some(vec![text_edit])
    }

    /// Handler for `textDocument/rangeFormatting` request.
    pub fn range_formatting(&self, params: DocumentRangeFormattingParams) -> Option<Vec<TextEdit>> {
        let uri = self.uri(params.text_document.uri);
        let line_index = self.line_index(uri);
        let root = Root::cast(SyntaxNode::new_root(self.root(uri)))?;
        let (formatted, range) = wat_formatter::format_range(
            &root,
            &build_options(&params.options),
            helpers::lsp_range_to_rowan_range(&line_index, params.range)?,
            &line_index,
        )?;
        let text_edit = TextEdit {
            range: helpers::rowan_range_to_lsp_range(&line_index, range),
            new_text: formatted,
        };
        Some(vec![text_edit])
    }
}

fn build_options(options: &FormattingOptions) -> FormatOptions {
    FormatOptions {
        layout: LayoutOptions {
            indent_width: options.tab_size as usize,
            use_tabs: !options.insert_spaces,
            ..Default::default()
        },
        language: Default::default(),
    }
}
