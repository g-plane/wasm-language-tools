use super::is_whitespace;
use std::arch::x86_64::*;

#[target_feature(enable = "avx2")]
pub unsafe fn scan_whitespace_avx2(text: &str) -> usize {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    // for short whitespace there's no need to execute SIMD
    while i < 16
        && let Some(byte) = bytes.get(i)
    {
        if !is_whitespace(*byte) {
            return i;
        }
        i += 1;
    }

    let space = _mm256_set1_epi8(b' ' as i8);
    let cr = _mm256_set1_epi8(b'\r' as i8);
    let lf = _mm256_set1_epi8(b'\n' as i8);
    let tab = _mm256_set1_epi8(b'\t' as i8);

    while i + 32 <= len {
        let chunk = unsafe { _mm256_loadu_si256(bytes.as_ptr().add(i) as *const __m256i) };
        let whitespace = _mm256_or_si256(
            _mm256_or_si256(_mm256_cmpeq_epi8(chunk, space), _mm256_cmpeq_epi8(chunk, tab)),
            _mm256_or_si256(_mm256_cmpeq_epi8(chunk, lf), _mm256_cmpeq_epi8(chunk, cr)),
        );

        let mask = _mm256_movemask_epi8(whitespace) as u32;
        // "MAX" means all bytes of the chunk are whitespaces
        if mask != u32::MAX {
            return i + mask.trailing_ones() as usize;
        }

        i += 32;
    }

    // handle trailing bytes
    while let Some(byte) = bytes.get(i) {
        if !is_whitespace(*byte) {
            return i;
        }
        i += 1;
    }

    i
}
