use super::is_whitespace;
use std::arch::aarch64::*;

static BIT_VALUES: [u8; 16] = [1, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8, 16, 32, 64, 128];

#[target_feature(enable = "neon")]
pub unsafe fn scan_whitespace_neon(text: &str) -> usize {
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

    let bit_values = unsafe { vld1q_u8(BIT_VALUES.as_ptr()) };
    let space = vdupq_n_u8(b' ');
    let cr = vdupq_n_u8(b'\r');
    let lf = vdupq_n_u8(b'\n');
    let tab = vdupq_n_u8(b'\t');

    while i + 16 <= len {
        let chunk = unsafe { vld1q_u8(bytes.as_ptr().add(i)) };
        let whitespace = vorrq_u8(
            vorrq_u8(vceqq_u8(chunk, space), vceqq_u8(chunk, tab)),
            vorrq_u8(vceqq_u8(chunk, lf), vceqq_u8(chunk, cr)),
        );

        // simulate "movemask" in x86:
        // for example, a chunk that is checked in previous step can be:
        // [FF, FF, FF, FF, FF, 00, FF, 00, ...the high 8 bytes]
        // after the "AND" operation:
        // [ 1,  2,  4,  8, 16,  0, 64,  0, ...]
        // the "addv" adds these 8 bytes together:
        // 1 + 2 + 4 + 8 + 16 + 0 + 64 + 0 = 95
        // (same for the high 8 bytes)
        // 95.trailing_ones() = 5
        let whitespace_bits = vandq_u8(whitespace, bit_values);
        let lo = vaddv_u8(vget_low_u8(whitespace_bits));
        let hi = vaddv_u8(vget_high_u8(whitespace_bits));
        let mask = (hi as u16) << 8 | lo as u16;

        // "MAX" means all bytes of the chunk are whitespaces
        if mask != u16::MAX {
            return i + mask.trailing_ones() as usize;
        }

        i += 16;
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
