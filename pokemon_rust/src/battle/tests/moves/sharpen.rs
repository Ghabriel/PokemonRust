use crate::battle::backend::{BattleEvent, StatChangeKind};

use super::super::{prelude::*, TestMethods};

#[test]
fn sharpen_sharply_raises_attack() {
    let mut backend = battle! {
        "Porygon" 4 (max ivs, Serious) vs "Metapod" 4 (max ivs, Serious)
    };

    let events = backend.process_turn("Sharpen", "Harden");

    assert_event!(events[1], StatChange { target: 0, kind: StatChangeKind::Rose, stat: Stat::Attack });
}
