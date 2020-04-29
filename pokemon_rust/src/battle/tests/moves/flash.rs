use crate::battle::backend::{BattleEvent, StatChangeKind};

use super::super::{prelude::*, TestMethods};

#[test]
fn flash_reduces_accuracy() {
    let mut backend = battle! {
        "Kadabra" 13 (max ivs, Serious) vs "Metapod" 13 (max ivs, Serious)
    };

    let events = backend.process_turn("Flash", "Harden");

    assert_event!(events[1], StatChange { target: 1, kind: StatChangeKind::Fell, stat: Stat::Accuracy });
}
