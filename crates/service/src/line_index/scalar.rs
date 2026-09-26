pub fn scan_scalar(text: &str) -> (Vec<u32>, bool) {
    let mut indices = vec![0];
    let mut fully_ascii = true;

    let bytes = text.as_bytes();
    let mut i = 0;
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
