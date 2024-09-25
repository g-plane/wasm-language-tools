use lsp_types::Range;

#[derive(Clone, Debug, Hash)]
pub struct Diagnostic {
    pub range: Range,
    pub message: String,
}
