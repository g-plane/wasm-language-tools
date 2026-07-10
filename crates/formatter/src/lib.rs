#![doc = include_str!("../README.md")]

use self::{
    config::FormatOptions,
    printer::{Ctx, format_root},
};
use tiny_pretty::{IndentKind, PrintOptions};
use wat_syntax::{AmberNode, GreenNode};

pub mod config;
mod printer;

/// Print the whole syntax tree.
pub fn format(root: &GreenNode, options: &FormatOptions) -> String {
    let ctx = Ctx::new(options);
    tiny_pretty::print(&format_root(AmberNode::new_root(root), &ctx), &build_options(options))
}

/// Print a specific syntax node.
pub fn format_node(node: AmberNode, options: &FormatOptions, base_indent: usize) -> Option<String> {
    let ctx = Ctx::new(options);
    crate::printer::format_node(node, &ctx)
        .map(|doc| tiny_pretty::print(&doc.nest(base_indent), &build_options(options)))
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
