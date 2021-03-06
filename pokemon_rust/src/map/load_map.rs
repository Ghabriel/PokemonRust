use amethyst::{
    assets::ProgressCounter,
    ecs::{world::Builder, Entity, World, WorldExt},
    renderer::SpriteRender,
    utils::application_root_dir,
};

use crate::{
    common::{load_full_texture_sprite_sheet, AssetTracker},
    constants::{MAP_DECORATION_LAYER_Z, MAP_TERRAIN_LAYER_Z, TILE_SIZE},
    overworld::{
        entities::character::{CharacterId, PendingInteraction, PlayerEntity},
        events::{EventQueue, MapChangeEvent, ScriptEvent},
    },
};

use ron::de::from_reader;

use std::{collections::HashMap, convert::TryFrom, fs::File};

use super::{
    conversions::{get_reference_point_from_tile, map_to_world_coordinates},
    coordinates::{
        CoordinateSystem,
        MapCoordinates,
        PlayerCoordinates,
        WorldCoordinates,
        WorldOffset,
    },
    map::{
        GameActionKind,
        GameScript,
        GameScriptParameters,
        Map,
        MapConnection,
        MapScript,
        MapScriptKind,
    },
    serializable_map::{InitializedMap, SerializableMap},
    tile_data_builder::TileDataBuilder,
    MapHandler,
    MapId,
    TileData,
    ValidatedGameAction,
};

pub fn interact_with_npc(character_id: CharacterId, map_id: &MapId, event_queue: &mut EventQueue) {
    event_queue.push(ScriptEvent::from_script(GameScript::Lua {
        file: format!("assets/maps/{}/scripts.lua", map_id.0),
        function: "interact_with_npc".to_string(),
        parameters: Some(GameScriptParameters::TargetCharacter(character_id)),
    }));

    event_queue.push(ScriptEvent::from_script(GameScript::Native {
        script: |world, _| {
            world.remove::<PendingInteraction>();
        },
        parameters: None,
    }))
}

pub fn change_player_tile(
    initial_tile_data: &TileData,
    final_tile_data: &TileData,
    player_entity: &PlayerEntity,
    map: &mut MapHandler,
    event_queue: &mut EventQueue,
) {
    if initial_tile_data.map_id != final_tile_data.map_id {
        println!("Changing to map {}", final_tile_data.map_id.0);
        let current_map = map
            .characters
            .iter_mut()
            .find(|(_, c)| c.entity == player_entity.0)
            .map(|(_, c)| &mut c.current_map)
            .unwrap();
        *current_map = final_tile_data.map_id.clone();

        let natural_map = map
            .characters
            .iter_mut()
            .find(|(_, c)| c.entity == player_entity.0)
            .map(|(_, c)| &mut c.natural_map)
            .unwrap();
        *natural_map = final_tile_data.map_id.clone();

        map.get_map_scripts(&final_tile_data.map_id, MapScriptKind::OnMapEnter)
            .for_each(|event| {
                event_queue.push(event);
            });
    }

    // TODO: remove this after battles are properly connected to the overworld
    use crate::{battle::types::BattleType, overworld::events::BattleStartEvent};
    event_queue.push(BattleStartEvent::wild(BattleType::Single));

    map.get_map_scripts(&final_tile_data.map_id, MapScriptKind::OnTileChange)
        .for_each(|event| {
            event_queue.push(event);
        });

    match map.get_action_at(&final_tile_data) {
        Some(ValidatedGameAction { when, script_event }) if when == GameActionKind::OnStep => {
            event_queue.push(script_event);
        },
        _ => {},
    }
}

pub fn prepare_warp(
    world: &mut World,
    map_name: &str,
    tile: &MapCoordinates,
    progress_counter: &mut ProgressCounter,
) -> TileData {
    if !is_map_loaded(world, map_name) {
        let map = load_detached_map(world, &map_name, progress_counter);

        world
            .write_resource::<MapHandler>()
            .loaded_maps
            .insert(map_name.to_string(), map);
    }

    let map_handler = world.read_resource::<MapHandler>();
    let map = &map_handler.loaded_maps[map_name];
    let target_position = map_to_world_coordinates(&tile, &map.reference_point);

    TileData {
        position: PlayerCoordinates::from_world_coordinates(&target_position),
        map_id: MapId(map_name.to_string()),
    }
}

pub fn is_map_loaded(world: &World, map_name: &str) -> bool {
    world
        .read_resource::<MapHandler>()
        .loaded_maps
        .contains_key(map_name)
}

pub fn load_detached_map(
    world: &mut World,
    map_name: &str,
    progress_counter: &mut ProgressCounter,
) -> Map {
    let max_reference_point_x = world
        .read_resource::<MapHandler>()
        .loaded_maps
        .iter()
        .map(|(_, map)| map.reference_point.x())
        .max()
        .unwrap();

    let reference_point = WorldCoordinates::new(max_reference_point_x + 1_000_000, 0);

    load_map(world, &map_name, reference_point, progress_counter)
}

pub fn initialise_map(
    world: &mut World,
    starting_map: &str,
    progress_counter: &mut ProgressCounter,
) {
    let map = load_map(
        world,
        starting_map,
        WorldCoordinates::origin(),
        progress_counter,
    );

    let map_handler = MapHandler {
        loaded_maps: {
            let mut loaded_maps = HashMap::new();
            loaded_maps.insert(starting_map.to_string(), map);
            loaded_maps
        },
        next_character_id: 0,
        characters: HashMap::new(),
    };

    {
        let starting_map_id = MapId(starting_map.to_string());
        let mut event_queue = world.write_resource::<EventQueue>();

        map_handler
            .get_map_scripts(&starting_map_id, MapScriptKind::OnMapEnter)
            .for_each(|event| {
                event_queue.push(event);
            });
    }

    world.insert(map_handler);
}

fn load_nearby_connections(world: &mut World, progress_counter: &mut ProgressCounter) {
    let (nearby_connections, reference_point) = {
        let player_entity = world.read_resource::<PlayerEntity>().0;
        let character_id = world
            .read_resource::<MapHandler>()
            .get_character_id_by_entity(player_entity);

        let tile_data = TileDataBuilder::default()
            .with_character_id(character_id)
            .build(world);

        let map = world.read_resource::<MapHandler>();

        let mut nearby_connections: Vec<_> = map
            .get_nearby_connections(&tile_data)
            .filter(|(_, connection)| !map.loaded_maps.contains_key(&connection.map))
            .map(|(tile, connection)| (tile.clone(), connection.clone()))
            .collect();

        nearby_connections.sort_by(|(_, lhs_connection), (_, rhs_connection)| {
            lhs_connection.map.cmp(&rhs_connection.map)
        });

        nearby_connections.dedup_by(|(_, lhs_connection), (_, rhs_connection)| {
            lhs_connection.map == rhs_connection.map
        });

        let reference_point = map.loaded_maps[&map.get_character_current_map(character_id).0]
            .reference_point
            .clone();

        (nearby_connections, reference_point)
    };

    let loaded_maps: Vec<_> = nearby_connections
        .into_iter()
        .map(|(tile, connection)| {
            let reference_point = get_new_map_reference_point(&tile, &connection, &reference_point);
            let map = load_map(world, &connection.map, reference_point, progress_counter);

            (connection.map, map)
        })
        .collect();

    world
        .write_resource::<MapHandler>()
        .loaded_maps
        .extend(loaded_maps);
}

fn get_new_map_reference_point(
    tile: &MapCoordinates,
    connection: &MapConnection,
    current_map_reference_point: &WorldCoordinates,
) -> WorldCoordinates {
    let tile_world_coordinates = map_to_world_coordinates(&tile, &current_map_reference_point);

    // TODO: handle multi-connections (non-rectangular maps)
    let (first_direction, external_tile) = connection.directions.iter().next().unwrap();

    let external_tile_world_coordinates =
        tile_world_coordinates.offset_by_direction(&first_direction);

    get_reference_point_from_tile(&external_tile, &external_tile_world_coordinates)
}

fn load_map(
    world: &mut World,
    map_name: &str,
    reference_point: WorldCoordinates,
    progress_counter: &mut ProgressCounter,
) -> Map {
    println!("Loading map {}...", map_name);

    let map = read_map_file(&map_name);
    let tile_size: u32 = TILE_SIZE.into();
    let map_size = (map.num_tiles_x * tile_size, map.num_tiles_y * tile_size);

    let map_center = {
        let half_map_offset = WorldOffset::new(
            i32::try_from(map_size.0 / 2).unwrap(),
            i32::try_from(map_size.1 / 2).unwrap(),
        );

        reference_point.with_offset(&half_map_offset)
    };

    let terrain_entity = initialise_map_layer(
        world,
        MAP_TERRAIN_LAYER_Z,
        &map.base_file_name,
        &map_size,
        &map_center,
        progress_counter,
    );
    let decoration_entity = initialise_map_layer(
        world,
        MAP_DECORATION_LAYER_Z,
        &map.layer3_file_name,
        &map_size,
        &map_center,
        progress_counter,
    );

    let mut map = Map::from_initialized_map(InitializedMap {
        map_id: MapId(map_name.to_string()),
        map_name: map.map_name,
        reference_point,
        terrain_entity,
        solids: map.solids,
        decoration_entity,
        script_repository: map.script_repository,
        actions: map.actions,
        map_scripts: map.map_scripts,
        connections: map.connections,
    });

    add_intrinsic_scripts(&mut map);

    let mut event_queue = world.write_resource::<EventQueue>();

    map.get_map_scripts(MapScriptKind::OnMapLoad)
        .for_each(|event| {
            event_queue.push(event);
        });

    map
}

fn read_map_file(map_name: &str) -> SerializableMap {
    let map_file = application_root_dir()
        .unwrap()
        .join("assets")
        .join("maps")
        .join(map_name)
        .join("map.ron");

    let file = File::open(map_file).expect("Failed opening map file");

    from_reader(file).expect("Failed deserializing map")
}

fn initialise_map_layer(
    world: &mut World,
    depth: f32,
    image_name: &str,
    image_size: &(u32, u32),
    position: &WorldCoordinates,
    progress_counter: &mut ProgressCounter,
) -> Entity {
    let sprite_render = SpriteRender {
        sprite_sheet: load_full_texture_sprite_sheet(
            world,
            &image_name,
            &image_size,
            progress_counter,
        ),
        sprite_number: 0,
    };

    let mut transform = position.to_transform();
    transform.set_translation_z(depth);

    world
        .create_entity()
        .with(transform)
        .with(sprite_render)
        .build()
}

fn add_intrinsic_scripts(map: &mut Map) {
    map.script_repository.push(GameScript::Native {
        script: |world, _| {
            let mut asset_tracker = world.remove::<AssetTracker>().unwrap();
            load_nearby_connections(world, &mut asset_tracker.get_progress_counter_mut());
            world.insert(asset_tracker);
        },
        parameters: None,
    });

    map.map_scripts.push(MapScript {
        when: MapScriptKind::OnTileChange,
        script_index: map.script_repository.len() - 1,
    });

    map.script_repository.push(GameScript::Native {
        script: |world, params| {
            let map_name = match params {
                Some(GameScriptParameters::SourceMap(map_name)) => map_name,
                _ => unreachable!(),
            };

            world
                .write_resource::<EventQueue>()
                .push(MapChangeEvent::new(map_name));
        },
        parameters: Some(GameScriptParameters::SourceMap(map.map_name.clone())),
    });

    map.map_scripts.push(MapScript {
        when: MapScriptKind::OnMapEnter,
        script_index: map.script_repository.len() - 1,
    });
}
