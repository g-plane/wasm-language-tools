use line_index::{LineCol, LineIndex};
use lspt::{Position, Range};
use rowan::{TextRange, TextSize};
use std::num::ParseIntError;

pub use self::arena::{BumpCollectionsExt, BumpHashMap, BumpHashSet};

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

pub fn parse_u32(s: &str) -> Result<u32, ParseIntError> {
    let s = s.replace('_', "");
    if let Some(s) = s.strip_prefix("0x") {
        u32::from_str_radix(s, 16)
    } else {
        s.parse()
    }
}

pub(crate) struct RenderWithDb<'db, T> {
    pub value: T,
    pub db: &'db dyn salsa::Database,
}

pub(crate) mod syntax {
    use crate::binder::Symbol;
    use rowan::{
        Direction, TextSize, TokenAtOffset,
        ast::{AstNode, support},
    };
    use std::ops::ControlFlow;
    use wat_syntax::{
        SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken,
        ast::{CompType, ExternIdx, TypeDef},
    };

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

pub(crate) mod arena {
    use bumpalo::Bump;
    use hashbrown::{HashMap, HashSet};
    use rustc_hash::FxBuildHasher;
    use std::hash::Hash;

    pub trait BumpCollectionsExt<'bump, T> {
        fn new_in(bump: &'bump Bump) -> Self;
        fn with_capacity_in(capacity: usize, bump: &'bump Bump) -> Self;
        fn from_iter_in<I>(iter: I, bump: &'bump Bump) -> Self
        where
            I: IntoIterator<Item = T>;
    }

    pub type BumpHashMap<'bump, K, V> = HashMap<K, V, FxBuildHasher, &'bump Bump>;
    impl<'bump, K, V> BumpCollectionsExt<'bump, (K, V)> for BumpHashMap<'bump, K, V>
    where
        K: Eq + Hash,
    {
        #[inline]
        fn new_in(bump: &'bump Bump) -> Self {
            HashMap::with_hasher_in(FxBuildHasher, bump)
        }
        #[inline]
        fn with_capacity_in(capacity: usize, bump: &'bump Bump) -> Self {
            HashMap::with_capacity_and_hasher_in(capacity, FxBuildHasher, bump)
        }
        #[inline]
        fn from_iter_in<I>(iter: I, bump: &'bump Bump) -> Self
        where
            I: IntoIterator<Item = (K, V)>,
        {
            let iter = iter.into_iter();
            let capacity = iter.size_hint().0;
            iter.fold(Self::with_capacity_in(capacity, bump), |mut map, (k, v)| {
                map.insert(k, v);
                map
            })
        }
    }

    pub type BumpHashSet<'bump, T> = HashSet<T, FxBuildHasher, &'bump Bump>;
    impl<'bump, T> BumpCollectionsExt<'bump, T> for BumpHashSet<'bump, T>
    where
        T: Eq + Hash,
    {
        #[inline]
        fn new_in(bump: &'bump Bump) -> Self {
            HashSet::with_hasher_in(FxBuildHasher, bump)
        }
        #[inline]
        fn with_capacity_in(capacity: usize, bump: &'bump Bump) -> Self {
            HashSet::with_capacity_and_hasher_in(capacity, FxBuildHasher, bump)
        }
        #[inline]
        fn from_iter_in<I>(iter: I, bump: &'bump Bump) -> Self
        where
            I: IntoIterator<Item = T>,
        {
            let iter = iter.into_iter();
            let capacity = iter.size_hint().0;
            iter.fold(Self::with_capacity_in(capacity, bump), |mut set, k| {
                set.insert(k);
                set
            })
        }
    }
}
