use crate::battle::backend::{BattleEvent, StatChangeKind};

use super::super::{prelude::*, TestMethods};

#[test]
fn leer_reduces_defense() {
    let mut backend = battle! {
        "Spearow" 8 (max ivs, Serious) vs "Metapod" 8 (max ivs, Serious)
    };

    let events = backend.process_turn("Leer", "Harden");

    assert_event!(events[1], StatChange { target: 1, kind: StatChangeKind::Fell, stat: Stat::Defense });
}
