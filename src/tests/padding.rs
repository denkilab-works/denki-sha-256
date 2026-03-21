use crate::*;
use proptest::prelude::*;

fn random_message() -> impl Strategy<Value = Vec<u8>> {
    proptest::collection::vec(any::<u8>(), 0..1024)
}

proptest! {
    #[test]
    fn length_should_be_64bytes_multiple(msg in random_message()) {
        let padded = padding(&msg);
        assert_eq!(padded.len() % 64, 0);
    }

    #[test]
    fn last_8bytes_should_contain_msg_length(msg in random_message()) {
        let padded = padding(&msg);
        let last_8 = &padded[padded.len() - 8..];
        let stored_len = u64::from_be_bytes(last_8.try_into().unwrap());
        assert_eq!(stored_len, (msg.len() as u64) * 8);
    }

    #[test]
    fn padding_bytes_should_be_zero(msg in random_message()) {
        let padded = padding(&msg);
        let padding_zone = &padded[msg.len() + 1..padded.len() - 8];
        assert!(padding_zone.iter().all(|&b| b == 0x00));
        assert_eq!(padded[msg.len()], 0x80);
    }
}
