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
    map: &MapHandler,
    event_queue: &mut EventQueue,
) {
    if *starting_map_id != final_tile_data.map_id {
        map.get_map_scripts(&final_tile_data, MapScriptKind::OnMapEnter)
            .for_each(|event| {
                event_queue.push(event);
            });
    }

    map.get_map_scripts(&final_tile_data, MapScriptKind::OnTileChange)
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

pub fn is_map_loaded(world: &mut World, map_name: &str) -> bool {
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

    load_map(world, &map_name, Some(reference_point), progress_counter)
}

pub fn initialise_map(world: &mut World, progress_counter: &mut ProgressCounter) {
    let map = load_map(world, "test_map", None, progress_counter);

    world.insert(MapHandler {
        loaded_maps: {
            let mut loaded_maps = HashMap::new();
            loaded_maps.insert("test_map".to_string(), map);
            loaded_maps
        },
        current_map: "test_map".to_string(),
    });
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
            .loaded_maps[&map.current_map]
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
            let map = load_map(world, &connection.map, Some(reference_point), &mut progress_counter);

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

fn change_current_map(world: &mut World, new_map: String) {
    println!("Changing to map {}", new_map);
    world
        .write_resource::<MapHandler>()
        .current_map = new_map;
}

fn load_map(
    world: &mut World,
    map_name: &str,
    reference_point: Option<WorldCoordinates>,
    progress_counter: &mut ProgressCounter,
) -> Map {
    let map = read_map_file(&map_name);
    let map_size = (map.num_tiles_x * TILE_SIZE as u32, map.num_tiles_y * TILE_SIZE as u32);
    let half_map_offset = WorldOffset::new(
        map_size.0 as i32 / 2,
        map_size.1 as i32 / 2,
    );

    let (reference_point, map_center) = match reference_point {
        Some(reference_point) => {
            let center = reference_point.with_offset(&half_map_offset);
            (reference_point, center)
        },
        None => (
            WorldCoordinates::origin().with_offset(&half_map_offset.invert()),
            WorldCoordinates::origin()
        ),
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

    // TODO: find a way to do this generically
    let change_map_script = match map.map_name.as_str() {
        "Test Map" => {
            GameScript::Native(|world| {
                change_current_map(world, "test_map".to_string());
            })
        },
        "Test Map 2" => {
            GameScript::Native(|world| {
                change_current_map(world, "test_map2".to_string());
            })
        },
        "Test Map 3" => {
            GameScript::Native(|world| {
                change_current_map(world, "test_map3".to_string());
            })
        },
        _ => panic!("Tried to add intrinsic scripts of non-supported map: {}", map.map_name),
    };

    map.script_repository.push(change_map_script);

    map.map_scripts.push(MapScript {
        when: MapScriptKind::OnMapEnter,
        script_index: map.script_repository.len() - 1,
    });
}
