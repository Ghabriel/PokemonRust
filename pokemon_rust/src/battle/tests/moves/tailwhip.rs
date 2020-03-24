use crate::battle::backend::{BattleEvent, StatChangeKind};

use super::super::{prelude::*, TestMethods};

#[test]
fn tailwhip_reduces_defense() {
    let mut backend = battle! {
        "Rattata" 3 (max ivs, Serious) vs "Pidgey" 3 (max ivs, Serious)
    };

    let events = backend.process_turn("TailWhip", "Tackle");

    assert_event!(events[1], StatChange { target: 1, kind: StatChangeKind::Fell, stat: Stat::Defense });
}
