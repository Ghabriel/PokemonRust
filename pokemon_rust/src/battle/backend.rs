use crate::entities::pokemon::Pokemon;

use std::collections::VecDeque;

use super::types::Battle;

/// Represents an event that can be sent from the frontend to the backend.
#[derive(Debug)]
pub struct FrontendEvent {
    team: Team,
    event: FrontendEventKind,
}

/// The kind of events that the frontend can send to the backend.
#[derive(Debug)]
pub enum FrontendEventKind {
    UseMove(usize),
}

/// The kind of events that the backend can send to the frontend.
#[derive(Debug)]
pub enum BattleEvent {
    InitialSwitchIn(Team, Pokemon),
    ChangeTurn(usize),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Team {
    P1,
    P2,
}

pub struct BattleBackend {
    data: Battle,
    input_events: VecDeque<FrontendEvent>,
    event_queue: Vec<BattleEvent>,
}

impl BattleBackend {
    pub fn new(data: Battle) -> BattleBackend {
        BattleBackend {
            data,
            input_events: VecDeque::new(),
            event_queue: Vec::new(),
        }
    }

    pub fn push_frontend_event(&mut self, event: FrontendEvent) {
        self.input_events.push_back(event);
    }

    pub fn tick(&mut self) -> impl Iterator<Item = BattleEvent> + '_ {
        if self.data.turn == 0 {
            self.first_tick();
            self.next_turn();
            return self.event_queue.drain(..);
        }

        let (p1_action, p2_action) = self.decompose_input_events();
        let p1 = self.data.p1.active_pokemon.as_mut().unwrap();
        let p2 = self.data.p2.active_pokemon.as_mut().unwrap();

        match (p1_action, p2_action) {
            (FrontendEventKind::UseMove(p1_index), FrontendEventKind::UseMove(p2_index)) => {
                let p1_move = p1.moves[p1_index].as_ref().unwrap();
                let p2_move = p1.moves[p1_index].as_ref().unwrap();
            },
        }

        self.next_turn();
        self.event_queue.drain(..)
    }

    fn first_tick(&mut self) {
        self.data.p1.active_pokemon = self.data.p1.party.pokemon.pop_front();
        assert!(self.data.p1.active_pokemon.is_some());
        self.event_queue.push(BattleEvent::InitialSwitchIn(
            Team::P1,
            self.data.p1.active_pokemon.as_ref().unwrap().clone(),
        ));

        self.data.p2.active_pokemon = self.data.p2.party.pokemon.pop_front();
        assert!(self.data.p2.active_pokemon.is_some());
        self.event_queue.push(BattleEvent::InitialSwitchIn(
            Team::P2,
            self.data.p2.active_pokemon.as_ref().unwrap().clone(),
        ));

        // TODO: trigger things like Intimidate, entry hazards, Drought, etc
        // TODO: in which order?
    }

    fn next_turn(&mut self) {
        self.data.turn += 1;
        self.event_queue.push(BattleEvent::ChangeTurn(self.data.turn));
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
