#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[inline(always)]
fn is_whitespace(byte: u8) -> bool {
    matches!(byte, b' ' | b'\n' | b'\t' | b'\r')
}

pub fn scan_whitespace_scalar(text: &str) -> usize {
    text.bytes().position(|byte| !is_whitespace(byte)).unwrap_or(text.len())
}
