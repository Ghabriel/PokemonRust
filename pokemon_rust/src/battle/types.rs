use amethyst::ecs::{Component, DenseVecStorage};

use crate::entities::{
    character::CharacterId,
    pokemon::Pokemon,
};

/// Represents a Pokémon Battle.
#[derive(Clone)]
pub struct Battle {
    /// The type of battle that is happening.
    pub battle_type: BattleType,
    /// The Pokémon that make up the first team. If the local player is
    /// participating, this is always his team.
    pub p1: BattleCharacterTeam,
    /// The Pokémon that make up the second team.
    pub p2: BattleCharacterTeam,
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
    pub pokemon: Vec<Pokemon>,
}

impl Component for Party {
    type Storage = DenseVecStorage<Self>;
}
