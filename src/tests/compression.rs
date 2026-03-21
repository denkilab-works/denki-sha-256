use crate::*;
use proptest::prelude::*;

fn random_hash() -> impl Strategy<Value = [u32; 8]> {
    proptest::array::uniform8(any::<u32>())
}

fn random_schedule() -> impl Strategy<Value = [u32; 64]> {
    proptest::array::uniform(any::<u32>())
}

proptest! {
    #[test]
    fn output_has_eight_words(hash in random_hash(), w in random_schedule()) {
        let result = compress(&hash, &w);
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn same_inputs_produce_same_output(hash in random_hash(), w in random_schedule()) {
        assert_eq!(compress(&hash, &w), compress(&hash, &w));
    }

    #[test]
    fn different_schedules_produce_different_hashes(
        hash in random_hash(),
        w1 in random_schedule(),
        w2 in random_schedule(),
    ) {
        prop_assume!(w1 != w2);
        assert_ne!(compress(&hash, &w1), compress(&hash, &w2));
    }

    #[test]
    fn output_differs_from_input_hash(hash in random_hash(), w in random_schedule()) {
        let result = compress(&hash, &w);
        assert_ne!(result, hash);
    }
}
