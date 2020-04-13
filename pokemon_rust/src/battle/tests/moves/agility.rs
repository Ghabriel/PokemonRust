use crate::battle::backend::{BattleEvent, StatChangeKind};

use super::super::{prelude::*, TestMethods};

#[test]
fn agility_sharply_raises_attack() {
    let mut backend = battle! {
        "Hitmonchan" 28 (max ivs, Serious) vs "Metapod" 28 (max ivs, Serious)
    };

    let events = backend.process_turn("Agility", "Harden");

    assert_event!(events[1], StatChange { target: 0, kind: StatChangeKind::SharplyRose, stat: Stat::Speed });
}
