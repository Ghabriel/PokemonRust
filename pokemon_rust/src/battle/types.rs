use amethyst::ecs::{Component, DenseVecStorage};

use crate::entities::{
    character::CharacterId,
    pokemon::Pokemon,
};

use std::collections::VecDeque;

/// Represents a Pokémon Battle.
#[derive(Clone)]
pub struct Battle {
    /// The type of battle that is happening.
    pub battle_type: BattleType,
    /// The current turn.
    pub turn: usize,
    /// The Pokémon that make up the first team. If the local player is
    /// participating, this is always his team.
    pub p1: BattleCharacterTeam,
    /// The Pokémon that make up the second team.
    pub p2: BattleCharacterTeam,
}

impl Battle {
    pub fn new(
        battle_type: BattleType,
        p1: BattleCharacterTeam,
        p2: BattleCharacterTeam,
    ) -> Battle {
        Battle {
            battle_type,
            turn: 0,
            p1,
            p2,
        }
    }
}

/// Represents the type of battle that is happening.
#[derive(Clone)]
pub enum BattleType {
    Single,
}

/// Represents which Pokémon make up a team.
#[derive(Clone)]
pub struct BattleCharacterTeam {
    // TODO: change this to a Vec to implement doubles/triples
    /// The active Pokémon of this team.
    pub active_pokemon: Option<Pokemon>,
    /// The Pokémon party of this team.
    pub party: Party,
    /// If this team is owned by a trainer, contains its character ID.
    pub character_id: Option<CharacterId>,
}

#[derive(Clone)]
pub struct Party {
    pub pokemon: VecDeque<Pokemon>,
}

impl Component for Party {
    type Storage = DenseVecStorage<Self>;
}
