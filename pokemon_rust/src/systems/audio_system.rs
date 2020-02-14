use amethyst::{
    assets::AssetStorage,
    audio::{AudioSink, output::Output, Source},
    ecs::{Read, ReadExpect, System, Write, WriteExpect},
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
        Option<Write<'a, AudioSink>>,
        Read<'a, Output>,
        ReadExpect<'a, GameConfig>,
        WriteExpect<'a, Music>,
    );

    fn run(&mut self, (storage, sink, output, game_config, mut music): Self::SystemData) {
        if game_config.play_bgm {
            if let Some(mut sink) = sink {
                match (sink.empty(), music.changed_bgm()) {
                    (true, _) => {
                        if let Some(source) = music.next().and_then(|h| storage.get(&h)) {
                            sink.append(source).unwrap();
                        }
                    },
                    (false, true) => {
                        sink.stop();
                        *sink = AudioSink::new(&output);

                        if let Some(source) = music.next().and_then(|h| storage.get(&h)) {
                            sink.append(source).unwrap();
                        }
                    },
                    (false, false) => { },
                }
            }
        }
    }
}

