use amethyst::{
    core::Time,
    ecs::{
        Entity,
        Entities,
        Join,
        Read,
        ReadExpect,
        SystemData,
        world::EntitiesRes,
        World,
        WorldExt,
        WriteStorage,
    },
    ui::UiTransform,
};

use crate::{
    config::GameConfig,
    events::fade_out_event::Fade,
};

use super::{GameEvent, ShouldDisableInput};

#[derive(Default)]
pub struct FadeInEvent {
    top_fade: Option<Entity>,
    bottom_fade: Option<Entity>,
    elapsed_time: f32,
    completed: bool,
}

impl GameEvent for FadeInEvent {
    fn start(&mut self, world: &mut World) -> ShouldDisableInput {
        let entities = world.read_resource::<EntitiesRes>();
        let fades = world.read_component::<Fade>();

        for (entity, fade) in (&entities, &fades).join() {
            match fade.id {
                0 => self.top_fade = Some(entity),
                1 => self.bottom_fade = Some(entity),
                _ => panic!("Invalid Fade ID"),
            }
        }

        ShouldDisableInput(true)
    }

    fn tick(&mut self, world: &mut World, _disabled_inputs: bool) {
        let (mut ui_transforms, entities, game_config, time) = <(
            WriteStorage<UiTransform>,
            Entities,
            ReadExpect<GameConfig>,
            Read<Time>,
        )>::fetch(world);

        let fade_duration = game_config.fade_duration;
        let top_fade = self.top_fade.as_mut().unwrap();
        let bottom_fade = self.bottom_fade.as_mut().unwrap();

        self.elapsed_time += time.delta_seconds();

        ui_transforms
            .get_mut(*top_fade)
            .expect("Failed to retrieve UiTransform")
            .height = 300. * (1. - self.elapsed_time / fade_duration);

        ui_transforms
            .get_mut(*bottom_fade)
            .expect("Failed to retrieve UiTransform")
            .height = 300. * (1. - self.elapsed_time / fade_duration);

        if self.elapsed_time >= fade_duration {
            entities.delete(*top_fade).expect("Failed to delete top fade");
            entities.delete(*bottom_fade).expect("Failed to delete bottom fade");

            self.completed = true;
        }
    }

    fn is_complete(&self) -> bool {
        self.completed
    }
}