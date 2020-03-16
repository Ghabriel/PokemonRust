use crate::battle::backend::{BattleEvent, StatChangeKind};

use super::{prelude::*, TestMethods, TestRng};

#[test]
fn harden_raises_defense() {
    let mut backend = battle! {
        "Metapod" 30 (max ivs, Serious) vs "Clefairy" 5 (max ivs, Serious)
    };

    let events = backend.process_turn("Harden", "Pound");

    assert_event!(events[1], StatChange { target: 0, kind: StatChangeKind::Rose, stat: Stat::Defense });
}
