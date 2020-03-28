use amethyst::input::{InputEvent, StringBindings};

use crate::{
    audio::Sound,
    battle::backend::{
        rng::StandardBattleRng,
        BattleBackend,
        FrontendEvent,
        FrontendEventKind,
        Team,
    },
    constants::AXIS_SENSITIVITY,
};

use super::super::{BattleSystemData, FrontendAnimation, TickResult};

use super::SelectionScreen;

pub enum MoveSelectionScreen {
    PendingStart,
    Started { selection_screen: SelectionScreen },
}

impl MoveSelectionScreen {
    fn select_option(&mut self, system_data: &mut BattleSystemData) -> TickResult {
        if let Self::Started {
            selection_screen, ..
        } = self {
            let move_index = selection_screen.get_focused_option();
            selection_screen.remove(system_data);

            TickResult::emit(FrontendEvent {
                team: Team::P1,
                event: FrontendEventKind::UseMove(move_index.into()),
            })
        } else {
            TickResult::Incomplete
        }
    }
}

impl FrontendAnimation for MoveSelectionScreen {
    fn start(
        &mut self,
        _backend: &BattleBackend,
        system_data: &mut BattleSystemData,
    ) {
        *self = MoveSelectionScreen::Started {
            selection_screen: SelectionScreen::new(
                160.,
                vec![
                    system_data.resources.fight_button.clone(),
                    system_data.resources.fight_button.clone(),
                    system_data.resources.fight_button.clone(),
                    system_data.resources.fight_button.clone(),
                ],
                system_data,
            ),
        };
    }

    fn tick(
        &mut self,
        input_events: Vec<InputEvent<StringBindings>>,
        _backend: &BattleBackend,
        system_data: &mut BattleSystemData,
    ) -> TickResult {
        for event in input_events {
            let BattleSystemData { sound_kit, .. } = system_data;

            if let Self::Started {
                selection_screen, ..
            } = self {
                match event {
                    InputEvent::ActionPressed(action) if action == "action" => {
                        sound_kit.play_sound(Sound::SelectOption);
                        return self.select_option(system_data);
                    },
                    InputEvent::AxisMoved { axis, value } if axis == "vertical" => {
                        let offset = if value < -AXIS_SENSITIVITY {
                            1
                        } else if value > AXIS_SENSITIVITY {
                            -1
                        } else {
                            return TickResult::Incomplete;
                        };

                        sound_kit.play_sound(Sound::SelectOption);
                        selection_screen.move_selection(offset, system_data);
                    },
                    _ => {},
                }
            } else {
                panic!("Called tick() before start()");
            }
        }

        TickResult::Incomplete
    }
}
