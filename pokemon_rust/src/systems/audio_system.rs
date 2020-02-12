use amethyst::{
    assets::AssetStorage,
    audio::{AudioSink, Source},
    ecs::{Read, ReadExpect, System, WriteExpect},
};

use crate::{
    audio::Music,
    config::GameConfig,
};

#[derive(Default)]
pub struct AudioSystem;

impl<'a> System<'a> for AudioSystem {
    type SystemData = (
        Read<'a, AssetStorage<Source>>,
        Option<Read<'a, AudioSink>>,
        ReadExpect<'a, GameConfig>,
        WriteExpect<'a, Music>,
    );

    fn run(&mut self, (storage, sink, game_config, mut music): Self::SystemData) {
        if game_config.play_bgm {
            if let Some(sink) = sink {
                if sink.empty() {
                    if let Some(source) = music.next().and_then(|h| storage.get(&h)) {
                        sink.append(source).unwrap();
                    }
                }
            }
        }
    }
}

