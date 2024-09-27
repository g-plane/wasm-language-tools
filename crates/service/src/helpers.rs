use line_index::LineIndex;
use lsp_types::{Position, Range};

pub fn rowan_range_to_lsp_range(line_index: &LineIndex, range: rowan::TextRange) -> Range {
    let start = line_index.line_col(range.start());
    let end = line_index.line_col(range.end());
    Range::new(
        Position::new(start.line, start.col),
        Position::new(end.line, end.col),
    )
}
