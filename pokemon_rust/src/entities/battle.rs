use amethyst::ecs::{Component, DenseVecStorage};

use crate::entities::{
    character::CharacterId,
    pokemon::Pokemon,
};

/// Represents a Pokémon Battle.
pub struct Battle {
    /// The type of battle that is happening.
    pub battle_type: BattleType,
    /// The characters or wild Pokémon that make up the first team.
    /// If the local player is participating, this is always his team.
    pub p1: BattleCharacterTeam,
    /// The characters or wild Pokémon that make up the second team.
    pub p2: BattleCharacterTeam,
}

impl Component for Battle {
    type Storage = DenseVecStorage<Self>;
}

/// Represents the type of battle that is happening.
#[derive(Clone)]
pub enum BattleType {
    Single,
}

/// Represents which characters or wild Pokémon make up a team.
#[derive(Clone)]
pub enum BattleCharacterTeam {
    Trainer {
        character_id: CharacterId,
    },
    WildPokemon {
        pokemon: Pokemon,
    },
}
