#[cfg(feature = "config_serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(default))]
/// The whole configuration.
pub struct FormatOptions {
    #[cfg_attr(feature = "config_serde", serde(flatten))]
    pub layout: LayoutOptions,
    #[cfg_attr(feature = "config_serde", serde(flatten))]
    pub language: LanguageOptions,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(default))]
/// Configuration related to layout, such as indentation or print width.
pub struct LayoutOptions {
    #[cfg_attr(feature = "config_serde", serde(alias = "printWidth"))]
    pub print_width: usize,

    #[cfg_attr(feature = "config_serde", serde(alias = "indentWidth"))]
    pub indent_width: usize,

    #[cfg_attr(
        feature = "config_serde",
        serde(alias = "lineBreak", alias = "linebreak")
    )]
    pub line_break: LineBreak,

    #[cfg_attr(feature = "config_serde", serde(alias = "useTabs"))]
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

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "kebab-case"))]
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(default))]
/// Configuration related to syntax.
pub struct LanguageOptions {
    #[cfg_attr(feature = "config_serde", serde(alias = "formatComments"))]
    pub format_comments: bool,

    #[cfg_attr(feature = "config_serde", serde(alias = "ignoreCommentDirective"))]
    pub ignore_comment_directive: String,
}

impl Default for LanguageOptions {
    fn default() -> Self {
        Self {
            format_comments: false,
            ignore_comment_directive: "fmt-ignore".to_string(),
        }
    }
}
