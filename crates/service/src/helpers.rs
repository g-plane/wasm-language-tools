use line_index::{LineCol, LineIndex};
use lsp_types::{Position, Range};
use rowan::{TextRange, TextSize};

pub fn rowan_range_to_lsp_range(line_index: &LineIndex, range: TextRange) -> Range {
    let start = line_index.line_col(range.start());
    let end = line_index.line_col(range.end());
    Range::new(
        Position::new(start.line, start.col),
        Position::new(end.line, end.col),
    )
}

pub fn lsp_range_to_rowan_range(line_index: &LineIndex, range: Range) -> Option<TextRange> {
    let start = line_index.offset(LineCol {
        line: range.start.line,
        col: range.start.character,
    })?;
    let end = line_index.offset(LineCol {
        line: range.end.line,
        col: range.end.character,
    })?;
    Some(TextRange::new(
        TextSize::new(start.into()),
        TextSize::new(end.into()),
    ))
}
