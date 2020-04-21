use crate::battle::backend::{BattleEvent, StatChangeKind};

use super::super::{prelude::*, TestMethods};

#[test]
fn amnesia_sharply_raises_special_defense() {
    let mut backend = battle! {
        "Slowpoke" 27 (max ivs, Serious) vs "Clefairy" 27 (max ivs, Serious)
    };
    backend.get_pokemon_mut(1).stats[5] = 0;

    let events = backend.process_turn("Amnesia", "Pound");

    assert_event!(events[1], StatChange { target: 0, kind: StatChangeKind::SharplyRose, stat: Stat::SpecialDefense });
}
