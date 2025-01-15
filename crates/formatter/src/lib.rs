#![doc = include_str!("../README.md")]

use self::{
    config::FormatOptions,
    printer::{format_node, Ctx, DocGen},
};
use line_index::LineIndex;
use rowan::{ast::AstNode, TextRange};
use tiny_pretty::{IndentKind, PrintOptions};
use wat_syntax::{ast::Root, SyntaxElement};

pub mod config;
mod printer;

/// Print the given concrete syntax tree.
pub fn format(root: &Root, options: &FormatOptions) -> String {
    let ctx = Ctx::new(options);
    tiny_pretty::print(&root.doc(&ctx), &build_options(options))
}

/// Print a specific range from a root syntax tree.
/// Returned string reflects specific syntax node, not full.
/// Affected range may be wider than requested range, which will be returned.
/// This returned range should be used when replacing text.
pub fn format_range(
    root: &Root,
    options: &FormatOptions,
    range: TextRange,
    line_index: &LineIndex,
) -> Option<(String, TextRange)> {
    let mut node = root.syntax().clone();
    while let Some(SyntaxElement::Node(it)) = node.child_or_token_at_range(range) {
        node = it;
    }
    let range = node.text_range();
    let col = line_index.line_col(range.start()).col as usize;

    let ctx = Ctx::new(options);
    let doc = format_node(node, &ctx)?.nest(col);
    Some((tiny_pretty::print(&doc, &build_options(options)), range))
}

fn build_options(options: &FormatOptions) -> PrintOptions {
    PrintOptions {
        indent_kind: if options.layout.use_tabs {
            IndentKind::Tab
        } else {
            IndentKind::Space
        },
        line_break: options.layout.line_break.clone().into(),
        width: options.layout.print_width,
        tab_size: options.layout.indent_width,
    }
}
