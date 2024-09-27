use lsp_types::Range;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Diagnostic {
    pub range: Range,
    pub message: String,
}
