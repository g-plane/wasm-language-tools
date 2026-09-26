use std::arch::aarch64::*;

static BIT_VALUES: [u8; 16] = [1, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8, 16, 32, 64, 128];

#[target_feature(enable = "neon")]
pub unsafe fn scan_neon(text: &str) -> (Vec<u32>, bool) {
    let bit_values = unsafe { vld1q_u8(BIT_VALUES.as_ptr()) };
    let newline = vdupq_n_u8(b'\n');

    let mut indices = vec![0];
    let mut fully_ascii = true;

    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i + 16 <= len {
        let chunk = unsafe { vld1q_u8(bytes.as_ptr().add(i)) };
        let test_newline = vceqq_u8(chunk, newline);
        let test_ascii = vshrq_n_u8::<7>(chunk);

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

        fully_ascii &= vmaxvq_u8(test_ascii) == 0;

        i += 16;
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
