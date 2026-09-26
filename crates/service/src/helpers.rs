use crate::binder::{Symbol, SymbolTable};
use std::{borrow::Cow, num::ParseIntError};
use wat_syntax::{NodeOrToken, SyntaxKind};

pub use self::arena::{BumpCollectionsExt, BumpHashMap, BumpHashSet};

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
    let s = clean_underscores(s);
    if let Some(s) = s.strip_prefix("0x") {
        u32::from_str_radix(s, 16)
    } else {
        s.parse()
    }
}
pub fn parse_u64(s: &str) -> Result<u64, ParseIntError> {
    let s = clean_underscores(s);
    if let Some(s) = s.strip_prefix("0x") {
        u64::from_str_radix(s, 16)
    } else {
        s.parse()
    }
}
fn clean_underscores(s: &str) -> Cow<'_, str> {
    if s.contains('_') {
        Cow::from(s.replace('_', ""))
    } else {
        Cow::from(s)
    }
}

pub fn get_doc_comment(def_symbol: &Symbol, symbol_table: &SymbolTable) -> Option<String> {
    let node = def_symbol.amber();
    symbol_table.symbols.get(def_symbol.region).map(|module| {
        module
            .amber()
            .children_with_tokens()
            .rev()
            .skip_while(|node_or_token| node_or_token.text_range().start() >= node.text_range().start())
            .map_while(|node_or_token| match node_or_token {
                NodeOrToken::Token(token) if token.kind().is_trivia() => Some(token),
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
    })
}

pub(crate) struct RenderWithDb<'db, T> {
    pub value: T,
    pub db: &'db dyn salsa::Database,
}

pub(crate) mod syntax {
    use std::{
        hash::Hash,
        ops::{ControlFlow, Deref},
    };
    use wat_syntax::{
        AmberNode, GreenNode, NodeOrToken, SyntaxKind, TextRange,
        ast::{AstNode, ExternIdx},
    };

    /// Pick the `$idx` part from `(func (type $idx) ...)`.
    /// It will return `None` if there're inlined params or results.
    pub fn pick_type_idx_from_func<'a>(func: AmberNode<'a>) -> Option<AmberNode<'a>> {
        if let ControlFlow::Continue(Some(index)) = func
            .children_by_kind(SyntaxKind::TYPE_USE)
            .next()
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

    pub fn extract_index_from_export<'a>(module_field_export: AmberNode<'a>) -> Option<AmberNode<'a>> {
        module_field_export
            .children_by_kind(ExternIdx::can_cast)
            .next()
            .and_then(|extern_idx| extern_idx.children_by_kind(SyntaxKind::INDEX).next())
    }

    /// This returns a 2-component tuple.
    /// The first component is the node that contains given descendant node;
    /// the second component is the node that contains accessible block types.
    pub fn find_outer_block_for_types<'a>(
        module: AmberNode<'a>,
        descendant: AmberNode<'a>,
    ) -> (AmberNode<'a>, Option<AmberNode<'a>>) {
        descendant
            .path_from(module)
            .fold((module, None), |acc, node| match node.kind() {
                SyntaxKind::MODULE_FIELD_FUNC => (node, None),
                SyntaxKind::BLOCK_BLOCK | SyntaxKind::BLOCK_LOOP | SyntaxKind::BLOCK_TRY_TABLE => (node, Some(node)),
                SyntaxKind::BLOCK_IF => (node, acc.1),
                SyntaxKind::BLOCK_IF_THEN | SyntaxKind::BLOCK_IF_ELSE => (node, Some(acc.0)),
                _ => acc,
            })
    }

    pub fn infer_def_poi(node: AmberNode) -> TextRange {
        if node.kind() == SyntaxKind::REF_TYPE {
            node.text_range()
        } else {
            match node
                .children_with_tokens()
                .try_fold(node.text_range(), |range, node_or_token| match node_or_token {
                    NodeOrToken::Node(..) => ControlFlow::Break(range),
                    NodeOrToken::Token(token) => {
                        if matches!(token.kind(), SyntaxKind::KEYWORD | SyntaxKind::IDENT) {
                            ControlFlow::Continue(token.text_range())
                        } else {
                            ControlFlow::Continue(range)
                        }
                    }
                }) {
                ControlFlow::Continue(range) => range,
                ControlFlow::Break(range) => range,
            }
        }
    }

    #[derive(Clone)]
    /// Wrapper type for `GreenNode` that implements `Hash` and `Eq` based on ptr for Salsa tracked query.
    pub struct GreenNodeKey(GreenNode);
    impl Deref for GreenNodeKey {
        type Target = GreenNode;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl PartialEq for GreenNodeKey {
        fn eq(&self, other: &Self) -> bool {
            self.0.raw_ptr() == other.0.raw_ptr()
        }
    }
    impl Eq for GreenNodeKey {}
    impl Hash for GreenNodeKey {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.raw_ptr().hash(state);
        }
    }
    impl From<GreenNode> for GreenNodeKey {
        fn from(node: GreenNode) -> Self {
            Self(node)
        }
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
