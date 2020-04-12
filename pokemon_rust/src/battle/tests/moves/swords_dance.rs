use crate::battle::backend::{BattleEvent, StatChangeKind};

use super::super::{prelude::*, TestMethods};

#[test]
fn swords_dance_raises_defense() {
    let mut backend = battle! {
        "Krabby" 40 (max ivs, Serious) vs "Clefairy" 5 (max ivs, Serious)
    };

    let events = backend.process_turn("SwordsDance", "Pound");

    assert_event!(events[1], StatChange { target: 0, kind: StatChangeKind::SharplyRose, stat: Stat::Attack });
}
