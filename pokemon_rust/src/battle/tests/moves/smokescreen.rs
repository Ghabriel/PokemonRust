use crate::battle::backend::{BattleEvent, StatChangeKind};

use super::super::{prelude::*, TestMethods};

#[test]
fn smokescreen_reduces_accuracy() {
    let mut backend = battle! {
        "Charmander" 8 (max ivs, Serious) vs "Metapod" 8 (max ivs, Serious)
    };

    let events = backend.process_turn("Smokescreen", "Harden");

    assert_event!(events[1], StatChange { target: 1, kind: StatChangeKind::Fell, stat: Stat::Accuracy });
}
