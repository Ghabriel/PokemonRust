use crate::battle::backend::{BattleEvent, StatChangeKind};

use super::super::{prelude::*, TestMethods};

#[test]
fn withdraw_raises_defense() {
    let mut backend = battle! {
        "Squirtle" 6 (max ivs, Serious) vs "Clefairy" 6 (max ivs, Serious)
    };

    let events = backend.process_turn("Withdraw", "Pound");

    assert_event!(events[1], StatChange { target: 0, kind: StatChangeKind::Rose, stat: Stat::Defense });
}
