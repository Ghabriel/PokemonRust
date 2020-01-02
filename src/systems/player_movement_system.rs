use amethyst::{
    core::{Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::entities::player::{Direction, Player, PlayerAction};

#[derive(Default)]
pub struct PlayerMovementSystem;

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
    );

    fn run(&mut self, (players, mut transforms, time): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {
            if !player.moving {
                continue;
            }

            let (offset_x, offset_y) = match player.facing_direction {
                Direction::Up => (0., 1.),
                Direction::Down => (0., -1.),
                Direction::Left => (-1., 0.),
                Direction::Right => (1., 0.),
            };

            let velocity = match player.action {
                PlayerAction::Idle => unreachable!(),
                PlayerAction::Walk => 160.,
                PlayerAction::Run => 256.,
            };

            transform.prepend_translation_x(offset_x * velocity * time.delta_seconds());
            transform.prepend_translation_y(offset_y * velocity * time.delta_seconds());
        }
    }
}
