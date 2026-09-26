use lspt::{Position, Range};
use std::ops::ControlFlow;
use wat_syntax::{TextRange, TextSize};

#[cfg(target_arch = "aarch64")]
mod aarch64;
mod scalar;
#[cfg(target_arch = "x86_64")]
mod x86_64;

#[derive(Clone, Debug)]
pub struct LineIndex {
    lines: Vec<u32>,
    fully_ascii: bool,
    text: String,
}
impl LineIndex {
    pub fn new(text: String) -> Self {
        #[cfg(target_arch = "x86_64")]
        let (lines, fully_ascii) = if std::arch::is_x86_feature_detected!("avx2") {
            // SAFETY: AVX2 support is checked
            unsafe { self::x86_64::scan_avx2(&text) }
        } else {
            self::scalar::scan_scalar(&text)
        };
        #[cfg(target_arch = "aarch64")]
        let (lines, fully_ascii) = if std::arch::is_aarch64_feature_detected!("neon") {
            // SAFETY: NEON support is checked
            unsafe { self::aarch64::scan_neon(&text) }
        } else {
            self::scalar::scan_scalar(&text)
        };
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        let (lines, fully_ascii) = self::scalar::scan_scalar(&text);

        Self {
            lines,
            fully_ascii,
            text,
        }
    }

    #[inline]
    pub fn convert<T>(&self, value: T) -> T::Out
    where
        T: LocationConvert,
    {
        value.convert(self)
    }

    #[inline]
    pub fn text(&self) -> &str {
        &self.text
    }
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
        if line_index.fully_ascii {
            Some(Position {
                line,
                character: u32::from(*self) - line_start,
            })
        } else {
            line_index
                .text
                .get(*line_start as usize..usize::from(*self))
                .map(|content| Position {
                    line,
                    character: content.encode_utf16().count() as u32,
                })
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
        if line_index.fully_ascii {
            let offset = line_start + self.character;
            if let Some(next_line_start) = next_line_start {
                if (offset as usize) < next_line_start {
                    Some(TextSize::new(offset))
                } else {
                    None
                }
            } else if (offset as usize) <= line_index.text().len() {
                Some(TextSize::new(offset))
            } else {
                None
            }
        } else {
            let content = line_index
                .text
                .get(*line_start as usize..next_line_start.unwrap_or(line_index.text().len()))?;
            match content.char_indices().try_fold(0, |acc, (bytes, char)| {
                if acc == self.character {
                    ControlFlow::Break(bytes)
                } else {
                    ControlFlow::Continue(acc + char.len_utf16() as u32)
                }
            }) {
                ControlFlow::Break(byte_col) => Some(TextSize::new(*line_start + byte_col as u32)),
                ControlFlow::Continue(byte_col) => {
                    if next_line_start.is_none() && byte_col as usize == content.len() {
                        Some(TextSize::new(*line_start + byte_col))
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
        Some(TextRange::new(start, end))
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
        let line_index = LineIndex::new(text.into());
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
        let line_index = LineIndex::new(text.into());
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
        let line_index = LineIndex::new(text.into());
        assert_eq!(
            line_index.convert(Position { line: 5, character: 1 }).unwrap(),
            TextSize::new(93),
        );
        assert_eq!(
            line_index.convert(Position { line: 5, character: 2 }).unwrap(),
            TextSize::new(94),
        );
        assert_eq!(
            line_index.convert(Position { line: 6, character: 15 }).unwrap(),
            TextSize::new(130),
        );
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
}
