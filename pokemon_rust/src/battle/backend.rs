use crate::entities::{
    character::CharacterId,
    pokemon::{
        get_all_moves,
        get_all_pokemon_species,
        movement::{
            Move,
            MoveCategory,
            MovePower,
            SimpleEffect,
            SimpleEffectTarget,
        },
        Pokemon,
        PokemonSpeciesData,
        PokemonType,
        Stat,
    },
};

use std::collections::{HashMap, VecDeque};

use super::{
    rng::BattleRng,
    types::{Battle, BattleType},
};

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
    Damage(event::Damage),
    Miss(event::Miss),
    StatChange(event::StatChange),
}

pub mod event {
    use super::{StatChangeKind, Team, TypeEffectiveness};

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
    pub struct Damage {
        pub target: usize,
        pub amount: usize,
        pub effectiveness: TypeEffectiveness,
        pub is_critical_hit: bool,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Miss {
        pub move_user: usize,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct StatChange {
        pub target: usize,
        pub kind: StatChangeKind,
    }
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
pub struct BattleBackend<Rng: BattleRng> {
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
    input_events: VecDeque<FrontendEvent>,
    event_queue: Vec<BattleEvent>,
    pub(super) pokemon_repository: HashMap<usize, Pokemon>,
    /// The RNG that this battle is using.
    pub(super) rng: Rng,
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

#[derive(Debug)]
enum Flag {
    StatStages(HashMap<Stat, i8>),
}

impl<Rng: BattleRng> BattleBackend<Rng> {
    pub fn new(data: Battle, rng: Rng) -> BattleBackend<Rng> {
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

        self.event_queue.push(BattleEvent::InitialSwitchIn(event::InitialSwitchIn {
            team: Team::P2,
            pokemon: self.p2.active_pokemon.as_ref().unwrap().clone(),
            is_already_sent_out: self.p2.character_id.is_none(),
        }));

        self.event_queue.push(BattleEvent::InitialSwitchIn(event::InitialSwitchIn {
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
                    let p1_move_id = self.pokemon_repository[&p1].moves[p1_index].as_ref().unwrap();
                    let p1_move = movedex.get_move(&p1_move_id).unwrap();

                    UsedMove {
                        user: p1,
                        target: p2,
                        movement: &p1_move,
                    }
                };

                let p2_move = {
                    let p2_move_id = self.pokemon_repository[&p2].moves[p2_index].as_ref().unwrap();
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

    fn process_moves<'a>(
        &mut self,
        moves: impl Iterator<Item = UsedMove<'a>>,
    ) {
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
            b.movement.priority
                .cmp(&a.movement.priority)
                .then_with(|| {
                    let a_speed = self.get_stat(a.user, Stat::Speed);
                    let b_speed = self.get_stat(b.user, Stat::Speed);

                    b_speed.cmp(&a_speed)
                })
        });

        result.into_iter()
    }

    fn process_move(&mut self, used_move: UsedMove) {
        if self.check_miss(&used_move) {
            self.event_queue.push(BattleEvent::Miss(event::Miss {
                move_user: used_move.user
            }));
            return;
        }

        match used_move.movement.category {
            MoveCategory::Physical | MoveCategory::Special => {
                self.process_damage_effect(&used_move);
            },
            MoveCategory::Status => {
                // TODO
            },
        }

        self.process_secondary_effect(&used_move);
    }

    fn process_damage_effect(&mut self, used_move: &UsedMove) {
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

        self.inflict_damage(&used_move, attack, defense, is_critical_hit);
    }

    fn process_secondary_effect(&mut self, used_move: &UsedMove) {
        if let Some(effect) = used_move.movement.secondary_effect.as_ref() {
            if !self.rng.check_secondary_effect(effect.chance) {
                return;
            }

            match &effect.effect {
                SimpleEffect::StatChange { changes, target } => {
                    let target = match target {
                        SimpleEffectTarget::MoveTarget => used_move.target,
                        SimpleEffectTarget::MoveUser => used_move.user,
                    };

                    for (stat, delta) in changes {
                        self.change_stat_stage(target, *stat, *delta);
                    }
                },
                _ => todo!(),
            }
        }
    }

    fn next_turn(&mut self) {
        self.turn += 1;
        self.event_queue.push(BattleEvent::ChangeTurn(event::ChangeTurn {
            new_turn: self.turn
        }));
    }

    fn decompose_input_events(&mut self) -> (FrontendEventKind, FrontendEventKind) {
        if self.input_events.len() != 2 {
            panic!("Invalid number of input events: {}", self.input_events.len());
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

impl<Rng: BattleRng> BattleBackend<Rng> {
    fn inflict_damage(
        &mut self,
        used_move: &UsedMove,
        attack: usize,
        defense: usize,
        is_critical_hit: bool,
    ) {
        let effectiveness = self.get_type_effectiveness(&used_move.movement, used_move.target);
        let damage = self.get_move_damage(
            &used_move,
            attack,
            defense,
            effectiveness,
            is_critical_hit,
        );

        let target = self.pokemon_repository.get_mut(&used_move.target).unwrap();
        target.current_hp = target.current_hp.saturating_sub(damage);

        self.event_queue.push(BattleEvent::Damage(event::Damage {
            target: used_move.target,
            amount: damage,
            effectiveness: TypeEffectiveness::from(effectiveness),
            is_critical_hit,
        }));

        // TODO: trigger effects like Static
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
        }

        self.event_queue.push(BattleEvent::StatChange(event::StatChange {
            target,
            kind: stat_change_kind,
        }));
    }
}

impl<Rng: BattleRng> BattleBackend<Rng> {
    pub fn get_species(&self, pokemon: usize) -> &PokemonSpeciesData {
        let pokedex = get_all_pokemon_species();
        let species_id = &self.get_pokemon(pokemon).species_id;

        pokedex.get_species(species_id).unwrap()
    }

    pub fn get_pokemon(&self, pokemon: usize) -> &Pokemon {
        &self.pokemon_repository[&pokemon]
    }

    pub fn get_active_pokemon(&self, team: Team) -> impl Iterator<Item = &Pokemon> + '_ {
        let team_data = match team {
            Team::P1 => &self.p1,
            Team::P2 => &self.p2,
        };

        team_data.active_pokemon
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
    fn get_stat(&self, pokemon: usize, stat: Stat) -> usize {
        // TODO: take other factors into account
        let stat_stage = self.get_stat_stage(pokemon, stat);
        let multiplier = self.get_stat_stage_multiplier(stat_stage);
        let pure_stat = self.get_pure_stat(pokemon, stat);

        (multiplier * pure_stat as f32) as usize
    }

    /// Returns the value of a stat without considering stat stages and other
    /// factors.
    fn get_pure_stat(&self, pokemon: usize, stat: Stat) -> usize {
        self.pokemon_repository[&pokemon].stats[stat as usize]
    }

    fn get_stat_stage(&self, pokemon: usize, stat: Stat) -> i8 {
        let hash_map = self
            .pokemon_flags[&pokemon]
            .flags
            .get("stat_stages");

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
            2. / (2. + stage)
        }
    }

    fn get_accuracy_multiplier(&self, stage: i8) -> f32 {
        let stage = stage.max(-6).min(6) as f32;

        if stage >= 0. {
            (3. + stage) / 3.
        } else {
            3. / (3. + stage)
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
            let critical = if is_critical_hit {
                1.25
            } else {
                1.
            };
            let random = self.rng.get_damage_modifier();
            let stab = if self.check_stab(&used_move.movement, used_move.user) {
                1.5
            } else {
                1.
            };
            let burn = 1.; // TODO
            let other = 1.; // TODO

            targets * weather * critical * random * stab * effectiveness * burn * other
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
        // TODO: handle Pokémon that currently have a different type than their
        // original ones (e.g after Soak or Roost)
        // TODO: handle moves without types (e.g Struggle)

        self.get_species(target)
            .types
            .iter()
            .map(|t| PokemonType::get_effectiveness(mov.move_type, *t))
            .product()
    }

    fn check_stab(&self, mov: &Move, user: usize) -> bool {
        // TODO: handle Pokémon that currently have a different type than their
        // original ones (e.g after Soak or Roost)
        // TODO: handle moves without types (e.g Struggle)

        let pokedex = get_all_pokemon_species();
        let user_species_id = &self.pokemon_repository[&user].species_id;

        pokedex
            .get_species(user_species_id)
            .unwrap()
            .types
            .iter()
            .find(|t| **t == mov.move_type)
            .is_some()
    }

    fn check_miss(&mut self, used_move: &UsedMove) -> bool {
        if let Some(accuracy) = used_move.movement.accuracy {
            let accuracy = accuracy as f32;

            let adjusted_stages = {
                let user_accuracy = self.get_stat_stage(used_move.user, Stat::Accuracy);
                let target_evasion = self.get_stat_stage(used_move.target, Stat::Evasion);

                self.get_accuracy_multiplier(user_accuracy - target_evasion)
            };

            let chance = accuracy * adjusted_stages;

            self.rng.check_miss(chance as usize)
        } else {
            false
        }
    }
}
