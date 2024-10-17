use line_index::{LineCol, LineIndex};
use lsp_types::{Position, Range};
use rowan::{TextRange, TextSize};

pub fn rowan_pos_to_lsp_pos(line_index: &LineIndex, pos: TextSize) -> Position {
    let line_col = line_index.line_col(pos);
    Position::new(line_col.line, line_col.col)
}

pub fn rowan_range_to_lsp_range(line_index: &LineIndex, range: TextRange) -> Range {
    Range::new(
        rowan_pos_to_lsp_pos(line_index, range.start()),
        rowan_pos_to_lsp_pos(line_index, range.end()),
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
