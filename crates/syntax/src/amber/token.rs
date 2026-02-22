use crate::{GreenToken, SyntaxKind, SyntaxToken};
use text_size::{TextRange, TextSize};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
/// Leaf token in the amber syntax tree.
///
/// It's a lightweight version of [`SyntaxToken`](crate::SyntaxToken) without access to parent and siblings.
/// It's much cheaper than [`SyntaxToken`](crate::SyntaxToken) to create and use.
/// This is preferred to use for better performance if you don't need to visit parent and siblings.
pub struct AmberToken<'a> {
    green: &'a GreenToken,
    range: TextRange,
}

impl<'a> AmberToken<'a> {
    #[inline]
    pub(crate) fn new(green: &'a GreenToken, start: TextSize) -> Self {
        Self {
            green,
            range: TextRange::new(start, start + green.text_len()),
        }
    }

    #[inline]
    /// Kind of this token.
    pub fn kind(&self) -> SyntaxKind {
        self.green.kind()
    }

    #[inline]
    /// The range that this token covers in the original text.
    pub fn text_range(&self) -> TextRange {
        self.range
    }

    #[inline]
    /// The underlying green token of this red token.
    pub fn green(&self) -> &'a GreenToken {
        self.green
    }

    #[inline]
    /// Text of this token.
    pub fn text(&self) -> &'a str {
        self.green.text()
    }
}

impl<'a> From<&'a SyntaxToken> for AmberToken<'a> {
    #[inline]
    fn from(token: &'a SyntaxToken) -> Self {
        Self {
            green: token.green(),
            range: token.text_range(),
        }
    }
}
