use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn double_slap_deals_damage_two_to_five_times() {
    macro_rules! test {
        (rng = $value:literal yields $expected_number_of_hits:literal hits) => {
            let mut backend = battle! {
                "Clefairy" 10 (max ivs, Serious) vs "Metapod" 10 (max ivs, Serious)
            };

            test_rng_mut(&mut backend.rng).force_custom_multi_hit_value($value);
            let events = backend.process_turn("DoubleSlap", "Harden");

            for i in 0..$expected_number_of_hits {
                assert_event!(events[1 + i], Damage { target: 1, is_critical_hit: false, .. });
            }
        }
    }

    test!(rng = 1 yields 2 hits);
    test!(rng = 2 yields 2 hits);
    test!(rng = 3 yields 3 hits);
    test!(rng = 4 yields 3 hits);
    test!(rng = 5 yields 4 hits);
    test!(rng = 6 yields 5 hits);
}
