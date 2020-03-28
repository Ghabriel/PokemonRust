use crate::battle::backend::{rng::BattleRng, BattleBackend};

use std::collections::{HashMap, HashSet};

use super::{Pokemon, PokemonType, Stat, StatusCondition};

pub struct MoveDex {
    data: HashMap<String, Move>,
}

impl MoveDex {
    pub fn new(data: HashMap<String, Move>) -> MoveDex {
        MoveDex { data }
    }

    pub fn get_move(&self, id: &str) -> Option<&Move> {
        self.data.get(id)
    }
}

pub struct Move {
    pub id: String,
    pub display_name: String,
    pub description: String,
    pub move_type: PokemonType,
    pub category: MoveCategory,
    pub base_power: MovePower,
    pub power_modifier: Option<MoveCallback<usize>>,
    /// The accuracy of this move. This is None for moves that never miss,
    /// e.g Swift.
    pub accuracy: Option<usize>,
    pub accuracy_modifier: Option<MoveCallback<ModifiedAccuracy>>,
    pub flags: HashSet<MoveFlag>,
    pub on_usage_attempt: Option<ExtendedMoveCallback<ModifiedUsageAttempt>>,
    pub pp: usize,
    pub priority: i8,
    pub target_type: TargetType,
    pub multi_hit: Option<MultiHit>,
    /// A "shortcut" for moves with a simple secondary effect
    pub secondary_effect: Option<SecondaryEffect>,
    /// In this game, there's no RNG in critical hits: a move either always
    /// crits or never crits. Moves that originally mention "high critical
    /// chance" in their description or possess some other kind of critical hit
    /// buff always crit, dealing 125% damage and ignoring both offensive stat
    /// debuffs and defensive stat buffs. Every other move never crits.
    pub critical_hit: bool,
}

#[derive(Clone, Copy)]
pub enum MoveCategory {
    Physical,
    Special,
    Status,
}

pub enum MovePower {
    /// Moves which have a fixed base power. This value is displayed in the
    /// information screen of the move.
    Constant(usize),
    /// Used for moves that have no meaningful value to display as their power,
    /// e.g Fissure.
    Special,
}

pub type MoveCallback<T = ()> = fn(user: &Pokemon, target: &Pokemon, movement: &Move) -> T;
pub type ExtendedMoveCallback<T = ()> = fn(backend: &BattleBackend, user: usize, target: usize, movement: &Move) -> T;

pub enum ModifiedAccuracy {
    Miss,
    Hit,
    NewValue(usize),
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum MoveFlag {
    OneHitKO,
}

pub enum ModifiedUsageAttempt {
    Fail,
    Continue,
}

pub enum TargetType {
    /// Affects everyone in the field, e.g Wonder Room
    Everyone,

    /// Affects all adjacent Pokémon, e.g Teeter Dance
    AllAdjacent,
    /// Affects all adjacent foes, but not allies, e.g Acid
    AllAdjacentFoes,
    /// Affects the user and all allies, e.g Aromatherapy
    AllyTeam,
    /// Affects all foes, e.g Toxic Spikes
    OpposingTeam,

    /// Can affect anyone but the user, e.g Air Slash
    SingleTarget,
    /// Affects a single adjacent target, e.g Flamethrower
    SingleAdjacentTarget,
    /// Affects a single adjacent ally, e.g Aromatic Mist
    SingleAdjacentAlly,
    /// Affects a single adjacent ally or the user, e.g Acupressure
    SingleAdjacentAllyOrUser,
    /// Affects a single adjacent foe, e.g Me First
    SingleAdjacentFoe,
    /// Affects the user, e.g Aqua Ring
    User,
}

pub enum MultiHit {
    Uniform {
        min_hits: usize,
        max_hits: usize,
    },
    Custom(fn(rng: Box<dyn BattleRng>) -> usize),
}

pub struct SecondaryEffect {
    pub chance: usize,
    pub effect: SimpleEffect,
}

pub enum SimpleEffect {
    Confusion,
    Flinch,
    StatChange {
        changes: Vec<(Stat, i8)>,
        target: SimpleEffectTarget,
    },
    StatusCondition(StatusCondition),
    OnHit(MoveCallback),
}

pub enum SimpleEffectTarget {
    MoveTarget,
    MoveUser,
}
