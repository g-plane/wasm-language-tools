use super::NonAsciiChar;
use std::arch::x86_64::*;

const CHUNK_SIZE: usize = 32;

#[target_feature(enable = "avx2")]
pub unsafe fn scan_avx2(text: &str) -> (Vec<u32>, Vec<NonAsciiChar>) {
    let newline = _mm256_set1_epi8(b'\n' as i8);

    let mut indices = vec![0];
    let mut non_ascii_chars = vec![];

    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i + CHUNK_SIZE <= len {
        let chunk = unsafe { _mm256_loadu_si256(bytes.as_ptr().add(i) as *const __m256i) };
        let mut mask = _mm256_movemask_epi8(_mm256_cmpeq_epi8(chunk, newline)) as u32;
        while mask != 0 {
            indices.push(i as u32 + mask.trailing_zeros() as u32 + 1);
            mask &= mask - 1;
        }

        // "movemask" extracts the most significant bit (MSB) of each byte,
        // and the MSB of non-ASCII byte is always 1.
        if _mm256_movemask_epi8(chunk) != 0
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
