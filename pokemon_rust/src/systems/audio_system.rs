//! A system for background music playback. Reads the next BGM from
//! [`Music`](../../audio/struct.Music.html) and plays it whenever it changes
//! or ends. Affected by
//! [`GameConfig::play_bgm`](../../config/struct.GameConfig.html#structfield.play_bgm).

use amethyst::{
    assets::AssetStorage,
    audio::{AudioSink, output::Output, Source},
    ecs::{Read, ReadExpect, System, Write, WriteExpect},
};

use crate::{
    audio::Music,
    config::GameConfig,
};

/// A system for background music playback. Reads the next BGM from
/// [`Music`](../../audio/struct.Music.html) and plays it whenever it changes
/// or ends. Affected by
/// [`GameConfig::play_bgm`](../../config/struct.GameConfig.html#structfield.play_bgm).
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

