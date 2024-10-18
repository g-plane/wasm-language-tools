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

pub fn lsp_pos_to_rowan_pos(line_index: &LineIndex, pos: Position) -> Option<TextSize> {
    line_index.offset(LineCol {
        line: pos.line,
        col: pos.character,
    })
}

pub fn lsp_range_to_rowan_range(line_index: &LineIndex, range: Range) -> Option<TextRange> {
    line_index
        .offset(LineCol {
            line: range.start.line,
            col: range.start.character,
        })
        .zip(line_index.offset(LineCol {
            line: range.end.line,
            col: range.end.character,
        }))
        .map(|(start, end)| TextRange::new(TextSize::new(start.into()), TextSize::new(end.into())))
}

pub(crate) mod ast {
    use rowan::{GreenNode, NodeOrToken};
    use wat_syntax::SyntaxKind;

    pub fn find_func_type_of_type_def(green: &GreenNode) -> Option<GreenNode> {
        green.children().find_map(|child| match child {
            NodeOrToken::Node(node) if node.kind() == SyntaxKind::FUNC_TYPE.into() => {
                Some(node.to_owned())
            }
            _ => None,
        })
    }
}
