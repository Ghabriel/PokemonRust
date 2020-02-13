use amethyst::{
    assets::{Asset, AssetStorage, Loader},
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

const TEST_TRACK: &str = "bgm/littleroot-town.wav";

const SELECT_OPTION_SOUND: &str = "sfx/select_option.wav";

type AudioData = <Source as Asset>::Data;

#[derive(Default)]
pub struct Music {
    storage: HashMap<String, SourceHandle>,
    active_bgm: Option<Cycle<IntoIter<SourceHandle>>>,
}

impl Music {
    pub fn push_bgm(
        &mut self,
        bgm: String,
        format: AudioFileFormat,
        loader: &Loader,
        storage: &AssetStorage<Source>,
    ) {
        if !self.storage.contains_key(&bgm) {
            let handle = match format {
                AudioFileFormat::Flac => loader.load(bgm.clone(), FlacFormat, (), &storage),
                AudioFileFormat::Mp3 => loader.load(bgm.clone(), Mp3Format, (), &storage),
                AudioFileFormat::Ogg => loader.load(bgm.clone(), OggFormat, (), &storage),
                AudioFileFormat::Wav => loader.load(bgm.clone(), WavFormat, (), &storage),
            };
            self.storage.insert(bgm.clone(), handle);
        }

        let handle = self.storage.get(&bgm).unwrap().clone();
        self.active_bgm = Some(vec![handle].into_iter().cycle());
    }

    pub fn next(&mut self) -> Option<SourceHandle> {
        self.active_bgm.as_mut().and_then(|bgm| bgm.next())
    }
}

#[derive(Clone)]
pub enum AudioFileFormat {
    Flac,
    Mp3,
    Ogg,
    Wav,
}

#[derive(Eq, Hash, PartialEq)]
pub enum Sound {
    SelectOption,
}

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

pub struct SoundStorage {
    sounds: HashMap<Sound, SourceHandle>,
}

pub fn initialise_audio(world: &mut World) {
    let sound_storage = {
        let loader = world.read_resource::<Loader>();

        let mut sound_storage = SoundStorage { sounds: HashMap::new() };
        sound_storage.sounds.insert(
            Sound::SelectOption,
            loader.load(SELECT_OPTION_SOUND, WavFormat, (), &world.read_resource()),
        );

        sound_storage
    };

    world.insert(sound_storage);
    world.insert(Music::default());
}
