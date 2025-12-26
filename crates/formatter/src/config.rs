//! Configuration-related types.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
/// The whole configuration.
pub struct FormatOptions {
    #[serde(flatten)]
    pub layout: LayoutOptions,
    #[serde(flatten)]
    pub language: LanguageOptions,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
/// Configuration related to layout, such as indentation or print width.
pub struct LayoutOptions {
    #[serde(alias = "printWidth")]
    /// The line width limitation that formatter should *(but not must)* avoid exceeding.
    /// The formatter will try its best to keep line width less than this value,
    /// but it may exceed for some cases, for example, a very very long single word.
    ///
    /// Default: `80`
    pub print_width: usize,

    #[serde(alias = "indentWidth")]
    /// Size of indentation. When enabled `useTabs`, this option may be disregarded,
    /// since only one tab will be inserted when indented once.
    ///
    /// Default: `2`
    ///
    /// Panics if value is `0`.
    pub indent_width: usize,

    #[serde(alias = "lineBreak", alias = "linebreak")]
    /// Specify use `\n` (LF) or `\r\n` (CRLF) for line break.
    ///
    /// Default: `Lf`
    pub line_break: LineBreak,

    #[serde(alias = "useTabs")]
    /// Specify use space or tab for indentation.
    ///
    /// Default: `false`
    pub use_tabs: bool,
}

impl Default for LayoutOptions {
    fn default() -> Self {
        Self {
            print_width: 80,
            indent_width: 2,
            line_break: LineBreak::Lf,
            use_tabs: false,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LineBreak {
    #[default]
    Lf,
    Crlf,
}

impl From<LineBreak> for tiny_pretty::LineBreak {
    fn from(value: LineBreak) -> Self {
        match value {
            LineBreak::Lf => tiny_pretty::LineBreak::Lf,
            LineBreak::Crlf => tiny_pretty::LineBreak::Crlf,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
/// Configuration related to syntax.
pub struct LanguageOptions {
    #[serde(alias = "splitClosingParens")]
    /// Control whether closing parentheses should be splitted into different lines.
    ///
    /// Default: `false`
    pub split_closing_parens: bool,

    #[serde(alias = "wrapBeforeLocals")]
    /// Control whether to insert line break before function locals.
    ///
    /// Default: `Overflow`
    pub wrap_before_locals: WrapBefore,

    #[serde(alias = "wrapBeforeConstExpr")]
    /// Control whether to insert line break before constant expression.
    ///
    /// Default: `Always`
    pub wrap_before_const_expr: WrapBefore,

    #[serde(alias = "multiLineLocals")]
    /// Control how to insert whitespace between multiple locals in a function.
    ///
    /// Default: `Never`
    pub multi_line_locals: MultiLine,

    #[serde(alias = "formatComments")]
    /// Control whether whitespace should be inserted at the beginning and end of comments.
    ///
    /// Though this option is set to `false`,
    /// comments contain leading or trailing whitespace will still be kept as-is.
    ///
    /// Default: `false`
    pub format_comments: bool,

    #[serde(alias = "ignoreCommentDirective")]
    /// Text directive for ignoring formatting specific module or module field.
    ///
    /// Default: `"fmt-ignore"`
    pub ignore_comment_directive: String,
}

impl Default for LanguageOptions {
    fn default() -> Self {
        Self {
            split_closing_parens: false,
            wrap_before_locals: WrapBefore::Overflow,
            wrap_before_const_expr: WrapBefore::Always,
            multi_line_locals: MultiLine::Never,
            format_comments: false,
            ignore_comment_directive: "fmt-ignore".to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WrapBefore {
    Never,
    Overflow,
    #[serde(alias = "multiOnly")]
    MultiOnly,
    Always,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MultiLine {
    Never,
    Overflow,
    Smart,
    Always,
}
