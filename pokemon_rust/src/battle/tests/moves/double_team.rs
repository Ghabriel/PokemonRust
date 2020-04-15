use crate::battle::backend::{BattleEvent, StatChangeKind};

use super::super::{prelude::*, TestMethods};

#[test]
fn double_team_raises_evasion() {
    let mut backend = battle! {
        "Pikachu" 8 (max ivs, Serious) vs "Metapod" 8 (max ivs, Serious)
    };

    let events = backend.process_turn("DoubleTeam", "Harden");

    assert_event!(events[1], StatChange { target: 0, kind: StatChangeKind::Rose, stat: Stat::Evasion });
}
