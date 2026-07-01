use super::GreenHead;
use crate::SyntaxKind;
use servo_arc::ThinArc;
use std::{fmt, hash};
use text_size::TextSize;

#[derive(Clone, PartialEq, Eq)]
/// Leaf token in the green syntax tree.
pub struct GreenToken {
    data: ThinArc<GreenHead, u8>,
}

impl GreenToken {
    #[inline]
    /// Create a new token.
    pub fn new(kind: SyntaxKind, text: &str) -> Self {
        GreenToken {
            data: ThinArc::from_header_and_iter(
                GreenHead {
                    kind,
                    text_len: TextSize::of(text),
                },
                text.bytes(),
            ),
        }
    }

    #[inline]
    /// Kind of this token.
    pub fn kind(&self) -> SyntaxKind {
        self.data.header.kind
    }

    #[inline]
    /// Text of this token.
    pub fn text(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.data.slice()) }
    }

    #[inline]
    /// Length of the text of this token.
    pub(crate) fn text_len(&self) -> TextSize {
        self.data.header.text_len
    }
}

impl fmt::Debug for GreenToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GreenToken")
            .field("kind", &self.kind())
            .field("text", &self.text())
            .finish()
    }
}

impl fmt::Display for GreenToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.text().fmt(f)
    }
}

impl hash::Hash for GreenToken {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.data.header.hash(state);
        self.data.slice().hash(state);
    }
}
