use crate::*;

#[test]
fn length_should_be_64bytes_multiple() {
    let padded = padding(b"abc");
    assert_eq!(padded.len() % 64, 0);
}

#[test]
fn last_8bytes_should_contain_msg_length() {
    let bin = b"abc";
    let padded = padding(bin);
    let last_8 = &padded[padded.len() - 8..];
    let stored_len = u64::from_be_bytes(last_8.try_into().unwrap());
    assert_eq!(stored_len, (bin.len() as u64) * 8);
}

#[test]
fn padding_bytes_should_be_zero() {
    let bin = b"abc";
    let padded = padding(bin);
    let padding_zone = &padded[bin.len() + 1..padded.len() - 8];
    assert!(padding_zone.iter().all(|&b| b == 0x00));
    assert_eq!(padded[bin.len()], 0b10000000);
}
