use amethyst::{
    core::{math::Vector3, Time, Transform},
    ecs::{
        Entities,
        Join,
        Read,
        ReadStorage,
        System,
        WriteExpect,
        WriteStorage,
    },
};

use crate::{
    common::get_direction_offset,
    entities::npc::{Npc, NpcMovement},
    map::{CoordinateSystem, MapHandler},
};

#[derive(Default)]
pub struct NpcMovementSystem;

impl<'a> System<'a> for NpcMovementSystem {
    type SystemData = (
        ReadStorage<'a, Npc>,
        WriteStorage<'a, NpcMovement>,
        WriteStorage<'a, Transform>,
        WriteExpect<'a, MapHandler>,
        Entities<'a>,
        Read<'a, Time>,
    );

    fn run(&mut self, (
        npcs,
        mut npc_movements,
        mut transforms,
        mut map,
        entities,
        time,
    ): Self::SystemData) {
        let mut static_npcs = Vec::new();

        for (entity, npc, movement_data, transform) in (
            &entities,
            &npcs,
            &mut npc_movements,
            &mut transforms,
        ).join() {
            // TODO: extract velocity to constant or use GameConfig::player_walking_speed
            let velocity = 160.;

            let delta_seconds = time.delta_seconds();

            if !movement_data.started {
                map.mark_tile_as_solid(&movement_data.to);
                movement_data.started = true;
            }

            if movement_data.estimated_time <= delta_seconds {
                transform.set_translation(Vector3::new(
                    movement_data.to.position.x(),
                    movement_data.to.position.y(),
                    0.,
                ));

                map.remove_solid_mark(&movement_data.from);
                static_npcs.push(entity);
                continue;
            }

            movement_data.estimated_time -= delta_seconds;

            let (offset_x, offset_y) = get_direction_offset::<f32>(&npc.facing_direction);
            transform.prepend_translation_x(offset_x * velocity * time.delta_seconds());
            transform.prepend_translation_y(offset_y * velocity * time.delta_seconds());
        }

        for entity in static_npcs {
            npc_movements.remove(entity);
        }
    }
}
