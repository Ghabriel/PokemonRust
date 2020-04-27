use crate::battle::backend::{BattleEvent, StatChangeKind};

use super::super::{prelude::*, TestMethods};

#[test]
fn acid_armor_sharply_raises_defense() {
    let mut backend = battle! {
        "Vaporeon" 45 (max ivs, Serious) vs "Metapod" 45 (max ivs, Serious)
    };

    let events = backend.process_turn("AcidArmor", "Harden");

    assert_event!(events[1], StatChange { target: 0, kind: StatChangeKind::SharplyRose, stat: Stat::Defense });
}
