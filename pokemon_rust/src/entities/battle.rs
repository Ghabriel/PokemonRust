use crate::entities::{
    character::CharacterId,
    pokemon::Pokemon,
};

/// Represents the type of battle that is happening.
enum BattleType {
    Single,
}

/// Represents the kind of opponent(s) that the player is facing.
enum OpponentKind {
    Trainer {
        character_id: CharacterId,
    },
    WildPokemon {
        pokemon: Pokemon,
    },
}
