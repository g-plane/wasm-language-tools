use crate::{LanguageService, helpers};
use lspt::{DocumentFormattingParams, DocumentRangeFormattingParams, FormattingOptions, TextEdit};
use rowan::ast::AstNode;
use wat_formatter::config::{FormatOptions, LanguageOptions, LayoutOptions};
use wat_syntax::ast::Root;

impl LanguageService {
    /// Handler for `textDocument/formatting` request.
    pub fn formatting(&self, params: DocumentFormattingParams) -> Option<Vec<TextEdit>> {
        let document = self.get_document(params.text_document.uri)?;
        let line_index = document.line_index(self);
        let root = Root::cast(document.root_tree(self))?;
        let formatted = wat_formatter::format(
            &root,
            &build_options(&params.options, self.get_config(document).format.clone()),
        );
        let text_edit = TextEdit {
            range: helpers::rowan_range_to_lsp_range(line_index, root.syntax().text_range()),
            new_text: formatted,
        };
        Some(vec![text_edit])
    }

    /// Handler for `textDocument/rangeFormatting` request.
    pub fn range_formatting(&self, params: DocumentRangeFormattingParams) -> Option<Vec<TextEdit>> {
        let document = self.get_document(params.text_document.uri)?;
        let line_index = document.line_index(self);
        let root = Root::cast(document.root_tree(self))?;
        let (formatted, range) = wat_formatter::format_range(
            &root,
            &build_options(&params.options, self.get_config(document).format.clone()),
            helpers::lsp_range_to_rowan_range(line_index, params.range)?,
            line_index,
        )?;
        let text_edit = TextEdit {
            range: helpers::rowan_range_to_lsp_range(line_index, range),
            new_text: formatted,
        };
        Some(vec![text_edit])
    }
}

fn build_options(layout: &FormattingOptions, language: LanguageOptions) -> FormatOptions {
    FormatOptions {
        layout: LayoutOptions {
            indent_width: layout.tab_size as usize,
            use_tabs: !layout.insert_spaces,
            ..Default::default()
        },
        language,
    }
}
