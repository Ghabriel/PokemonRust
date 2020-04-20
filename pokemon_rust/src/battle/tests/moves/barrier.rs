use crate::battle::backend::{BattleEvent, StatChangeKind};

use super::super::{prelude::*, TestMethods};

#[test]
fn barrier_sharply_raises_defense() {
    let mut backend = battle! {
        "Tentacool" 31 (max ivs, Serious) vs "Clefairy" 31 (max ivs, Serious)
    };

    let events = backend.process_turn("Barrier", "Pound");

    assert_event!(events[1], StatChange { target: 0, kind: StatChangeKind::SharplyRose, stat: Stat::Defense });
}
