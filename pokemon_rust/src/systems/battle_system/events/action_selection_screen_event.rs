use amethyst::input::{InputEvent, StringBindings};

use crate::{
    audio::Sound,
    battle::{
        backend::BattleBackend,
        rng::StandardBattleRng,
    },
    constants::AXIS_SENSITIVITY,
};

use super::super::{BattleSystemData, FrontendEvent};

use super::SelectionScreen;

pub enum ActionSelectionScreenEvent {
    PendingStart,
    Started {
        selection_screen: SelectionScreen,
    },
}

impl ActionSelectionScreenEvent {
    fn select_option(&mut self) {
        if let Self::Started { selection_screen, .. } = self {
            match selection_screen.get_focused_option() {
                0 => self.select_fight_option(),
                1 => self.select_run_option(),
                _ => unreachable!(),
            }
        }
    }

    fn select_fight_option(&mut self) {
        // TODO
        println!("Selected option: fight");
    }

    fn select_run_option(&mut self) {
        // TODO
        println!("Selected option: run");
    }
}

impl FrontendEvent for ActionSelectionScreenEvent {
    fn start(
        &mut self,
        _backend: &BattleBackend<StandardBattleRng>,
        system_data: &mut BattleSystemData,
    ) {
        *self = ActionSelectionScreenEvent::Started {
            selection_screen: SelectionScreen::new(
                160.,
                vec![
                    system_data.resources.fight_button.clone(),
                    system_data.resources.run_button.clone(),
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
    ) -> bool {
        for event in input_events {
            let BattleSystemData { sound_kit, .. } = system_data;

            if let Self::Started { selection_screen, .. } = self {
                match event {
                    InputEvent::ActionPressed(action) if action == "action" => {
                        sound_kit.play_sound(Sound::SelectOption);
                        self.select_option();
                    },
                    InputEvent::AxisMoved { axis, value } if axis == "vertical" => {
                        let offset = if value < -AXIS_SENSITIVITY {
                            -1
                        } else if value > AXIS_SENSITIVITY {
                            1
                        } else {
                            return false;
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

        false
    }
}
