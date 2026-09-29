use super::NonAsciiChar;
use std::arch::aarch64::*;

static BIT_VALUES: [u8; 16] = [1, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8, 16, 32, 64, 128];

const CHUNK_SIZE: usize = 16;

#[target_feature(enable = "neon")]
pub unsafe fn scan_neon(text: &str) -> (Vec<u32>, Vec<NonAsciiChar>) {
    let bit_values = unsafe { vld1q_u8(BIT_VALUES.as_ptr()) };
    let newline = vdupq_n_u8(b'\n');

    let mut indices = vec![0];
    let mut non_ascii_chars = vec![];

    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i + CHUNK_SIZE <= len {
        let chunk = unsafe { vld1q_u8(bytes.as_ptr().add(i)) };
        let test_newline = vceqq_u8(chunk, newline);
        if vmaxvq_u8(test_newline) != 0 {
            let mask = vandq_u8(test_newline, bit_values);
            let lo = vaddv_u8(vget_low_u8(mask));
            let hi = vaddv_u8(vget_high_u8(mask));
            let mut mask = (hi as u16) << 8 | lo as u16;
            while mask != 0 {
                indices.push(i as u32 + mask.trailing_zeros() as u32 + 1);
                mask &= mask - 1;
            }
        }

        if !vmaxvq_u8(chunk).is_ascii()
            && let Some(text) = text.get(i..)
        {
            std::hint::cold_path();
            text.char_indices()
                .take_while(|(index, _)| *index < CHUNK_SIZE)
                .filter(|(_, char)| !char.is_ascii())
                .for_each(|(index, char)| {
                    let utf8_width = char.len_utf8();
                    non_ascii_chars.push(NonAsciiChar {
                        offset: (i + index) as u32,
                        utf8_width: utf8_width as u8,
                        utf16_width: char.len_utf16() as u8,
                    });
                    // for the case that the non-ASCII character is split across two chunks
                    if let Some(extra) = (index + utf8_width).checked_sub(CHUNK_SIZE) {
                        i += extra;
                    }
                });
        }

        i += CHUNK_SIZE;
    }

    if let Some(text) = text.get(i..) {
        text.char_indices().for_each(|(index, char)| {
            if char == '\n' {
                indices.push((i + index + 1) as u32);
            } else if !char.is_ascii() {
                non_ascii_chars.push(NonAsciiChar {
                    offset: (i + index) as u32,
                    utf8_width: char.len_utf8() as u8,
                    utf16_width: char.len_utf16() as u8,
                });
            }
        });
    }

    (indices, non_ascii_chars)
}
