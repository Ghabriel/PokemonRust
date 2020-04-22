use crate::battle::backend::{BattleEvent, StatChangeKind};

use super::super::{prelude::*, TestMethods};

#[test]
fn kinesis_reduces_accuracy() {
    let mut backend = battle! {
        "Kadabra" 5 (max ivs, Serious) vs "Metapod" 5 (max ivs, Serious)
    };

    let events = backend.process_turn("Kinesis", "Harden");

    assert_event!(events[1], StatChange { target: 1, kind: StatChangeKind::Fell, stat: Stat::Accuracy });
}
