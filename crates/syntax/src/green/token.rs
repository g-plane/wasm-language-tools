use super::GreenHead;
use crate::SyntaxKind;
use std::fmt;
use text_size::TextSize;
use triomphe::ThinArc;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GreenToken {
    data: ThinArc<GreenHead, u8>,
}

impl GreenToken {
    #[inline]
    pub fn new(kind: SyntaxKind, text: &str) -> Self {
        GreenToken {
            data: ThinArc::from_header_and_slice(
                GreenHead {
                    kind,
                    text_len: TextSize::of(text),
                },
                text.as_bytes(),
            ),
        }
    }

    #[inline]
    pub fn kind(&self) -> SyntaxKind {
        self.data.header.header.kind
    }

    #[inline]
    pub fn text_len(&self) -> TextSize {
        self.data.header.header.text_len
    }

    #[inline]
    pub fn text(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.data.slice) }
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
