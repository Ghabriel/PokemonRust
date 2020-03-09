use amethyst::input::{InputEvent, StringBindings};

use crate::{
    audio::Sound,
    battle::{
        backend::BattleBackend,
        rng::StandardBattleRng,
    },
    constants::AXIS_SENSITIVITY,
};

use super::super::{BattleSystemData, FrontendEvent, TickResult};

use super::SelectionScreen;

pub enum MoveSelectionScreenEvent {
    PendingStart,
    Started {
        selection_screen: SelectionScreen,
    },
}

impl MoveSelectionScreenEvent {
    fn select_option(&mut self, _system_data: &mut BattleSystemData) -> TickResult {
        if let Self::Started { selection_screen, .. } = self {
            let focused_option = selection_screen.get_focused_option();

            println!("Selected option {}", focused_option);
        }

        TickResult::Incomplete
    }
}

impl FrontendEvent for MoveSelectionScreenEvent {
    fn start(
        &mut self,
        _backend: &BattleBackend<StandardBattleRng>,
        system_data: &mut BattleSystemData,
    ) {
        *self = MoveSelectionScreenEvent::Started {
            selection_screen: SelectionScreen::new(
                160.,
                vec![
                    system_data.resources.fight_button.clone(),
                    system_data.resources.fight_button.clone(),
                    system_data.resources.fight_button.clone(),
                    system_data.resources.fight_button.clone(),
                ],
                system_data,
            )
        };
    }

    fn tick(
        &mut self,
        input_events: Vec<InputEvent<StringBindings>>,
        _backend: &BattleBackend<StandardBattleRng>,
        system_data: &mut BattleSystemData
    ) -> TickResult {
        for event in input_events {
            let BattleSystemData { sound_kit, .. } = system_data;

            if let Self::Started { selection_screen, .. } = self {
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
                    }
                    _ => {},
                }
            } else {
                panic!("Called tick() before start()");
            }
        }

        TickResult::Incomplete
    }
}
