use std::arch::x86_64::*;

#[target_feature(enable = "avx2")]
pub unsafe fn scan_avx2(text: &str) -> (Vec<u32>, bool) {
    let newline = _mm256_set1_epi8(b'\n' as i8);

    let mut indices = vec![0];
    let mut fully_ascii = true;

    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i + 32 <= len {
        let chunk = unsafe { _mm256_loadu_si256(bytes.as_ptr().add(i) as *const __m256i) };
        let mut mask = _mm256_movemask_epi8(_mm256_cmpeq_epi8(chunk, newline)) as u32;
        while mask != 0 {
            indices.push(i as u32 + mask.trailing_zeros() as u32 + 1);
            mask &= mask - 1;
        }

        // "movemask" extracts the most significant bit (MSB) of each byte,
        // and the MSB of non-ASCII byte is always 1, so "result == 0" means all bytes are ASCII.
        fully_ascii &= _mm256_movemask_epi8(chunk) == 0;

        i += 32;
    }

    while let Some(byte) = bytes.get(i) {
        if *byte == b'\n' {
            indices.push(i as u32 + 1);
        } else {
            fully_ascii &= byte.is_ascii();
        }
        i += 1;
    }

    (indices, fully_ascii)
}
