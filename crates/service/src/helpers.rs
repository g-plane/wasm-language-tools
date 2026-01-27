use crate::binder::{Symbol, SymbolKey};
use line_index::{LineCol, LineIndex};
use lspt::{Location, Position, Range};
use rowan::{
    TextRange, TextSize,
    ast::{AstNode, support},
};
use wat_syntax::{
    SyntaxKind, SyntaxNode,
    ast::{CompType, TypeDef},
};

pub trait LineIndexExt<In> {
    type Out;
    fn convert(&self, input: In) -> Self::Out;
}
impl LineIndexExt<TextSize> for LineIndex {
    type Out = Position;
    /// Convert rowan offset to LSP position.
    fn convert(&self, input: TextSize) -> Self::Out {
        let line_col = self.line_col(input);
        Position {
            line: line_col.line,
            character: line_col.col,
        }
    }
}
impl LineIndexExt<TextRange> for LineIndex {
    type Out = Range;
    /// Convert rowan range to LSP range.
    fn convert(&self, input: TextRange) -> Self::Out {
        Range {
            start: self.convert(input.start()),
            end: self.convert(input.end()),
        }
    }
}
impl LineIndexExt<Position> for LineIndex {
    type Out = Option<TextSize>;
    /// Convert LSP position to rowan offset.
    fn convert(&self, input: Position) -> Self::Out {
        self.offset(LineCol {
            line: input.line,
            col: input.character,
        })
    }
}
impl LineIndexExt<Range> for LineIndex {
    type Out = Option<TextRange>;
    /// Convert LSP range to rowan range.
    fn convert(&self, input: Range) -> Self::Out {
        self.offset(LineCol {
            line: input.start.line,
            col: input.start.character,
        })
        .zip(self.offset(LineCol {
            line: input.end.line,
            col: input.end.character,
        }))
        .map(|(start, end)| TextRange::new(start, end))
    }
}

// https://webassembly.github.io/spec/core/valid/instructions.html#polymorphism
pub fn is_stack_polymorphic(instr_name: &str) -> bool {
    matches!(
        instr_name,
        "unreachable"
            | "return"
            | "br"
            | "br_table"
            | "return_call"
            | "return_call_indirect"
            | "return_call_ref"
            | "throw"
            | "throw_ref"
    )
}

pub fn create_selection_range(symbol: &Symbol, root: &SyntaxNode, line_index: &LineIndex) -> Range {
    let node = symbol.key.to_node(root);
    let range = support::token(&node, SyntaxKind::IDENT)
        .or_else(|| support::token(&node, SyntaxKind::KEYWORD))
        .map(|token| token.text_range())
        .unwrap_or_else(|| node.text_range());
    line_index.convert(range)
}

pub fn create_location_by_symbol(
    uri: String,
    line_index: &LineIndex,
    symbol_key: SymbolKey,
    root: &SyntaxNode,
) -> Location {
    let node = symbol_key.to_node(root);
    let range = support::token(&node, SyntaxKind::IDENT)
        .or_else(|| support::token(&node, SyntaxKind::KEYWORD))
        .map(|token| token.text_range())
        .unwrap_or_else(|| node.text_range());
    Location {
        uri,
        range: line_index.convert(range),
    }
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

pub(crate) struct RenderWithDb<'db, T> {
    pub value: T,
    pub db: &'db dyn salsa::Database,
}

pub(crate) mod ast {
    use rowan::{
        Direction, TextSize, TokenAtOffset,
        ast::{AstNode, support},
    };
    use std::ops::ControlFlow;
    use wat_syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken, ast::ExternIdx};

    /// Pick the `$idx` part from `(func (type $idx) ...)`.
    /// It will return `None` if there're inlined params or results.
    pub fn pick_type_idx_from_func(func: &SyntaxNode) -> Option<SyntaxNode> {
        if let ControlFlow::Continue(Some(index)) = func
            .first_child_by_kind(&|kind| kind == SyntaxKind::TYPE_USE)
            .into_iter()
            .flat_map(|type_use| type_use.children())
            .try_fold(None, |r, child| match child.kind() {
                SyntaxKind::PARAM | SyntaxKind::RESULT => ControlFlow::Break(()),
                SyntaxKind::INDEX => ControlFlow::Continue(Some(child)),
                _ => ControlFlow::Continue(r),
            })
        {
            Some(index)
        } else {
            None
        }
    }

    pub fn extract_index_from_export(module_field_export: &SyntaxNode) -> Option<SyntaxNode> {
        module_field_export
            .first_child_by_kind(&ExternIdx::can_cast)
            .and_then(|extern_idx| extern_idx.first_child_by_kind(&|kind| kind == SyntaxKind::INDEX))
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
            .map_while(|node_or_token| match node_or_token {
                SyntaxElement::Token(token) if token.kind().is_trivia() => Some(token),
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
}

pub(crate) mod locals {
    use super::*;
    use crate::{
        binder::{SymbolKind, SymbolTable},
        document::Document,
    };

    #[salsa::tracked]
    pub fn has_locals(db: &dyn salsa::Database, document: Document, func_key: SymbolKey) -> bool {
        SymbolTable::of(db, document)
            .symbols
            .values()
            .any(|symbol| symbol.region == func_key && symbol.kind == SymbolKind::Local)
    }
}
