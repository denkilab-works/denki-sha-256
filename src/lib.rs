/// Pads a message according to SHA-256 specification.
///
/// ```
/// let padded = denki_sha_256::padding(b"abc");
/// assert_eq!(padded.len() % 64, 0);
/// ```
pub fn padding(message: &[u8]) -> Vec<u8> {
    let mut padded = message.to_vec();

    // Delimiter bit: marks where the original message ends and the padding begins
    padded.push(0b10000000);

    // Pad with zero bytes until length == 448 mod 512, leaving room for the 8-byte (64-bit) length suffix
    while padded.len() % 64 != 56 {
        padded.push(0x00); // Hexadecimal of 0b00000000
    }

    // Append original message length in bits as a 64-bit big-endian value
    let bit_len = (message.len() as u64) * 8;
    padded.extend_from_slice(&bit_len.to_be_bytes());

    padded
}

#[cfg(test)]
mod tests;
