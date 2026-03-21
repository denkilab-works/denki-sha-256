use crate::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn choose_selects_f_when_e_is_all_ones(f in any::<u32>(), g in any::<u32>()) {
        assert_eq!(choose(0xFFFFFFFF, f, g), f);
    }

    #[test]
    fn choose_selects_g_when_e_is_all_zeros(f in any::<u32>(), g in any::<u32>()) {
        assert_eq!(choose(0x00000000, f, g), g);
    }

    #[test]
    fn majority_unanimous_ones(a in any::<u32>()) {
        assert_eq!(majority(a, a, a), a);
    }

    #[test]
    fn majority_two_equal_wins(a in any::<u32>(), b in any::<u32>()) {
        assert_eq!(majority(a, a, b), (a & a) ^ (a & b) ^ (a & b));
    }

    #[test]
    fn rotate_mix_a_zero_gives_zero(_dummy in 0u8..1) {
        assert_eq!(rotate_mix_a(0), 0);
    }

    #[test]
    fn rotate_mix_e_zero_gives_zero(_dummy in 0u8..1) {
        assert_eq!(rotate_mix_e(0), 0);
    }

    #[test]
    fn schedule_mix_0_zero_gives_zero(_dummy in 0u8..1) {
        assert_eq!(schedule_mix_0(0), 0);
    }

    #[test]
    fn schedule_mix_1_zero_gives_zero(_dummy in 0u8..1) {
        assert_eq!(schedule_mix_1(0), 0);
    }

    #[test]
    fn rotate_mix_a_differs_from_input(a in 1u32..=u32::MAX) {
        assert_ne!(rotate_mix_a(a), a);
    }

    #[test]
    fn rotate_mix_e_differs_from_input(e in 1u32..=u32::MAX) {
        assert_ne!(rotate_mix_e(e), e);
    }

    #[test]
    fn schedule_mix_0_differs_from_input(w in 1u32..=u32::MAX) {
        assert_ne!(schedule_mix_0(w), w);
    }

    #[test]
    fn schedule_mix_1_differs_from_input(w in 1u32..=u32::MAX) {
        assert_ne!(schedule_mix_1(w), w);
    }
}
