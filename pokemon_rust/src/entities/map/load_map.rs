use amethyst::{
    assets::ProgressCounter,
    core::Transform,
    ecs::{Entity, world::Builder, World, WorldExt},
    renderer::SpriteRender,
    utils::application_root_dir,
};

use crate::{
    common::load_full_texture_sprite_sheet,
    constants::{MAP_DECORATION_LAYER_Z, MAP_TERRAIN_LAYER_Z, TILE_SIZE},
    entities::{
        player::PlayerEntity,
    },
    events::EventQueue,
};

use ron::de::from_reader;

use std::{
    collections::HashMap,
    convert::TryFrom,
    fs::File,
};

use super::{
    conversions::{
        get_reference_point_from_tile,
        map_to_world_coordinates,
    },
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
        Map,
        MapConnection,
        MapScript,
        MapScriptKind,
    },
    MapHandler,
    MapId,
    serializable_map::{InitializedMap, SerializableMap},
    TileData,
    ValidatedGameAction,
};

pub fn change_tile(
    starting_map_id: &MapId,
    final_tile_data: &TileData,
    map: &mut MapHandler,
    event_queue: &mut EventQueue,
) {
    if *starting_map_id != final_tile_data.map_id {
        println!("Changing to map {}", final_tile_data.map_id.0);
        map.current_map = final_tile_data.map_id.clone();

        map.get_map_scripts(&final_tile_data.map_id, MapScriptKind::OnMapEnter)
            .for_each(|event| {
                event_queue.push(event);
            });
    }

    map.get_map_scripts(&final_tile_data.map_id, MapScriptKind::OnTileChange)
        .for_each(|event| {
            event_queue.push(event);
        });

    match map.get_action_at(&final_tile_data) {
        Some(
            ValidatedGameAction { when, script_event }
        ) if when == GameActionKind::OnStep => {
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
    world.read_resource::<MapHandler>()
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

pub fn initialise_map(world: &mut World, progress_counter: &mut ProgressCounter) {
    let map = load_map(world, "test_map", WorldCoordinates::origin(), progress_counter);

    let map_handler = MapHandler {
        loaded_maps: {
            let mut loaded_maps = HashMap::new();
            loaded_maps.insert("test_map".to_string(), map);
            loaded_maps
        },
        current_map: MapId("test_map".to_string()),
    };

    {
        let mut event_queue = world.write_resource::<EventQueue>();

        map_handler.get_map_scripts(&map_handler.current_map, MapScriptKind::OnMapEnter)
            .for_each(|event| {
                event_queue.push(event);
            });
    }

    world.insert(map_handler);
}

fn load_nearby_connections(world: &mut World) {
    let (nearby_connections, reference_point) = {
        let map = world.read_resource::<MapHandler>();
        let player_position = get_player_position(world);

        let mut nearby_connections: Vec<_> = map
            .get_nearby_connections(&player_position)
            .filter(|(_, connection)| !map.loaded_maps.contains_key(&connection.map))
            .map(|(tile, connection)| (tile.clone(), connection.clone()))
            .collect();

        nearby_connections.sort_by(|(_, lhs_connection), (_, rhs_connection)| {
            lhs_connection.map.cmp(&rhs_connection.map)
        });

        nearby_connections.dedup_by(|(_, lhs_connection), (_, rhs_connection)| {
            lhs_connection.map == rhs_connection.map
        });

        let reference_point = map
            .loaded_maps[&map.current_map.0]
            .reference_point
            .clone();

        (nearby_connections, reference_point)
    };

    let loaded_maps: Vec<_> = nearby_connections
        .iter()
        .map(|(tile, connection)| {
            println!("Loading map {}...", connection.map);
            let reference_point = get_new_map_reference_point(
                &tile,
                &connection,
                &reference_point,
            );
            // TODO: use the Progress trait to avoid needing to construct a ProgressCounter
            let mut progress_counter = ProgressCounter::new();
            let map = load_map(world, &connection.map, reference_point, &mut progress_counter);

            (connection.map.clone(), map)
        })
        .collect();

    world
        .write_resource::<MapHandler>()
        .loaded_maps
        .extend(loaded_maps);
}

fn get_player_position(world: &World) -> PlayerCoordinates {
    let player_entity = world.read_resource::<PlayerEntity>();

    world.read_storage::<Transform>()
        .get(player_entity.0)
        .map(PlayerCoordinates::from_transform)
        .expect("Failed to retrieve Transform")
}

fn get_new_map_reference_point(
    tile: &MapCoordinates,
    connection: &MapConnection,
    current_map_reference_point: &WorldCoordinates,
) -> WorldCoordinates {
    let tile_world_coordinates = map_to_world_coordinates(&tile, &current_map_reference_point);

    // TODO: handle multi-connections (non-rectangular maps)
    let (first_direction, external_tile) = connection.directions.iter().next().unwrap();

    let external_tile_world_coordinates = tile_world_coordinates
        .offset_by_direction(&first_direction);

    get_reference_point_from_tile(
        &external_tile,
        &external_tile_world_coordinates,
    )
}

fn load_map(
    world: &mut World,
    map_name: &str,
    reference_point: WorldCoordinates,
    progress_counter: &mut ProgressCounter,
) -> Map {
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
    map.script_repository.push(GameScript::Native(load_nearby_connections));

    map.map_scripts.push(MapScript {
        when: MapScriptKind::OnTileChange,
        script_index: map.script_repository.len() - 1,
    });

    map.script_repository.push(GameScript::Native(test));

    map.map_scripts.push(MapScript {
        when: MapScriptKind::OnMapEnter,
        script_index: map.script_repository.len() - 1,
    });
}

fn test(world: &mut World) {
    use crate::{
        common::Direction,
        entities::{
            npc::{Npc, NpcAction},
            player::PlayerSpriteSheets,
        },
    };

    let npc = Npc {
        action: NpcAction::Idle,
        facing_direction: Direction::Down,
        moving: false,
    };

    let transform = PlayerCoordinates::from_world_coordinates(&WorldCoordinates::new(30, 30))
        .to_transform();

    let sprite_render = {
        let sprite_sheets = world.read_resource::<PlayerSpriteSheets>();

        SpriteRender {
            sprite_sheet: sprite_sheets.walking.clone(),
            sprite_number: 0,
        }
    };

    world.register::<Npc>();

    world
        .create_entity()
        .with(npc)
        .with(transform)
        .with(sprite_render)
        .build();
}
