use amethyst::{
    animation::{
        AnimationCommand,
        AnimationControlSet,
        ControlState,
    },
    ecs::{
        Join,
        ReadExpect,
        System,
        WriteStorage,
    },
    renderer::SpriteRender,
};

use crate::{
    entities::player::{Player, PlayerAction, PlayerSpriteSheets},
};

pub struct PlayerMovementSystem;

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'a, AnimationControlSet<PlayerAction, SpriteRender>>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, SpriteRender>,
        ReadExpect<'a, PlayerSpriteSheets>,
    );

    fn run(&mut self, (
        mut control_sets,
        mut players,
        mut sprite_renders,
        sprite_sheets,
    ): Self::SystemData) {
        for (control_set, player, sprite_render) in (&mut control_sets, &mut players, &mut sprite_renders).join() {
            if player.temp_flag {
                match player.action {
                    PlayerAction::Walk => player_walk(
                        control_set,
                        sprite_render,
                        &sprite_sheets,
                    ),
                    PlayerAction::Run => player_run(
                        control_set,
                        sprite_render,
                        &sprite_sheets,
                    ),
                }

                player.temp_flag = false;
            }
        }
    }
}

pub fn player_walk(
    control_set: &mut AnimationControlSet<PlayerAction, SpriteRender>,
    sprite_render: &mut SpriteRender,
    sprite_sheets: &PlayerSpriteSheets,
) {
    sprite_render.sprite_sheet = sprite_sheets.walking.clone();
    sprite_render.sprite_number = 0;

    control_set.animations
        .iter_mut()
        .filter(|(id, _)| *id != PlayerAction::Walk)
        .for_each(|(_, animation)| {
            animation.command = AnimationCommand::Pause;
        });

    let (_, animation) = control_set.animations
        .iter_mut()
        .find(|(id, _)| *id == PlayerAction::Walk)
        .unwrap();
    animation.state = ControlState::Requested;
    animation.command = AnimationCommand::Start;
}

pub fn player_run(
    control_set: &mut AnimationControlSet<PlayerAction, SpriteRender>,
    sprite_render: &mut SpriteRender,
    sprite_sheets: &PlayerSpriteSheets,
) {
    sprite_render.sprite_sheet = sprite_sheets.running.clone();
    sprite_render.sprite_number = 0;

    control_set.animations
        .iter_mut()
        .filter(|(id, _)| *id != PlayerAction::Run)
        .for_each(|(_, animation)| {
            animation.command = AnimationCommand::Pause;
        });

    let (_, animation) = control_set.animations
        .iter_mut()
        .find(|(id, _)| *id == PlayerAction::Run)
        .unwrap();
    animation.state = ControlState::Requested;
    animation.command = AnimationCommand::Start;
}
