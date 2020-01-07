use amethyst::core::{math::Vector3, Transform};

use crate::{
    constants::TILE_SIZE,
    entities::player::{Direction, Player},
};

pub fn get_forward_tile_position(player: &Player, player_position: &Transform) -> Vector3<f32> {
    let (offset_x, offset_y) = match player.facing_direction {
        Direction::Up => (0., 1.),
        Direction::Down => (0., -1.),
        Direction::Left => (-1., 0.),
        Direction::Right => (1., 0.),
    };

    let tile_size = TILE_SIZE as f32;

    player_position.translation() + Vector3::new(
        offset_x * tile_size,
        offset_y * tile_size,
        0.,
    )
}
