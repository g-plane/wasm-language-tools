use crate::{LanguageService, helpers::LineIndexExt, uri::InternUri};
use lspt::{DocumentFormattingParams, DocumentRangeFormattingParams, FormattingOptions, TextEdit};
use wat_formatter::config::{FormatOptions, LanguageOptions, LayoutOptions};
use wat_syntax::ast::{AstNode, Root};

impl LanguageService {
    /// Handler for `textDocument/formatting` request.
    pub fn formatting(&self, params: DocumentFormattingParams) -> Option<Vec<TextEdit>> {
        let uri = InternUri::new(self, params.text_document.uri);
        let document = self.get_document(uri)?;
        let configs = self.configs.read();
        let config = configs.get(&uri)?.unwrap_or_global(self);
        let line_index = document.line_index(self);
        let root = Root::cast(document.root_tree(self))?;
        let formatted = wat_formatter::format(&root, &build_options(&params.options, config.format.clone()));
        let text_edit = TextEdit {
            range: line_index.convert(root.syntax().text_range()),
            new_text: formatted,
        };
        Some(vec![text_edit])
    }

    /// Handler for `textDocument/rangeFormatting` request.
    pub fn range_formatting(&self, params: DocumentRangeFormattingParams) -> Option<Vec<TextEdit>> {
        let uri = InternUri::new(self, params.text_document.uri);
        let document = self.get_document(uri)?;
        let configs = self.configs.read();
        let config = configs.get(&uri)?.unwrap_or_global(self);
        let line_index = document.line_index(self);
        let root = Root::cast(document.root_tree(self))?;
        let (formatted, range) = wat_formatter::format_range(
            &root,
            &build_options(&params.options, config.format.clone()),
            line_index.convert(params.range)?,
            line_index,
        )?;
        let text_edit = TextEdit {
            range: line_index.convert(range),
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
