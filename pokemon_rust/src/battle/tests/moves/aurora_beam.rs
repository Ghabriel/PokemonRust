use crate::battle::backend::{BattleEvent, StatChangeKind};

use super::super::{prelude::*, TestMethods};

#[test]
fn aurora_beam_deals_damage_and_might_reduce_speed() {
    let mut backend = battle! {
        "Vaporeon" 30 (max ivs, Serious) vs "Metapod" 40 (max ivs, Serious)
    };

    test_rng_mut!(backend.rng).force_secondary_effect(1);
    let turn1 = backend.process_turn("AuroraBeam", "Harden");
    assert_event!(turn1[1], Damage { target: 1, is_critical_hit: false, .. });
    assert_event!(turn1[2], StatChange { target: 1, kind: StatChangeKind::Fell, stat: Stat::Attack });

    let turn2 = backend.process_turn("AuroraBeam", "Harden");
    assert_event!(turn2[1], Damage { target: 1, is_critical_hit: false, .. });
    assert_event!(turn2[2], UseMove { move_user: 1, .. });
}
