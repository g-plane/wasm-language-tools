use crate::{LanguageService, helpers::LineIndexExt, uri::InternUri};
use line_index::LineIndex;
use lspt::{
    DocumentFormattingParams, DocumentRangeFormattingParams, DocumentRangesFormattingParams, FormattingOptions,
    TextEdit,
};
use similar::{Algorithm, DiffOp};
use wat_formatter::config::{FormatOptions, LanguageOptions, LayoutOptions};
use wat_syntax::{AmberNode, TextRange, TextSize};

impl LanguageService {
    /// Handler for `textDocument/formatting` request.
    pub fn formatting(&self, params: DocumentFormattingParams) -> Option<Vec<TextEdit>> {
        let uri = InternUri::new(self, params.text_document.uri);
        let document = self.get_document(uri)?;
        let configs = self.configs.read();
        let config = configs.get(&uri)?.unwrap_or_global(self);
        let line_index = document.line_index(self);
        let old = document.text(self);
        let new = wat_formatter::format(
            document.root(self),
            &build_options(&params.options, config.format.clone()),
        );
        similar::capture_diff_slices(Algorithm::Myers, old.as_bytes(), new.as_bytes())
            .into_iter()
            .filter_map(|diff_op| match diff_op {
                DiffOp::Equal { .. } => None,
                diff_op => Some(diff_op),
            })
            .map(|diff_op| match diff_op {
                DiffOp::Equal { .. } => unreachable!(),
                DiffOp::Delete { old_index, old_len, .. } => {
                    let start = TextSize::try_from(old_index).ok()?;
                    let end = TextSize::try_from(old_index + old_len).ok()?;
                    line_index.convert(TextRange::new(start, end)).map(|range| TextEdit {
                        range,
                        new_text: String::new(),
                    })
                }
                DiffOp::Insert {
                    old_index,
                    new_index,
                    new_len,
                } => {
                    let start = TextSize::try_from(old_index).ok()?;
                    let new_text = new.get(new_index..new_index + new_len)?.into();
                    line_index
                        .convert(TextRange::empty(start))
                        .map(|range| TextEdit { range, new_text })
                }
                DiffOp::Replace {
                    old_index,
                    old_len,
                    new_index,
                    new_len,
                } => {
                    let start = TextSize::try_from(old_index).ok()?;
                    let end = TextSize::try_from(old_index + old_len).ok()?;
                    let new_text = new.get(new_index..new_index + new_len)?.into();
                    line_index
                        .convert(TextRange::new(start, end))
                        .map(|range| TextEdit { range, new_text })
                }
            })
            .collect::<Option<_>>()
            .or_else(|| {
                line_index
                    .convert(TextRange::new(0.into(), TextSize::of(old)))
                    .map(|range| vec![TextEdit { range, new_text: new }])
            })
    }

    /// Handler for `textDocument/rangeFormatting` request.
    pub fn range_formatting(&self, params: DocumentRangeFormattingParams) -> Option<Vec<TextEdit>> {
        let uri = InternUri::new(self, params.text_document.uri);
        let document = self.get_document(uri)?;
        let configs = self.configs.read();
        let config = configs.get(&uri)?.unwrap_or_global(self);
        let line_index = document.line_index(self);
        format_with_range(
            AmberNode::new_root(document.root(self)),
            line_index.convert(params.range)?,
            &build_options(&params.options, config.format.clone()),
            line_index,
        )
    }

    /// Handler for `textDocument/rangesFormatting` request.
    pub fn ranges_formatting(&self, params: DocumentRangesFormattingParams) -> Option<Vec<TextEdit>> {
        let uri = InternUri::new(self, params.text_document.uri);
        let document = self.get_document(uri)?;
        let configs = self.configs.read();
        let config = configs.get(&uri)?.unwrap_or_global(self);
        let options = build_options(&params.options, config.format.clone());
        let line_index = document.line_index(self);
        let root = AmberNode::new_root(document.root(self));

        let mut ranges = params
            .ranges
            .into_iter()
            .filter_map(|range| line_index.convert(range))
            .collect::<Vec<_>>();
        ranges.sort_unstable_by(|a, b| a.ordering(*b));
        ranges.dedup_by(|a, b| {
            if b.contains_inclusive(a.start()) {
                *b = b.cover(*a);
                true
            } else {
                false
            }
        });
        Some(
            ranges
                .into_iter()
                .filter_map(|range| format_with_range(root, range, &options, line_index))
                .flatten()
                .collect(),
        )
    }
}

fn format_with_range(
    root: AmberNode,
    range: TextRange,
    options: &FormatOptions,
    line_index: &LineIndex,
) -> Option<Vec<TextEdit>> {
    let mut node = root;
    while let Some(it) = node.child_at_range(range) {
        node = it;
    }
    let node_start = node.text_range().start();
    let old = node.green().to_string();
    let new = wat_formatter::format_node(node, options, line_index.convert(node_start)?.character as usize);
    let text_edits = similar::capture_diff_slices(Algorithm::Myers, old.as_bytes(), new.as_bytes())
        .into_iter()
        .filter_map(|diff_op| match diff_op {
            DiffOp::Delete { old_index, old_len, .. }
                if range.start() <= node_start + TextSize::try_from(old_index).ok()?
                    && node_start + TextSize::try_from(old_index + old_len).ok()? <= range.end() =>
            {
                let start = node_start + TextSize::try_from(old_index).ok()?;
                let end = node_start + TextSize::try_from(old_index + old_len).ok()?;
                line_index.convert(TextRange::new(start, end)).map(|range| TextEdit {
                    range,
                    new_text: String::new(),
                })
            }
            DiffOp::Insert {
                old_index,
                new_index,
                new_len,
            } if range.start() <= node_start + TextSize::try_from(old_index).ok()?
                && node_start + TextSize::try_from(old_index).ok()? < range.end() =>
            {
                let start = node_start + TextSize::try_from(old_index).ok()?;
                let new_text = new.get(new_index..new_index + new_len)?.into();
                line_index
                    .convert(TextRange::empty(start))
                    .map(|range| TextEdit { range, new_text })
            }
            DiffOp::Replace {
                old_index,
                old_len,
                new_index,
                new_len,
            } if range.start() <= node_start + TextSize::try_from(old_index).ok()?
                && node_start + TextSize::try_from(old_index + old_len).ok()? <= range.end() =>
            {
                let start = node_start + TextSize::try_from(old_index).ok()?;
                let end = node_start + TextSize::try_from(old_index + old_len).ok()?;
                let new_text = new.get(new_index..new_index + new_len)?.into();
                line_index
                    .convert(TextRange::new(start, end))
                    .map(|range| TextEdit { range, new_text })
            }
            _ => None,
        })
        .collect();
    Some(text_edits)
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
