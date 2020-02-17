//! Changes the current background music.

use amethyst::{
    assets::{AssetStorage, Loader},
    audio::Source,
    ecs::{World, WorldExt},
};

use crate::audio::{AudioFileFormat, Music};

use super::{BoxedGameEvent, ExecutionConditions, GameEvent};

#[derive(Clone)]
pub struct BgmChangeEvent {
    filename: String,
    format: AudioFileFormat,
}

impl BgmChangeEvent {
    pub fn new(filename: impl Into<String>, format: AudioFileFormat) -> BgmChangeEvent {
        BgmChangeEvent {
            filename: filename.into(),
            format,
        }
    }
}

impl GameEvent for BgmChangeEvent {
    fn boxed_clone(&self) -> BoxedGameEvent {
        Box::new(self.clone())
    }

    fn get_execution_conditions(&self) -> ExecutionConditions {
        ExecutionConditions {
            requires_disabled_input: false,
        }
    }

    fn start(&mut self, _world: &mut World) {}

    fn tick(&mut self, world: &mut World, _disabled_inputs: bool) {
        let mut music = world.write_resource::<Music>();
        let filename = format!("bgm/{}", self.filename);
        let format = self.format.clone();
        let loader = world.read_resource::<Loader>();
        let storage = world.read_resource::<AssetStorage<Source>>();

        music.play_bgm(filename, format, &loader, &storage);
    }

    fn is_complete(&self, _world: &mut World) -> bool {
        true
    }
}
