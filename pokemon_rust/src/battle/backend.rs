use crate::entities::{
    character::CharacterId,
    pokemon::{
        get_all_moves,
        movement::{Move, MoveCategory, MovePower},
        Pokemon,
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
    InitialSwitchIn(Team, usize),
    ChangeTurn(usize),
    Damage { target: usize, amount: usize },
}

#[derive(Debug, Eq, PartialEq)]
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
    input_events: VecDeque<FrontendEvent>,
    event_queue: Vec<BattleEvent>,
    pub(super) pokemon_repository: HashMap<usize, Pokemon>,
    /// The RNG that this battle is using.
    rng: Rng,
}

#[derive(Debug)]
pub(super) struct TeamData {
    pub(super) active_pokemon: Option<usize>,
    party: VecDeque<usize>,
    character_id: Option<CharacterId>,
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

        for pokemon in data.p1.party.pokemon {
            let index = pokemon_repository.len();
            pokemon_repository.insert(index, pokemon);
            p1.party.push_back(index);
        }

        for pokemon in data.p2.party.pokemon {
            let index = pokemon_repository.len();
            pokemon_repository.insert(index, pokemon);
            p2.party.push_back(index);
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
        self.event_queue.push(BattleEvent::InitialSwitchIn(
            Team::P1,
            self.p1.active_pokemon.as_ref().unwrap().clone(),
        ));

        self.p2.active_pokemon = self.p2.party.pop_front();
        assert!(self.p2.active_pokemon.is_some());
        self.event_queue.push(BattleEvent::InitialSwitchIn(
            Team::P2,
            self.p2.active_pokemon.as_ref().unwrap().clone(),
        ));

        // TODO: trigger things like Intimidate, entry hazards, Drought, etc
        // TODO: in which order?
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
        match used_move.movement.category {
            MoveCategory::Physical => {
                let attack = self.get_stat(used_move.user, Stat::Attack);
                let defense = self.get_stat(used_move.user, Stat::Defense);
                self.inflict_damage(&used_move, attack, defense);
            },
            MoveCategory::Special => {
                let attack = self.get_stat(used_move.user, Stat::SpecialAttack);
                let defense = self.get_stat(used_move.user, Stat::SpecialDefense);
                self.inflict_damage(&used_move, attack, defense);
            },
            MoveCategory::Status => {
                // TODO
            },
        }
    }

    fn next_turn(&mut self) {
        self.turn += 1;
        self.event_queue.push(BattleEvent::ChangeTurn(self.turn));
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
    fn inflict_damage(&mut self, used_move: &UsedMove, attack: usize, defense: usize) {
        let damage = self.get_move_damage(&used_move, attack, defense);

        let target = self.pokemon_repository.get_mut(&used_move.target).unwrap();
        target.current_hp = target.current_hp.saturating_sub(damage);

        self.event_queue.push(BattleEvent::Damage {
            target: used_move.target,
            amount: damage,
        });

        // TODO: trigger effects like Static
    }
}

impl<Rng: BattleRng> BattleBackend<Rng> {
    fn get_stat(&self, pokemon: usize, stat: Stat) -> usize {
        // TODO: take stat stages and other factors into account
        self.pokemon_repository[&pokemon].stats[stat as usize]
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

    fn get_move_damage(&mut self, used_move: &UsedMove, attack: usize, defense: usize) -> usize {
        let level = self.pokemon_repository[&used_move.user].level as f32;
        let level_modifier = (2. * level) / 5. + 2.;
        let power = self.get_move_power(&used_move) as f32;
        let stat_ratio = (attack as f32) / (defense as f32);

        let modifier = {
            let targets = 1.; // TODO: handle multi-target moves
            let weather = 1.; // TODO
            let critical = 1.; // TODO
            let random = self.rng.get_damage_modifier();
            let stab = 1.; // TODO
            let effectiveness = 1.; // TODO
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
}
