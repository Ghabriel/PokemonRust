use crate::battle::backend::{BattleEvent, StatChangeKind};

use super::super::{prelude::*, TestMethods};

#[test]
fn screech_reduces_defense() {
    let mut backend = battle! {
        "Onix" 24 (max ivs, Serious) vs "Metapod" 24 (max ivs, Serious)
    };

    let events = backend.process_turn("Screech", "Harden");

    assert_event!(events[1], StatChange { target: 1, kind: StatChangeKind::HarshlyFell, stat: Stat::Defense });
}
