use crate::*;
use proptest::prelude::*;

fn random_block() -> impl Strategy<Value = Vec<u8>> {
    proptest::collection::vec(any::<u8>(), 64..=64)
}

proptest! {
    #[test]
    fn output_has_64_words(block in random_block()) {
        let w = message_schedule(&block);
        assert_eq!(w.len(), 64);
    }

    #[test]
    fn first_16_words_match_block_bytes(block in random_block()) {
        let w = message_schedule(&block);
        for t in 0..16 {
            let expected = u32::from_be_bytes(block[t * 4..(t + 1) * 4].try_into().unwrap());
            assert_eq!(w[t], expected, "w[{t}] should match block bytes");
        }
    }

    #[test]
    fn expanded_words_follow_recurrence(block in random_block()) {
        let w = message_schedule(&block);
        for t in 16..64 {
            let expected = schedule_mix_1(w[t - 2])
                .wrapping_add(w[t - 7])
                .wrapping_add(schedule_mix_0(w[t - 15]))
                .wrapping_add(w[t - 16]);
            assert_eq!(w[t], expected, "w[{t}] should follow recurrence");
        }
    }

    #[test]
    fn same_block_produces_same_schedule(block in random_block()) {
        assert_eq!(message_schedule(&block), message_schedule(&block));
    }
}
