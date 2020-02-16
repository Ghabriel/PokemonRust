//! Types related to BGM/SFX playback.

use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{
        FlacFormat,
        Mp3Format,
        OggFormat,
        Source,
        SourceHandle,
        output::Output,
        WavFormat,
    },
    ecs::{
        Read,
        ReadExpect,
        SystemData,
        World,
        WorldExt,
    },
    shred::ResourceId,
};

use crate::config::GameConfig;

use std::{
    collections::HashMap,
    iter::Cycle,
    vec::IntoIter,
};

/// A type responsible for controlling the current background music (BGM).
/// BGMs are played in a loop as long as they aren't changed.
/// Internally, it contains a storage with every loaded BGM file. Files can
/// also be preloaded to be played later.
#[derive(Default)]
pub struct Music {
    storage: HashMap<String, SourceHandle>,
    changed_bgm: bool,
    active_bgm: Option<Cycle<IntoIter<SourceHandle>>>,
}

impl Music {
    /// Sets a BGM file as the active BGM.
    pub fn play_bgm(
        &mut self,
        bgm: String,
        format: AudioFileFormat,
        loader: &Loader,
        storage: &AssetStorage<Source>,
    ) {
        self.preload_bgm(bgm.clone(), format, loader, storage);

        let handle = self.storage.get(&bgm).unwrap().clone();
        self.changed_bgm = true;
        self.active_bgm = Some(vec![handle].into_iter().cycle());
    }

    /// Preloads a BGM for future playback.
    pub fn preload_bgm(
        &mut self,
        bgm: String,
        format: AudioFileFormat,
        loader: &Loader,
        storage: &AssetStorage<Source>,
    ) {
        if !self.storage.contains_key(&bgm) {
            println!("Preloading {}", bgm);
            let handle = match format {
                AudioFileFormat::Flac => loader.load(bgm.clone(), FlacFormat, (), &storage),
                AudioFileFormat::Mp3 => loader.load(bgm.clone(), Mp3Format, (), &storage),
                AudioFileFormat::Ogg => loader.load(bgm.clone(), OggFormat, (), &storage),
                AudioFileFormat::Wav => loader.load(bgm.clone(), WavFormat, (), &storage),
            };
            self.storage.insert(bgm, handle);
        }
    }

    /// Checks if the active BGM has been changed since the last call to `next()`.
    pub fn changed_bgm(&self) -> bool {
        self.changed_bgm
    }

    /// Returns the active BGM, if any.
    pub fn next(&mut self) -> Option<SourceHandle> {
        self.changed_bgm = false;
        self.active_bgm.as_mut().and_then(|bgm| bgm.next())
    }
}

/// An enumeration with all supported audio file formats.
#[derive(Clone)]
pub enum AudioFileFormat {
    Flac,
    Mp3,
    Ogg,
    Wav,
}

/// An enumeration with the possible sound effects of the game.
#[derive(Eq, Hash, PartialEq)]
pub enum Sound {
    SelectOption,
}

impl Sound {
    fn get_filename(&self) -> &str {
        match self {
            Sound::SelectOption => "sfx/select_option.wav",
        }
    }
}

/// A type which implements `SystemData` that can be used to play sound effects
/// (SFX) easily. This can be retrieved either via the `SystemData` of a System
/// or directly from the world through `SoundKit::fetch(world)`.
pub struct SoundKit<'a> {
    asset_storage: Read<'a, AssetStorage<Source>>,
    sound_storage: ReadExpect<'a, SoundStorage>,
    output: Option<Read<'a, Output>>,
    game_config: ReadExpect<'a, GameConfig>,
}

impl<'a> SoundKit<'a> {
    pub fn play_sound(&self, sound: Sound) {
        if self.game_config.play_sfx {
            let handle = self.sound_storage.sounds.get(&sound).unwrap();
            match (self.asset_storage.get(&handle), &self.output) {
                (Some(sound), Some(output)) => output.play_once(sound, 1.0),
                _ => {},
            }
        }
    }
}

impl<'a> SystemData<'a> for SoundKit<'a> {
    fn setup(world: &mut World) {
        <Read<'a, AssetStorage<Source>> as SystemData>::setup(world);
        <ReadExpect<'a, SoundStorage> as SystemData>::setup(world);
        <Option<Read<'a, Output>> as SystemData>::setup(world);
        <ReadExpect<'a, GameConfig> as SystemData>::setup(world);
    }

    fn fetch(world: &'a World) -> Self {
        SoundKit {
            asset_storage: <Read<'a, AssetStorage<Source>> as SystemData<'a>>::fetch(world),
            sound_storage: <ReadExpect<'a, SoundStorage> as SystemData<'a>>::fetch(world),
            output: <Option<Read<'a, Output>> as SystemData<'a>>::fetch(world),
            game_config: <ReadExpect<'a, GameConfig> as SystemData<'a>>::fetch(world),
        }
    }

    fn reads() -> Vec<ResourceId> {
        let mut r = Vec::new();

        r.append(&mut <Read<'a, AssetStorage<Source>> as SystemData>::reads());
        r.append(&mut <ReadExpect<'a, SoundStorage> as SystemData>::reads());
        r.append(&mut <Option<Read<'a, Output>> as SystemData>::reads());
        r.append(&mut <ReadExpect<'a, GameConfig> as SystemData>::reads());

        r
    }

    fn writes() -> Vec<ResourceId> {
        let mut r = Vec::new();

        r.append(&mut <Read<'a, AssetStorage<Source>> as SystemData>::writes());
        r.append(&mut <ReadExpect<'a, SoundStorage> as SystemData>::writes());
        r.append(&mut <Option<Read<'a, Output>> as SystemData>::writes());
        r.append(&mut <ReadExpect<'a, GameConfig> as SystemData>::writes());

        r
    }
}

/// A private storage for loaded sounds (SFX). This is an implementation detail
/// of `SoundKit`.
struct SoundStorage {
    sounds: HashMap<Sound, SourceHandle>,
}

/// Initialisation function for the audio module. Inserts a `SoundStorage` and
/// a `Music` into the world as resources.
pub fn initialise_audio(world: &mut World) {
    let sound_storage = {
        let loader = world.read_resource::<Loader>();

        let mut sound_storage = SoundStorage { sounds: HashMap::new() };
        sound_storage.sounds.insert(
            Sound::SelectOption,
            loader.load(Sound::SelectOption.get_filename(), WavFormat, (), &world.read_resource()),
        );

        sound_storage
    };

    world.insert(sound_storage);
    world.insert(Music::default());
}
