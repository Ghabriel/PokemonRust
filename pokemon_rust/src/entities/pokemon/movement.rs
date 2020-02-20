use super::{Pokemon, PokemonType, Stat, StatusCondition};

pub struct Move {
    id: String,
    display_name: String,
    description: String,
    move_type: PokemonType,
    category: MoveCategory,
    base_power: MovePower,
    power_modifier: Option<MoveCallback<usize>>,
    /// The accuracy of this move. This is None for moves that never miss,
    /// e.g Swift.
    accuracy: Option<usize>,
    pp: usize,
    priority: i8,
    target_type: TargetType,
    multi_hit: Option<MultiHit>,
    /// A "shortcut" for moves with a simple secondary effect
    secondary_effect: SecondaryEffect,
}

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

pub type MoveCallback<T = ()> = fn(
    user: &Pokemon,
    target: &Pokemon,
    movement: &Move,
) -> T;

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

pub struct MultiHit {
    min_hits: usize,
    max_hits: usize,
}

pub struct SecondaryEffect {
    chance: usize,
    effect: SimpleEffect,
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
