use crate::{GreenToken, SyntaxKind, SyntaxToken};
use text_size::{TextRange, TextSize};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
/// `AmberToken` is a lightweight version of [`SyntaxToken`](crate::SyntaxToken) that doesn't allocate on the heap.
/// It's pretty cheaper than `SyntaxToken`, but you can't visit parent and siblings.
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
    pub fn kind(&self) -> SyntaxKind {
        self.green.kind()
    }

    #[inline]
    pub fn text_range(&self) -> TextRange {
        self.range
    }

    #[inline]
    pub fn green(&self) -> &'a GreenToken {
        self.green
    }

    #[inline]
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
