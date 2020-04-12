use crate::battle::backend::{BattleEvent, StatChangeKind};

use super::super::{prelude::*, TestMethods};

#[test]
fn meditate_sharply_raises_attack() {
    let mut backend = battle! {
        "Hitmonlee" 5 (max ivs, Serious) vs "Metapod" 5 (max ivs, Serious)
    };

    let events = backend.process_turn("Meditate", "Harden");

    assert_event!(events[1], StatChange { target: 0, kind: StatChangeKind::Rose, stat: Stat::Attack });
}
