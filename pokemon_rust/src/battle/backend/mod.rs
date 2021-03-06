pub mod rng;

use crate::{
    overworld::entities::character::CharacterId,
    pokemon::{
        get_all_moves,
        get_all_pokemon_species,
        get_status_condition_effect,
        movement::{
            ModifiedAccuracy,
            ModifiedUsageAttempt,
            Move,
            MoveCategory,
            MoveFlag,
            MovePower,
            MultiHit,
            SimpleEffect,
            SimpleEffectTarget,
        },
        Pokemon,
        PokemonSpeciesData,
        PokemonType,
        SimpleStatusCondition,
        Stat,
        StatusCondition,
        StatusConditionEffect,
    },
};

use std::collections::{HashMap, VecDeque};

use self::rng::BattleRng;

use super::types::{Battle, BattleType};

/// Represents an event that can be sent from the frontend to the backend.
#[derive(Debug)]
pub struct FrontendEvent {
    pub team: Team,
    pub event: FrontendEventKind,
}

/// The kind of events that the frontend can send to the backend.
#[derive(Debug)]
pub enum FrontendEventKind {
    UseMove(usize),
}

/// The kind of events that the backend can send to the frontend.
#[derive(Debug, Eq, PartialEq)]
pub enum BattleEvent {
    InitialSwitchIn(event::InitialSwitchIn),
    ChangeTurn(event::ChangeTurn),
    UseMove(event::UseMove),
    Damage(event::Damage),
    Miss(event::Miss),
    StatChange(event::StatChange),
    VolatileStatusCondition(event::VolatileStatusCondition),
    ExpiredVolatileStatusCondition(event::ExpiredVolatileStatusCondition),
    NonVolatileStatusCondition(event::NonVolatileStatusCondition),
    ExpiredNonVolatileStatusCondition(event::ExpiredNonVolatileStatusCondition),
    FailedMove(event::FailedMove),
    Faint(event::Faint),
}

pub mod event {
    use super::{
        DamageCause,
        Flag,
        SimpleStatusCondition,
        Stat,
        StatChangeKind,
        StatusCondition,
        Team,
        TypeEffectiveness,
    };

    /// Corresponds to the very first switch-in of a battle participant in a
    /// battle.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct InitialSwitchIn {
        pub team: Team,
        pub pokemon: usize,
        /// Indicates if the Pokémon is already sent out when the battle
        /// started. The frontend uses this to decide whether a pokéball
        /// throwing animation should be played. This is `true` for wild
        /// Pokémon and `false` otherwise.
        pub is_already_sent_out: bool,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ChangeTurn {
        pub new_turn: usize,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct UseMove {
        pub move_user: usize,
        pub move_name: String,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Damage {
        pub target: usize,
        pub amount: usize,
        pub effectiveness: TypeEffectiveness,
        pub is_critical_hit: bool,
        pub multi_hit_index: Option<usize>,
        pub is_last_multi_hit_damage: bool,
        pub is_ohko: bool,
        pub cause: DamageCause,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Miss {
        pub target: usize,
        pub move_user: usize,
        pub caused_by_confusion: bool,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct StatChange {
        pub target: usize,
        pub kind: StatChangeKind,
        pub stat: Stat,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct VolatileStatusCondition {
        pub target: usize,
        pub added_flag: Flag,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ExpiredVolatileStatusCondition {
        pub target: usize,
        pub flag: Flag,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct NonVolatileStatusCondition {
        pub target: usize,
        pub condition: StatusCondition,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ExpiredNonVolatileStatusCondition {
        pub target: usize,
        pub condition: SimpleStatusCondition,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct FailedMove {
        pub move_user: usize,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Faint {
        pub target: usize,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DamageCause {
    Move,
    Burn,
    Poison,
    Toxic,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeEffectiveness {
    Immune,
    BarelyEffective,
    NotVeryEffective,
    Normal,
    SuperEffective,
    ExtremelyEffective,
}

impl TypeEffectiveness {
    fn from(effectiveness: f32) -> TypeEffectiveness {
        let scaled_effectiveness = (4. * effectiveness).round() as u8;

        match scaled_effectiveness {
            0 => Self::Immune,
            1 => Self::BarelyEffective,
            2 => Self::NotVeryEffective,
            4 => Self::Normal,
            8 => Self::SuperEffective,
            16 => Self::ExtremelyEffective,
            _ => panic!("Invalid type effectiveness: {}", effectiveness),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatChangeKind {
    WontGoAnyLower,
    SeverelyFell,
    HarshlyFell,
    Fell,
    Rose,
    SharplyRose,
    DrasticallyRose,
    WontGoAnyHigher,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Team {
    P1,
    P2,
}

pub struct UsedMove<'a> {
    user: usize,
    target: usize,
    movement: &'a Move,
}

#[derive(Debug)]
pub struct BattleBackend {
    /// The type of battle that is happening.
    battle_type: BattleType,
    /// The current turn.
    turn: usize,
    /// The Pokémon that make up the first team. If the local player is
    /// participating, this is always his team.
    pub(super) p1: TeamData,
    /// The Pokémon that make up the second team.
    pub(super) p2: TeamData,
    pokemon_flags: HashMap<usize, FlagContainer>,
    active_effects: HashMap<usize, Vec<StatusConditionEffect>>,
    input_events: VecDeque<FrontendEvent>,
    event_queue: Vec<BattleEvent>,
    pub(super) pokemon_repository: HashMap<usize, Pokemon>,
    /// The RNG that this battle is using.
    pub(super) rng: Box<dyn BattleRng + Sync + Send>,
}

#[derive(Debug)]
pub(super) struct TeamData {
    pub(super) active_pokemon: Option<usize>,
    party: VecDeque<usize>,
    character_id: Option<CharacterId>,
}

#[derive(Debug, Default)]
struct FlagContainer {
    flags: HashMap<&'static str, Flag>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Flag {
    Confusion { remaining_move_attempts: usize },
    Flinch,
    StatStages(HashMap<Stat, i8>),
}

pub struct MultiHitData {
    multi_hit_index: usize,
    maximum_number_of_hits: usize,
}

impl BattleBackend {
    pub fn new(data: Battle, rng: Box<dyn BattleRng + Sync + Send>) -> BattleBackend {
        let mut pokemon_repository = HashMap::new();
        let mut p1 = TeamData {
            active_pokemon: None,
            party: VecDeque::new(),
            character_id: data.p1.character_id,
        };
        let mut p2 = TeamData {
            active_pokemon: None,
            party: VecDeque::new(),
            character_id: data.p2.character_id,
        };
        let mut pokemon_flags = HashMap::new();

        for pokemon in data.p1.party.pokemon {
            let index = pokemon_repository.len();
            pokemon_repository.insert(index, pokemon);
            p1.party.push_back(index);
            pokemon_flags.insert(index, FlagContainer::default());
        }

        for pokemon in data.p2.party.pokemon {
            let index = pokemon_repository.len();
            pokemon_repository.insert(index, pokemon);
            p2.party.push_back(index);
            pokemon_flags.insert(index, FlagContainer::default());
        }

        BattleBackend {
            battle_type: data.battle_type,
            turn: 0,
            p1,
            p2,
            active_effects: HashMap::new(),
            input_events: VecDeque::new(),
            event_queue: Vec::new(),
            pokemon_repository,
            rng,
            pokemon_flags,
        }
    }

    pub fn push_frontend_event(&mut self, event: FrontendEvent) {
        self.input_events.push_back(event);
    }

    pub fn tick(&mut self) -> impl Iterator<Item = BattleEvent> + '_ {
        if self.turn == 0 {
            self.first_tick();
        } else {
            self.process_turn();
        }

        self.next_turn();
        self.event_queue.drain(..)
    }

    fn first_tick(&mut self) {
        self.p1.active_pokemon = self.p1.party.pop_front();
        assert!(self.p1.active_pokemon.is_some());

        self.p2.active_pokemon = self.p2.party.pop_front();
        assert!(self.p2.active_pokemon.is_some());

        self.event_queue
            .push(BattleEvent::InitialSwitchIn(event::InitialSwitchIn {
                team: Team::P2,
                pokemon: self.p2.active_pokemon.as_ref().unwrap().clone(),
                is_already_sent_out: self.p2.character_id.is_none(),
            }));

        self.event_queue
            .push(BattleEvent::InitialSwitchIn(event::InitialSwitchIn {
                team: Team::P1,
                pokemon: self.p1.active_pokemon.as_ref().unwrap().clone(),
                is_already_sent_out: false,
            }));

        // TODO: trigger things like Intimidate, entry hazards, Drought, etc.
        // The order is determined by speed.
    }

    fn process_turn(&mut self) {
        let (p1_action, p2_action) = self.decompose_input_events();

        match (p1_action, p2_action) {
            (FrontendEventKind::UseMove(p1_index), FrontendEventKind::UseMove(p2_index)) => {
                let movedex = get_all_moves();
                let p1 = self.p1.active_pokemon.unwrap();
                let p2 = self.p2.active_pokemon.unwrap();

                let p1_move = {
                    let p1_move_id = self.pokemon_repository[&p1].moves[p1_index]
                        .as_ref()
                        .unwrap();
                    let p1_move = movedex.get_move(&p1_move_id).unwrap();

                    UsedMove {
                        user: p1,
                        target: p2,
                        movement: &p1_move,
                    }
                };

                let p2_move = {
                    let p2_move_id = self.pokemon_repository[&p2].moves[p2_index]
                        .as_ref()
                        .unwrap();
                    let p2_move = movedex.get_move(&p2_move_id).unwrap();

                    UsedMove {
                        user: p2,
                        target: p1,
                        movement: &p2_move,
                    }
                };

                self.process_moves(vec![p1_move, p2_move].into_iter());
            },
        }
    }

    fn process_moves<'a>(&mut self, moves: impl Iterator<Item = UsedMove<'a>>) {
        let moves = self.sort_moves(moves);

        for used_move in moves {
            self.process_move(used_move);
        }
    }

    fn sort_moves<'a>(
        &mut self,
        moves: impl Iterator<Item = UsedMove<'a>>,
    ) -> impl Iterator<Item = UsedMove<'a>> {
        let mut result: Vec<_> = moves.collect();

        // Ensures random move order if both the priority and speed are equal
        self.rng.shuffle_moves(&mut result);

        result.sort_by(|a, b| {
            b.movement.priority.cmp(&a.movement.priority).then_with(|| {
                let a_speed = self.get_stat(a.user, Stat::Speed);
                let b_speed = self.get_stat(b.user, Stat::Speed);

                b_speed.cmp(&a_speed)
            })
        });

        result.into_iter()
    }

    fn process_move(&mut self, used_move: UsedMove) {
        if self.is_fainted(used_move.user) {
            return;
        }

        if self.has_flag(used_move.user, "flinch") {
            return;
        }

        if let Some(flag) = self.get_flag_mut(used_move.user, "confusion") {
            let remaining_move_attempts = match flag {
                Flag::Confusion { remaining_move_attempts } => remaining_move_attempts,
                _ => unreachable!(),
            };

            if *remaining_move_attempts > 0 {
                *remaining_move_attempts -= 1;
            } else {
                let flag = flag.clone();

                self.event_queue.push(BattleEvent::ExpiredVolatileStatusCondition(
                    event::ExpiredVolatileStatusCondition {
                        target: used_move.user,
                        flag,
                    }
                ));

                self.remove_flag(used_move.user, "confusion");
            }
        }

        let active_effects = self.active_effects
            .get(&used_move.user)
            .unwrap_or(&Vec::new())
            .clone();

        for effect in active_effects.iter().filter_map(|effect| effect.on_before_use_move) {
            if effect(self, used_move.user, &used_move.movement) == ModifiedUsageAttempt::Fail {
                self.event_queue.push(BattleEvent::FailedMove(event::FailedMove {
                    move_user: used_move.user,
                }));
                return;
            }
        }

        self.event_queue.push(BattleEvent::UseMove(event::UseMove {
            move_user: used_move.user,
            move_name: used_move.movement.display_name.clone(),
        }));

        for effect in active_effects.iter().filter_map(|effect| effect.on_try_use_move) {
            if effect(self, used_move.user, &used_move.movement) == ModifiedUsageAttempt::Fail {
                self.event_queue.push(BattleEvent::FailedMove(event::FailedMove {
                    move_user: used_move.user,
                }));
                return;
            }
        }

        if let Some(handler) = used_move.movement.on_usage_attempt {
            let result = handler(self, used_move.user, used_move.target, &used_move.movement);
            if result == ModifiedUsageAttempt::Fail {
                self.event_queue.push(BattleEvent::FailedMove(event::FailedMove {
                    move_user: used_move.user,
                }));
                return;
            }
        }

        if self.has_flag(used_move.user, "confusion") {
            if self.rng.check_confusion_miss() {
                self.event_queue.push(BattleEvent::Miss(event::Miss {
                    target: used_move.target,
                    move_user: used_move.user,
                    caused_by_confusion: true,
                }));
                return;
            }
        }

        if self.check_miss(&used_move) {
            self.event_queue.push(BattleEvent::Miss(event::Miss {
                target: used_move.target,
                move_user: used_move.user,
                caused_by_confusion: false,
            }));
            return;
        }

        match used_move.movement.category {
            MoveCategory::Physical | MoveCategory::Special => {
                if let Some(multi_hit) = &used_move.movement.multi_hit {
                    let number_of_hits = match multi_hit {
                        MultiHit::Uniform { min_hits, max_hits } => {
                            self.rng.check_uniform_multi_hit(*min_hits, *max_hits)
                        },
                        MultiHit::Custom(callback) => {
                            callback(self.rng.boxed_clone())
                        },
                    };

                    for i in 0..number_of_hits {
                        self.process_damage_effect(&used_move, Some(MultiHitData {
                            multi_hit_index: i,
                            maximum_number_of_hits: number_of_hits,
                        }));
                    }
                } else {
                    self.process_damage_effect(&used_move, None);
                }
            },
            MoveCategory::Status => {
                // TODO
            },
        }

        self.process_secondary_effect(&used_move);
    }

    fn process_damage_effect(
        &mut self,
        used_move: &UsedMove,
        multi_hit_data: Option<MultiHitData>,
    ) {
        let is_critical_hit = used_move.movement.critical_hit;

        let (attack, defense) = match (is_critical_hit, used_move.movement.category) {
            (false, MoveCategory::Physical) => (
                self.get_stat(used_move.user, Stat::Attack),
                self.get_stat(used_move.target, Stat::Defense),
            ),
            (true, MoveCategory::Physical) => (
                self.get_attack_critical_hit(used_move.user),
                self.get_defense_critical_hit(used_move.target),
            ),
            (false, MoveCategory::Special) => (
                self.get_stat(used_move.user, Stat::SpecialAttack),
                self.get_stat(used_move.target, Stat::SpecialDefense),
            ),
            (true, MoveCategory::Special) => (
                self.get_special_attack_critical_hit(used_move.user),
                self.get_special_defense_critical_hit(used_move.target),
            ),
            _ => unreachable!(),
        };

        self.inflict_damage(&used_move, attack, defense, is_critical_hit, multi_hit_data);
    }

    fn process_secondary_effect(&mut self, used_move: &UsedMove) {
        if let Some(effect) = used_move.movement.secondary_effect.as_ref() {
            if !self.rng.check_secondary_effect(effect.chance) {
                return;
            }

            match &effect.effect {
                SimpleEffect::Confusion => {
                    let duration = self.rng.get_confusion_duration();

                    self.add_volatile_status_condition(used_move.target, Flag::Confusion {
                        remaining_move_attempts: duration,
                    });
                },
                SimpleEffect::Flinch => {
                    self.add_volatile_status_condition(used_move.target, Flag::Flinch);
                },
                SimpleEffect::StatChange { changes, target } => {
                    let target = match target {
                        SimpleEffectTarget::MoveTarget => used_move.target,
                        SimpleEffectTarget::MoveUser => used_move.user,
                    };

                    for (stat, delta) in changes {
                        self.change_stat_stage(target, *stat, *delta);
                    }
                },
                SimpleEffect::StatusCondition(status_condition) => {
                    self.add_non_volatile_status_condition(used_move.target, *status_condition);
                },
                _ => todo!(),
            }
        }
    }

    fn next_turn(&mut self) {
        self.process_turn_end_events();

        self.turn += 1;
        self.event_queue
            .push(BattleEvent::ChangeTurn(event::ChangeTurn {
                new_turn: self.turn,
            }));
    }

    fn process_turn_end_events(&mut self) {
        if let Some(index) = self.p1.active_pokemon {
            self.remove_flag(index, "flinch");

            self.active_effects
                .get(&index)
                .unwrap_or(&Vec::new())
                .clone()
                .iter()
                .filter_map(|effect| effect.on_turn_end)
                .for_each(|effect| {
                    effect(self, index);
                });
        }

        if let Some(index) = self.p2.active_pokemon {
            self.remove_flag(index, "flinch");

            self.active_effects
                .get(&index)
                .unwrap_or(&Vec::new())
                .clone()
                .iter()
                .filter_map(|effect| effect.on_turn_end)
                .for_each(|effect| {
                    effect(self, index);
                });
        }
    }

    fn decompose_input_events(&mut self) -> (FrontendEventKind, FrontendEventKind) {
        if self.input_events.len() != 2 {
            panic!(
                "Invalid number of input events: {}",
                self.input_events.len()
            );
        }

        let mut input_events = self.input_events.drain(..);
        let first = input_events.next().unwrap();
        let second = input_events.next().unwrap();

        match (first.team, second.team) {
            (Team::P1, Team::P2) => (first.event, second.event),
            (Team::P2, Team::P1) => (second.event, first.event),
            _ => panic!("Invalid input events: each event must be emitted by a different team."),
        }
    }
}

impl BattleBackend {
    fn inflict_damage(
        &mut self,
        used_move: &UsedMove,
        attack: usize,
        defense: usize,
        is_critical_hit: bool,
        multi_hit_data: Option<MultiHitData>,
    ) {
        let effectiveness = self.get_type_effectiveness(&used_move.movement, used_move.target);

        let (damage, is_ohko) = if used_move.movement.flags.contains(&MoveFlag::OneHitKO) {
            (None, true)
        } else {
            let damage =
                self.get_move_damage(&used_move, attack, defense, effectiveness, is_critical_hit);

            (Some(damage), false)
        };

        let target = self.pokemon_repository.get_mut(&used_move.target).unwrap();
        let mut damage = damage.unwrap_or(target.current_hp);

        self.active_effects
            .get(&used_move.user)
            .unwrap_or(&Vec::new())
            .iter()
            .filter_map(|effect| effect.on_try_deal_damage)
            .for_each(|effect| {
                damage = effect(
                    &self,
                    used_move.user,
                    used_move.target,
                    &used_move.movement,
                    damage
                );
            });

        self.inflict_calculated_damage(
            used_move.target,
            damage,
            TypeEffectiveness::from(effectiveness),
            is_critical_hit,
            multi_hit_data,
            is_ohko,
            DamageCause::Move,
        )
    }

    pub fn inflict_calculated_damage(
        &mut self,
        target: usize,
        damage: usize,
        effectiveness: TypeEffectiveness,
        is_critical_hit: bool,
        multi_hit_data: Option<MultiHitData>,
        is_ohko: bool,
        cause: DamageCause,
    ) {
        let target_pokemon = self.pokemon_repository.get_mut(&target).unwrap();
        target_pokemon.current_hp = target_pokemon.current_hp.saturating_sub(damage);

        let (multi_hit_index, is_last_multi_hit_damage) = match multi_hit_data {
            Some(data) => {
                let is_last_multi_hit_damage =
                    data.multi_hit_index == data.maximum_number_of_hits - 1
                    || target_pokemon.current_hp == 0;

                (Some(data.multi_hit_index), is_last_multi_hit_damage)
            },
            None => (None, true),
        };

        self.event_queue.push(BattleEvent::Damage(event::Damage {
            target,
            amount: damage,
            effectiveness,
            is_critical_hit,
            multi_hit_index,
            is_last_multi_hit_damage,
            is_ohko,
            cause,
        }));

        // TODO: trigger effects like Static

        if target_pokemon.current_hp == 0 {
            match self.get_pokemon_team(target) {
                Team::P1 => {
                    self.p1.active_pokemon = None;
                    self.p1.party.push_front(target);
                },
                Team::P2 => {
                    self.p2.active_pokemon = None;
                    self.p2.party.push_front(target);
                },
            }

            self.event_queue.push(BattleEvent::Faint(event::Faint {
                target,
            }));
        }
    }

    fn add_volatile_status_condition(&mut self, target: usize, flag: Flag) {
        self.add_flag(target, flag.clone());

        self.event_queue
            .push(BattleEvent::VolatileStatusCondition(event::VolatileStatusCondition {
                target,
                added_flag: flag,
            }));
    }

    fn add_non_volatile_status_condition(&mut self, target: usize, condition: StatusCondition) {
        if self.can_inflict_non_volatile_status_condition_to(target, condition.into()) {
            let target_pokemon = self.get_pokemon_mut(target);

            target_pokemon.status_condition = Some(condition);

            if !self.active_effects.contains_key(&target) {
                self.active_effects.insert(target, Vec::new());
            }

            self.active_effects
                .get_mut(&target)
                .unwrap()
                .push(get_status_condition_effect(condition.into()));

            self.event_queue
                .push(BattleEvent::NonVolatileStatusCondition(
                    event::NonVolatileStatusCondition {
                        target,
                        condition,
                    },
                ));
        }
    }

    pub fn remove_non_volatile_status_condition(&mut self, target: usize) {
        let target_pokemon = self.get_pokemon_mut(target);
        let condition = target_pokemon.status_condition.take().unwrap();

        self.event_queue.push(BattleEvent::ExpiredNonVolatileStatusCondition(
            event::ExpiredNonVolatileStatusCondition {
                target,
                condition: condition.into(),
            },
        ));
    }

    fn add_flag(&mut self, target: usize, flag: Flag) {
        let key = match flag {
            Flag::Confusion { .. } => "confusion",
            Flag::Flinch => "flinch",
            Flag::StatStages(_) => unreachable!(),
        };

        self.pokemon_flags
            .get_mut(&target)
            .unwrap()
            .flags
            .insert(key, flag);
    }

    fn remove_flag(&mut self, target: usize, flag_id: &str) {
        self.pokemon_flags
            .get_mut(&target)
            .unwrap()
            .flags
            .remove(flag_id);
    }

    fn change_stat_stage(&mut self, target: usize, stat: Stat, delta: i8) {
        let stat_stages = self
            .pokemon_flags
            .get_mut(&target)
            .unwrap()
            .flags
            .entry("stat_stages")
            .or_insert(Flag::StatStages(HashMap::default()));

        let mut stat_change_kind = match delta {
            -3 => StatChangeKind::SeverelyFell,
            -2 => StatChangeKind::HarshlyFell,
            -1 => StatChangeKind::Fell,
            1 => StatChangeKind::Rose,
            2 => StatChangeKind::SharplyRose,
            3 => StatChangeKind::DrasticallyRose,
            _ => unreachable!(),
        };

        match stat_stages {
            Flag::StatStages(stages) => {
                let value = stages.entry(stat).or_insert(0);

                if delta < 0 && *value == -6 {
                    stat_change_kind = StatChangeKind::WontGoAnyLower;
                } else if delta > 0 && *value == 6 {
                    stat_change_kind = StatChangeKind::WontGoAnyHigher;
                } else {
                    *value = (*value + delta).max(-6).min(6);
                }
            },
            _ => unreachable!(),
        }

        self.event_queue
            .push(BattleEvent::StatChange(event::StatChange {
                target,
                kind: stat_change_kind,
                stat,
            }));
    }
}

impl BattleBackend {
    pub fn get_species(&self, pokemon: usize) -> &PokemonSpeciesData {
        let pokedex = get_all_pokemon_species();
        let species_id = &self.get_pokemon(pokemon).species_id;

        pokedex.get_species(species_id).unwrap()
    }

    pub fn get_pokemon(&self, pokemon: usize) -> &Pokemon {
        &self.pokemon_repository[&pokemon]
    }

    pub fn get_pokemon_mut(&mut self, pokemon: usize) -> &mut Pokemon {
        self.pokemon_repository.get_mut(&pokemon).unwrap()
    }

    pub fn get_active_pokemon(&self, team: Team) -> impl Iterator<Item = &Pokemon> + '_ {
        let team_data = match team {
            Team::P1 => &self.p1,
            Team::P2 => &self.p2,
        };

        team_data
            .active_pokemon
            .iter()
            .map(move |pokemon| self.get_pokemon(*pokemon))
    }

    pub fn get_pokemon_team(&self, pokemon: usize) -> Team {
        if let Some(index) = self.p1.active_pokemon {
            if index == pokemon {
                return Team::P1;
            }
        }

        if let Some(index) = self.p2.active_pokemon {
            if index == pokemon {
                return Team::P2;
            }
        }

        if self.p1.party.contains(&pokemon) {
            return Team::P1;
        }

        if self.p2.party.contains(&pokemon) {
            return Team::P2;
        }

        unreachable!();
    }

    pub fn get_flag_mut(&mut self, pokemon: usize, flag_id: &str) -> Option<&mut Flag> {
        self.pokemon_flags
            .get_mut(&pokemon)
            .unwrap()
            .flags
            .get_mut(flag_id)
    }

    pub fn has_flag(&self, pokemon: usize, flag_id: &str) -> bool {
        self.pokemon_flags
            .get(&pokemon)
            .unwrap()
            .flags
            .contains_key(flag_id)
    }

    pub fn get_non_volatile_status_condition_mut(
        &mut self,
        pokemon: usize,
    ) -> Option<&mut StatusCondition> {
        self.get_pokemon_mut(pokemon).status_condition.as_mut()
    }

    pub fn has_non_volatile_status_condition(&self, pokemon: usize) -> bool {
        self.get_pokemon(pokemon).status_condition.is_some()
    }

    pub fn can_inflict_non_volatile_status_condition_to(
        &self,
        target: usize,
        condition: SimpleStatusCondition,
    ) -> bool {
        let target_is_not_immune = {
            let can_affect = get_status_condition_effect(condition)
                .can_affect
                .unwrap_or(|_, _| true);

            can_affect(self, target)
        };

        !self.has_non_volatile_status_condition(target) && target_is_not_immune
    }

    fn get_attack_critical_hit(&self, pokemon: usize) -> usize {
        self.get_positive_critical_hit_stat(pokemon, Stat::Attack)
    }

    fn get_defense_critical_hit(&self, pokemon: usize) -> usize {
        self.get_negative_critical_hit_stat(pokemon, Stat::Defense)
    }

    fn get_special_attack_critical_hit(&self, pokemon: usize) -> usize {
        self.get_positive_critical_hit_stat(pokemon, Stat::SpecialAttack)
    }

    fn get_special_defense_critical_hit(&self, pokemon: usize) -> usize {
        self.get_negative_critical_hit_stat(pokemon, Stat::SpecialDefense)
    }

    /// Returns the value of a stat, ignoring negative stat changes.
    fn get_positive_critical_hit_stat(&self, pokemon: usize, stat: Stat) -> usize {
        // TODO: take other factors into account
        let stat_stage = self.get_stat_stage(pokemon, stat).max(0);
        let multiplier = self.get_stat_stage_multiplier(stat_stage);
        let pure_stat = self.get_pure_stat(pokemon, stat);

        (multiplier * pure_stat as f32) as usize
    }

    /// Returns the value of a stat, ignoring positive stat changes.
    fn get_negative_critical_hit_stat(&self, pokemon: usize, stat: Stat) -> usize {
        // TODO: take other factors into account
        let stat_stage = self.get_stat_stage(pokemon, stat).min(0);
        let multiplier = self.get_stat_stage_multiplier(stat_stage);
        let pure_stat = self.get_pure_stat(pokemon, stat);

        (multiplier * pure_stat as f32) as usize
    }

    /// Returns the effective value of a stat.
    pub fn get_stat(&self, pokemon: usize, stat: Stat) -> usize {
        let stat_stage = self.get_stat_stage(pokemon, stat);
        let multiplier = self.get_stat_stage_multiplier(stat_stage);
        let pure_stat = self.get_pure_stat(pokemon, stat);

        let mut result = (multiplier * pure_stat as f32) as usize;

        self.active_effects
            .get(&pokemon)
            .unwrap_or(&Vec::new())
            .clone()
            .iter()
            .filter_map(|effect| effect.on_stat_calculation)
            .for_each(|effect| {
                result = effect(self, pokemon, stat, result);
            });

        result
    }

    /// Returns the value of a stat without considering stat stages and other
    /// factors.
    fn get_pure_stat(&self, pokemon: usize, stat: Stat) -> usize {
        self.pokemon_repository[&pokemon].stats[stat as usize]
    }

    fn get_stat_stage(&self, pokemon: usize, stat: Stat) -> i8 {
        let hash_map = self.pokemon_flags[&pokemon].flags.get("stat_stages");

        if let Some(hash_map) = hash_map {
            if let Flag::StatStages(stages) = hash_map {
                *stages.get(&stat).unwrap_or(&0)
            } else {
                unreachable!()
            }
        } else {
            0
        }
    }

    fn get_stat_stage_multiplier(&self, stage: i8) -> f32 {
        let stage = stage.max(-6).min(6) as f32;

        if stage >= 0. {
            (2. + stage) / 2.
        } else {
            2. / (2. - stage)
        }
    }

    fn get_accuracy_multiplier(&self, stage: i8) -> f32 {
        let stage = stage.max(-6).min(6) as f32;

        if stage >= 0. {
            (3. + stage) / 3.
        } else {
            3. / (3. - stage)
        }
    }

    fn get_move_power(&self, used_move: &UsedMove) -> usize {
        let mov = used_move.movement;

        match mov.power_modifier {
            Some(modifier) => {
                let user = &self.pokemon_repository[&used_move.user];
                let target = &self.pokemon_repository[&used_move.target];
                modifier(user, target, mov)
            },
            None => match mov.base_power {
                MovePower::Constant(value) => value,
                MovePower::Special => 0,
            },
        }
    }

    fn get_move_damage(
        &mut self,
        used_move: &UsedMove,
        attack: usize,
        defense: usize,
        effectiveness: f32,
        is_critical_hit: bool,
    ) -> usize {
        let level = self.pokemon_repository[&used_move.user].level as f32;
        let level_modifier = (2. * level) / 5. + 2.;
        let power = self.get_move_power(&used_move) as f32;
        let stat_ratio = (attack as f32) / (defense as f32);

        let modifier = {
            let targets = 1.; // TODO: handle multi-target moves
            let weather = 1.; // TODO
            let critical = if is_critical_hit { 1.25 } else { 1. };
            let random = self.rng.get_damage_modifier();
            let stab = if self.check_stab(&used_move.movement, used_move.user) {
                1.5
            } else {
                1.
            };
            let other = 1.; // TODO

            targets * weather * critical * random * stab * effectiveness * other
        };

        let power_stat_ratio = (power * stat_ratio).floor();
        let level_power_stat_ratio = ((level_modifier * power_stat_ratio) / 50.).floor();

        let damage = (level_power_stat_ratio + 2.) * modifier;
        let damage = damage as usize;

        if damage == 0 {
            1
        } else {
            damage
        }
    }

    fn get_type_effectiveness(&self, mov: &Move, target: usize) -> f32 {
        // TODO: handle moves without types (e.g Struggle)

        self.get_pokemon_current_types(target)
            .map(|t| PokemonType::get_effectiveness(mov.move_type, *t))
            .product()
    }

    fn check_stab(&self, mov: &Move, user: usize) -> bool {
        // TODO: handle moves without types (e.g Struggle)

        self.has_type(user, mov.move_type)
    }

    pub fn has_type(&self, target: usize, tested_type: PokemonType) -> bool {
        self.get_pokemon_current_types(target)
            .find(|t| **t == tested_type)
            .is_some()
    }

    fn get_pokemon_current_types(&self, target: usize) -> impl Iterator<Item = &PokemonType> {
        // TODO: handle Pokémon that currently have a different type than their
        // original ones (e.g after Soak or Roost)

        self.get_species(target)
            .types
            .iter()
    }

    fn check_miss(&mut self, used_move: &UsedMove) -> bool {
        let mov = used_move.movement;

        let mut accuracy = match mov.accuracy_modifier {
            Some(modifier) => {
                modifier(self, used_move.user, used_move.target, mov)
            },
            None => ModifiedAccuracy::OriginalValue,
        };

        if accuracy == ModifiedAccuracy::OriginalValue {
            accuracy = match mov.accuracy {
                Some(accuracy) => ModifiedAccuracy::NewValue(accuracy),
                None => ModifiedAccuracy::Hit,
            };
        }

        match accuracy {
            ModifiedAccuracy::Miss => true,
            ModifiedAccuracy::Hit => false,
            ModifiedAccuracy::OriginalValue => unreachable!(),
            ModifiedAccuracy::NewValue(accuracy) => {
                let accuracy = accuracy as f32;

                let adjusted_stages = {
                    let user_accuracy = self.get_stat_stage(used_move.user, Stat::Accuracy);
                    let target_evasion = self.get_stat_stage(used_move.target, Stat::Evasion);

                    self.get_accuracy_multiplier(user_accuracy - target_evasion)
                };

                let chance = accuracy * adjusted_stages;

                self.rng.check_miss(chance as usize)
            },
        }
    }

    pub fn check_paralysis_move_prevention(&mut self) -> bool {
        self.rng.check_paralysis_move_prevention()
    }

    pub fn check_freeze_thaw(&mut self) -> bool {
        self.rng.check_freeze_thaw()
    }

    fn is_fainted(&self, pokemon: usize) -> bool {
        self.get_pokemon(pokemon).current_hp == 0
    }
}
