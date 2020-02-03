use amethyst::{
    core::Time,
    ecs::{Join, Read, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::entities::AnimationTable;

use std::{
    hash::Hash,
    marker::PhantomData,
};

pub struct AnimationSystem<T: 'static + Eq + Hash + Sync + Send> {
    phantom_data: PhantomData<T>,
}

impl<T> AnimationSystem<T>
where
    T: 'static + Eq + Hash + Sync + Send
{
    pub fn new() -> AnimationSystem<T> {
        AnimationSystem {
            phantom_data: PhantomData,
        }
    }
}

impl<'a, T> System<'a> for AnimationSystem<T>
where
    T: 'static + Eq + Hash + Sync + Send
{
    type SystemData = (
        WriteStorage<'a, AnimationTable<T>>,
        WriteStorage<'a, SpriteRender>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut animation_tables, mut sprite_renders, time): Self::SystemData) {
        for (animation_table, sprite_render) in (&mut animation_tables, &mut sprite_renders).join() {
            if let Some(animation) = &animation_table.active_animation {
                let animation_data = animation_table.get(&animation).unwrap();
                let mut timing = animation_table.timing;
                let frame_index = animation_data.timings
                    .binary_search_by(|value| value.partial_cmp(&timing).unwrap());
                let mut frame_index = match frame_index {
                    Ok(index) => index,
                    Err(index) => index,
                };

                if frame_index >= animation_data.timings.len() {
                    timing -= animation_data.timings.last().unwrap();
                    frame_index = 0;
                }

                sprite_render.sprite_number = animation_data.frames[frame_index];
                timing += time.delta_seconds();
                animation_table.timing = timing;
            }
        }
    }
}

