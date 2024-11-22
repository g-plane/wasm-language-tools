#![doc = include_str!("../README.md")]

use self::printer::DocGen;
use tiny_pretty::{IndentKind, PrintOptions};
use wat_syntax::ast::Root;

pub mod config;
mod printer;

/// Print the given concrete syntax tree.
pub fn format(root: &Root, options: &config::FormatOptions) -> String {
    let ctx = printer::Ctx {
        indent_width: options.layout.indent_width,
        options: &options.language,
    };
    tiny_pretty::print(
        &root.doc(&ctx),
        &PrintOptions {
            indent_kind: if options.layout.use_tabs {
                IndentKind::Tab
            } else {
                IndentKind::Space
            },
            line_break: options.layout.line_break.clone().into(),
            width: options.layout.print_width,
            tab_size: options.layout.indent_width,
        },
    )
}
