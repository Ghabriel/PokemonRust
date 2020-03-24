use crate::battle::backend::{BattleEvent, StatChangeKind};

use super::super::{prelude::*, TestMethods};

#[test]
fn growl_reduces_attack() {
    let mut backend = battle! {
        "Charmander" 1 (max ivs, Serious) vs "Metapod" 1 (max ivs, Serious)
    };

    let events = backend.process_turn("Growl", "Harden");

    assert_event!(events[1], StatChange { target: 1, kind: StatChangeKind::Fell, stat: Stat::Attack });
}
