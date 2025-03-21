use crate::binder::Symbol;
use line_index::{LineCol, LineIndex};
use lspt::{Position, Range};
use rowan::{
    ast::{support, AstNode},
    TextRange, TextSize,
};
use wat_syntax::{
    ast::{CompType, TypeDef},
    SyntaxKind, SyntaxNode,
};

pub fn rowan_pos_to_lsp_pos(line_index: &LineIndex, pos: TextSize) -> Position {
    let line_col = line_index.line_col(pos);
    Position {
        line: line_col.line,
        character: line_col.col,
    }
}

pub fn rowan_range_to_lsp_range(line_index: &LineIndex, range: TextRange) -> Range {
    Range {
        start: rowan_pos_to_lsp_pos(line_index, range.start()),
        end: rowan_pos_to_lsp_pos(line_index, range.end()),
    }
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
        .map(|(start, end)| TextRange::new(start, end))
}

pub fn fuzzy_search<S>(haystack: impl IntoIterator<Item = S>, needle: &str) -> Option<S>
where
    S: AsRef<str>,
{
    use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

    let matcher = SkimMatcherV2::default();
    haystack
        .into_iter()
        .filter_map(|name| {
            matcher
                .fuzzy_match(name.as_ref(), needle)
                .map(|score| (score, name))
        })
        .max_by_key(|(score, _)| *score)
        .map(|(_, guess)| guess)
}

pub fn can_produce_never(instr_name: &str) -> bool {
    matches!(
        instr_name,
        "unreachable"
            | "return"
            | "br"
            | "br_table"
            | "return_call"
            | "return_call_indirect"
            | "return_call_ref"
    )
}

pub fn create_selection_range(symbol: &Symbol, root: &SyntaxNode, line_index: &LineIndex) -> Range {
    let node = symbol.key.to_node(root);
    let range = support::token(&node, SyntaxKind::IDENT)
        .or_else(|| support::token(&node, SyntaxKind::KEYWORD))
        .map(|token| token.text_range())
        .unwrap_or_else(|| node.text_range());
    rowan_range_to_lsp_range(line_index, range)
}

pub fn infer_type_def_symbol_detail(symbol: &Symbol, root: &SyntaxNode) -> Option<String> {
    TypeDef::cast(symbol.key.to_node(root))
        .and_then(|node| node.sub_type())
        .and_then(|sub_type| sub_type.comp_type())
        .map(|comp_type| match comp_type {
            CompType::Array(..) => "array".into(),
            CompType::Struct(..) => "struct".into(),
            CompType::Func(..) => "func".into(),
        })
}

pub(crate) mod ast {
    use rowan::{
        ast::{support, AstNode},
        Direction, GreenNode, NodeOrToken, TextSize, TokenAtOffset,
    };
    use wat_syntax::{ast::RefType, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

    pub fn find_func_type_of_type_def(green: &GreenNode) -> Option<GreenNode> {
        green
            .children()
            .find_map(|child| match child {
                NodeOrToken::Node(node) if node.kind() == SyntaxKind::SUB_TYPE.into() => Some(node),
                _ => None,
            })
            .and_then(|sub_type| {
                sub_type.children().find_map(|child| match child {
                    NodeOrToken::Node(node) if node.kind() == SyntaxKind::FUNC_TYPE.into() => {
                        Some(node.to_owned())
                    }
                    _ => None,
                })
            })
    }

    pub fn is_call(node: &SyntaxNode) -> bool {
        support::token(node, SyntaxKind::INSTR_NAME)
            .is_some_and(|token| matches!(token.text(), "call" | "ref.func" | "return_call"))
    }

    pub fn find_token(root: &SyntaxNode, offset: TextSize) -> Option<SyntaxToken> {
        match root.token_at_offset(offset) {
            TokenAtOffset::None => None,
            TokenAtOffset::Single(token) => Some(token),
            TokenAtOffset::Between(left, _) => Some(left),
        }
    }

    pub fn get_doc_comment(node: &SyntaxNode) -> String {
        node.siblings_with_tokens(Direction::Prev)
            .skip(1)
            .map_while(|element| match element {
                SyntaxElement::Token(token)
                    if matches!(
                        token.kind(),
                        SyntaxKind::LINE_COMMENT | SyntaxKind::WHITESPACE
                    ) =>
                {
                    Some(token)
                }
                _ => None,
            })
            .filter(|token| token.kind() == SyntaxKind::LINE_COMMENT)
            .skip_while(|token| !token.text().starts_with(";;;"))
            .take_while(|token| token.text().starts_with(";;;"))
            .fold(String::new(), |mut doc, comment| {
                if !doc.is_empty() {
                    doc.insert(0, '\n');
                }
                if let Some(text) = comment.text().strip_prefix(";;;") {
                    doc.insert_str(0, text.strip_prefix([' ', '\t']).unwrap_or(text));
                }
                doc
            })
    }

    pub fn is_nullable_ref_type(ref_type: &RefType) -> bool {
        ref_type
            .syntax()
            .children_with_tokens()
            .any(|element| match element {
                SyntaxElement::Token(token) => {
                    let kind = token.kind();
                    kind == SyntaxKind::TYPE_KEYWORD
                        || kind == SyntaxKind::KEYWORD && token.text() == "null"
                }
                SyntaxElement::Node(_) => false,
            })
    }
}
