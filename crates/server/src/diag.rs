#[derive(Clone, Debug, Hash)]
pub struct Diagnostic {
    pub start: usize,
    pub end: usize,
    pub message: String,
}
