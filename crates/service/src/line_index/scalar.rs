use super::NonAsciiChar;

pub fn scan_scalar(text: &str) -> (Vec<u32>, Vec<NonAsciiChar>) {
    let mut indices = vec![0];
    let mut non_ascii_chars = vec![];

    text.char_indices().for_each(|(index, char)| {
        if char == '\n' {
            indices.push(index as u32 + 1);
        } else if !char.is_ascii() {
            non_ascii_chars.push(NonAsciiChar {
                offset: index as u32,
                utf8_width: char.len_utf8() as u8,
                utf16_width: char.len_utf16() as u8,
            });
        }
    });

    (indices, non_ascii_chars)
}
