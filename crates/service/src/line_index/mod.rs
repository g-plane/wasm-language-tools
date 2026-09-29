use lspt::{Position, Range};
use std::{cmp::Ordering, ops::ControlFlow};
use wat_syntax::{TextRange, TextSize};

#[cfg(target_arch = "aarch64")]
mod aarch64;
mod scalar;
#[cfg(target_arch = "x86_64")]
mod x86_64;

#[derive(Clone, Debug)]
pub struct LineIndex {
    lines: Vec<u32>,
    non_ascii_chars: Vec<NonAsciiChar>,
    len: u32,
}
impl LineIndex {
    pub fn new(text: &str) -> Self {
        let len = u32::try_from(text.len()).expect("text len must be less than 4 GiB");

        #[cfg(target_arch = "x86_64")]
        let (lines, non_ascii_chars) = if std::arch::is_x86_feature_detected!("avx2") {
            // SAFETY: AVX2 support is checked
            unsafe { self::x86_64::scan_avx2(text) }
        } else {
            self::scalar::scan_scalar(text)
        };
        #[cfg(target_arch = "aarch64")]
        let (lines, non_ascii_chars) = if std::arch::is_aarch64_feature_detected!("neon") {
            // SAFETY: NEON support is checked
            unsafe { self::aarch64::scan_neon(text) }
        } else {
            self::scalar::scan_scalar(text)
        };
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        let (lines, non_ascii_chars) = self::scalar::scan_scalar(text);

        Self {
            lines,
            non_ascii_chars,
            len,
        }
    }

    #[inline]
    pub fn convert<T>(&self, value: T) -> T::Out
    where
        T: LocationConvert,
    {
        value.convert(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct NonAsciiChar {
    offset: u32,
    utf8_width: u8,
    utf16_width: u8,
}

pub trait LocationConvert {
    type Out;
    fn convert(&self, line_index: &LineIndex) -> Self::Out;
}
impl LocationConvert for TextSize {
    type Out = Option<Position>;
    fn convert(&self, line_index: &LineIndex) -> Self::Out {
        let point = line_index.lines.partition_point(|line| *line <= u32::from(*self));
        let (line, line_start) = if let Some(index) = point.checked_sub(1) {
            (index as u32, line_index.lines.get(index)?)
        } else {
            return Some(Position { line: 0, character: 0 });
        };
        let offset = u32::from(*self);
        if line_index.non_ascii_chars.is_empty() {
            Some(Position {
                line,
                character: offset - line_start,
            })
        } else {
            let from = line_index
                .non_ascii_chars
                .partition_point(|char| char.offset < *line_start);
            line_index
                .non_ascii_chars
                .get(from..)?
                .iter()
                .take_while(|char| char.offset < offset)
                .try_fold(offset - line_start, |col, char| {
                    if offset < char.offset + char.utf8_width as u32 {
                        // invalid UTF-8 boundary
                        None
                    } else {
                        Some(col + char.utf16_width as u32 - char.utf8_width as u32)
                    }
                })
                .map(|character| Position { line, character })
        }
    }
}
impl LocationConvert for TextRange {
    type Out = Option<Range>;
    fn convert(&self, line_index: &LineIndex) -> Self::Out {
        let start = self.start().convert(line_index)?;
        let end = self.end().convert(line_index)?;
        Some(Range { start, end })
    }
}
impl LocationConvert for Position {
    type Out = Option<TextSize>;
    fn convert(&self, line_index: &LineIndex) -> Self::Out {
        let line_start = line_index.lines.get(self.line as usize)?;
        let next_line_start = line_index
            .lines
            .get(self.line as usize + 1)
            .map(|offset| *offset as usize);
        if line_index.non_ascii_chars.is_empty() {
            let offset = line_start.checked_add(self.character)?;
            if let Some(next_line_start) = next_line_start {
                if (offset as usize) < next_line_start {
                    Some(TextSize::new(offset))
                } else {
                    None
                }
            } else if offset <= line_index.len {
                Some(TextSize::new(offset))
            } else {
                None
            }
        } else {
            let line_end = next_line_start.map_or(line_index.len, |offset| offset as u32 - 1);
            let from = line_index
                .non_ascii_chars
                .partition_point(|char| char.offset < *line_start);
            match line_index
                .non_ascii_chars
                .get(from..)?
                .iter()
                .take_while(|char| char.offset < line_end)
                .try_fold((*line_start, 0), |(offset, col), char| {
                    // all chars between non-ASCII chars are ASCII
                    let ascii_len = char.offset - offset;
                    // calculate the UTF-16 column of current char
                    let col_before_current = col + ascii_len;
                    if self.character <= col_before_current {
                        ControlFlow::Break(Some(offset + (self.character - col)))
                    } else {
                        let offset_after_current = char.offset + char.utf8_width as u32;
                        let col_after_current = col_before_current + char.utf16_width as u32;
                        match self.character.cmp(&col_after_current) {
                            Ordering::Less => ControlFlow::Break(None), // invalid UTF-16 boundary
                            Ordering::Equal => ControlFlow::Break(Some(offset_after_current)),
                            Ordering::Greater => ControlFlow::Continue((offset_after_current, col_after_current)),
                        }
                    }
                }) {
                ControlFlow::Break(Some(offset)) => Some(TextSize::new(offset)),
                ControlFlow::Break(None) => None,
                ControlFlow::Continue((offset, col)) => {
                    let ascii_len = line_end - offset;
                    let col_line_end = col + ascii_len;
                    if self.character <= col_line_end {
                        Some(TextSize::new(offset + (self.character - col)))
                    } else {
                        None
                    }
                }
            }
        }
    }
}
impl LocationConvert for Range {
    type Out = Option<TextRange>;
    fn convert(&self, line_index: &LineIndex) -> Self::Out {
        let start = self.start.convert(line_index)?;
        let end = self.end.convert(line_index)?;
        if start <= end {
            Some(TextRange::new(start, end))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_vs_simd() {
        #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
        let text = "(module
  (type $t (func))
  (;😈;)(table $t1 10 (ref null func))
  (table $t2 10 (ref null $t))
  (elem $el funcref)
  (func $f
    (table.init $t1 $el
      (i32.const 0)
      (i32.const 1)
      (i32.const 2))
    (table.copy $t1 $t2
      (i32.const 0)
      (i32.const 1)
      (i32.const 2))))
";
        #[cfg(target_arch = "x86_64")]
        if std::arch::is_x86_feature_detected!("avx2") {
            unsafe {
                assert_eq!(super::scalar::scan_scalar(text), super::x86_64::scan_avx2(text));
            }
        }
        #[cfg(target_arch = "aarch64")]
        if std::arch::is_aarch64_feature_detected!("neon") {
            unsafe {
                assert_eq!(super::scalar::scan_scalar(text), super::aarch64::scan_neon(text));
            }
        }
    }

    #[test]
    fn ascii() {
        let text = "(module
  (type $t (func))
  (table $t1 10 (ref null func))
  (table $t2 10 (ref null $t))
  (elem $el funcref)
  (func $f
    (table.init $t1 $el
      (i32.const 0)
      (i32.const 1)
      (i32.const 2))
    (table.copy $t1 $t2
      (i32.const 0)
      (i32.const 1)
      (i32.const 2))))
";
        let line_index = LineIndex::new(text);
        assert_eq!(
            line_index.convert(Position { line: 4, character: 18 }).unwrap(),
            TextSize::new(109),
        );
        assert_eq!(
            line_index.convert(Position { line: 4, character: 20 }).unwrap(),
            TextSize::new(111),
        );
        assert!(line_index.convert(Position { line: 4, character: 21 }).is_none());
        assert_eq!(
            line_index.convert(Position { line: 5, character: 0 }).unwrap(),
            TextSize::new(112),
        );
    }

    #[test]
    fn offset_to_position() {
        let text = "
(module
  (type $t (func))
  (table $t1 10 (ref null func))
  (table $t2 10 (ref null $t))
  (elem $el funcref)
  (func $f (;😈🍔;)
    (table.init $t1 $el
      (i32.const 0)
      (i32.const 1)
      (i32.const 2))
    (table.copy $t1 $t2
      (i32.const 0)
      (i32.const 1)
      (i32.const 2))))
";
        let line_index = LineIndex::new(text);
        assert_eq!(
            line_index.convert(TextSize::new(93)).unwrap(),
            Position { line: 5, character: 1 },
        );
        assert_eq!(
            line_index.convert(TextSize::new(94)).unwrap(),
            Position { line: 5, character: 2 },
        );
        assert!(line_index.convert(TextSize::new(129)).is_none());
        assert_eq!(
            line_index.convert(TextSize::new(130)).unwrap(),
            Position { line: 6, character: 15 },
        );
        assert_eq!(
            line_index.convert(TextSize::new(134)).unwrap(),
            Position { line: 6, character: 17 },
        );
        assert_eq!(
            line_index.convert(TextSize::new(136)).unwrap(),
            Position { line: 6, character: 19 },
        );
        assert_eq!(
            line_index.convert(TextSize::new(137)).unwrap(),
            Position { line: 7, character: 0 },
        );

        assert!(
            line_index
                .convert(Position {
                    line: 5,
                    character: u32::MAX
                })
                .is_none(),
        );
    }

    #[test]
    fn position_to_offset() {
        let text = "
(module
  (type $t (func))
  (table $t1 10 (ref null func))
  (table $t2 10 (ref null $t))
  (elem $el funcref)
  (func $f (;😈🍔;)
    (table.init $t1 $el
      (i32.const 0)
      (i32.const 1)
      (i32.const 2))
    (table.copy $t1 $t2
      (i32.const 0)
      (i32.const 1)
      (i32.const 2))))
";
        let line_index = LineIndex::new(text);
        assert_eq!(
            line_index.convert(Position { line: 5, character: 1 }).unwrap(),
            TextSize::new(93),
        );
        assert_eq!(
            line_index.convert(Position { line: 5, character: 2 }).unwrap(),
            TextSize::new(94),
        );
        assert_eq!(
            line_index.convert(Position { line: 6, character: 11 }).unwrap(),
            TextSize::new(124),
        );
        assert_eq!(
            line_index.convert(Position { line: 6, character: 13 }).unwrap(),
            TextSize::new(126),
        );
        assert!(line_index.convert(Position { line: 6, character: 14 }).is_none());
        assert_eq!(
            line_index.convert(Position { line: 6, character: 15 }).unwrap(),
            TextSize::new(130),
        );
        assert!(line_index.convert(Position { line: 6, character: 16 }).is_none());
        assert_eq!(
            line_index.convert(Position { line: 6, character: 17 }).unwrap(),
            TextSize::new(134),
        );
        assert_eq!(
            line_index.convert(Position { line: 6, character: 19 }).unwrap(),
            TextSize::new(136),
        );
        assert!(line_index.convert(Position { line: 6, character: 20 }).is_none());
        assert_eq!(
            line_index.convert(Position { line: 7, character: 0 }).unwrap(),
            TextSize::new(137),
        );
        assert_eq!(
            line_index.convert(Position { line: 15, character: 0 }).unwrap(),
            TextSize::new(309),
        );
    }

    #[test]
    fn range() {
        let text = "
(module
  (type $t (func))
  (table $t1 10 (ref null func))
  (table $t2 10 (ref null $t))
  (elem $el funcref)
  (func $f (;😈🍔;)
    (table.init $t1 $el
      (i32.const 0)
      (i32.const 1)
      (i32.const 2))
    (table.copy $t1 $t2
      (i32.const 0)
      (i32.const 1)
      (i32.const 2))))
";
        let line_index = LineIndex::new(text);
        assert!(
            line_index
                .convert(Range {
                    start: Position { line: 5, character: 2 },
                    end: Position { line: 5, character: 1 }
                })
                .is_none(),
        );
    }

    #[test]
    fn crlf() {
        let line_index = LineIndex::new("(;;)\r\n(module)");
        assert_eq!(
            line_index.convert(Position { line: 0, character: 4 }).unwrap(),
            TextSize::new(4),
        );
        assert_eq!(
            line_index.convert(Position { line: 0, character: 5 }).unwrap(),
            TextSize::new(5),
        );

        let line_index = LineIndex::new("(;😈🍔;)\r\n(module)");
        assert_eq!(
            line_index.convert(Position { line: 0, character: 6 }).unwrap(),
            TextSize::new(10),
        );
        assert_eq!(
            line_index.convert(Position { line: 0, character: 7 }).unwrap(),
            TextSize::new(11),
        );
    }
}
