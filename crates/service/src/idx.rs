use crate::helpers::{self, RenderWithDb};
use std::fmt;
use wat_syntax::{GreenNode, NodeOrToken, SyntaxKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub struct Idx<'db> {
    pub num: Option<u32>,
    pub name: Option<InternIdent<'db>>,
}

impl<'db> Idx<'db> {
    pub fn from_immediate(node: &GreenNode, db: &'db dyn salsa::Database) -> Option<Self> {
        node.children().next().and_then(|child| match child {
            NodeOrToken::Token(token) if token.kind() == SyntaxKind::INT => Some(Idx {
                num: helpers::parse_u32(token.text()).ok(),
                name: None,
            }),
            NodeOrToken::Token(token) if token.kind() == SyntaxKind::IDENT => Some(Idx {
                num: None,
                name: Some(InternIdent::new(db, token.text())),
            }),
            _ => None,
        })
    }

    pub fn is_def(&self) -> bool {
        matches!(self, Idx { num: Some(..), .. })
    }

    pub fn is_ref(&self) -> bool {
        matches!(
            self,
            Idx {
                num: None,
                name: Some(..),
            } | Idx {
                num: Some(..),
                name: None,
            }
        )
    }

    pub fn is_defined_by(&self, other: &Self) -> bool {
        debug_assert!(self.is_ref());
        debug_assert!(other.is_def());
        match (self, other) {
            (
                Idx { num: Some(num), .. },
                Idx {
                    num: Some(other_num), ..
                },
            ) => num == other_num,
            (
                Idx { name: Some(name), .. },
                Idx {
                    name: Some(other_name), ..
                },
            ) => name == other_name,
            _ => false,
        }
    }

    pub fn render(&self, db: &'db dyn salsa::Database) -> RenderWithDb<'db, &Self> {
        RenderWithDb { value: self, db }
    }
}

impl fmt::Display for RenderWithDb<'_, &Idx<'_>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.value.name {
            name.ident(self.db).fmt(f)
        } else if let Some(num) = self.value.num {
            num.fmt(f)
        } else {
            Ok(())
        }
    }
}

#[salsa::interned(debug)]
pub(crate) struct InternIdent<'db> {
    #[returns(ref)]
    pub(crate) ident: String,
}

#[derive(Default)]
pub(crate) struct IdxGen(u32);

impl IdxGen {
    /// Get numeric idx then increment for next.
    pub fn pull(&mut self) -> u32 {
        let idx = self.0;
        self.0 += 1;
        idx
    }

    /// Reset idx generator.
    pub fn reset(&mut self) {
        self.0 = 0;
    }
}
