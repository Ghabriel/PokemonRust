use amethyst::{
    assets::ProgressCounter,
    core::{math::Vector2, Transform},
    ecs::{Entity, world::Builder, World, WorldExt},
    renderer::SpriteRender,
    shrev::EventChannel,
    utils::application_root_dir,
};

use crate::{
    common::{get_direction_offset, load_full_texture_sprite_sheet},
    constants::{HALF_TILE_SIZE, MAP_DECORATION_LAYER_Z, MAP_TERRAIN_LAYER_Z, TILE_SIZE},
    entities::{
        player::PlayerEntity,
    },
    events::{EventQueue, TextEvent, WarpEvent},
};

use ron::de::from_reader;

use std::{
    collections::HashMap,
    fs::File,
};

use super::{
    events::ScriptEvent,
    map::{
        GameActionKind,
        GameScript,
        Map,
        MapConnection,
        MapScript,
        MapScriptKind,
        Tile,
    },
    MapCoordinates,
    MapHandler,
    MapId,
    PlayerCoordinates,
    serializable_map::SerializableMap,
    TileData,
    ValidatedGameAction,
    WorldCoordinates,
};

pub fn change_tile(
    starting_map_id: &MapId,
    final_tile_data: &TileData,
    map: &MapHandler,
    script_event_channel: &mut EventChannel<ScriptEvent>,
) {
    if *starting_map_id != final_tile_data.map_id {
        map.get_map_scripts(&final_tile_data, MapScriptKind::OnMapEnter)
            .for_each(|event| {
                script_event_channel.single_write(event);
            });
    }

    map.get_map_scripts(&final_tile_data, MapScriptKind::OnTileChange)
        .for_each(|event| {
            script_event_channel.single_write(event);
        });

    match map.get_action_at(&final_tile_data) {
        Some(
            ValidatedGameAction { when, script_event }
        ) if when == GameActionKind::OnStep => {
            script_event_channel.single_write(script_event);
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
    // TODO: obtain this value algorithmically
    let reference_point = WorldCoordinates::new(1_000_000, 0);

    let mut map = load_map(world, &map_name, Some(reference_point), progress_counter);

    if map_name == "test_map3" {
        println!("Loading scripts...");

        map.script_repository.push(GameScript::Native(|world| {
            let event = TextEvent::new(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
                world,
            );

            world.write_resource::<EventQueue>().push(event);
        }));

        map.script_repository.push(GameScript::Native(load_nearby_connections));

        map.script_repository.push(GameScript::Native(|world| {
            change_current_map(world, "test_map3".to_string());
        }));

        map.map_scripts.push(MapScript {
            when: MapScriptKind::OnTileChange,
            script_index: 1,
        });

        map.map_scripts.push(MapScript {
            when: MapScriptKind::OnMapEnter,
            script_index: 2,
        });
    }

    map
}

pub fn initialise_map(world: &mut World, progress_counter: &mut ProgressCounter) {
    let mut map = load_map(world, "test_map", None, progress_counter);

    map.script_repository.push(GameScript::Native(|world| {
        let event = TextEvent::new(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
            world,
        );

        world.write_resource::<EventQueue>().push(event);
    }));

    map.script_repository.push(GameScript::Native(load_nearby_connections));

    map.script_repository.push(GameScript::Native(|world| {
        change_current_map(world, "test_map".to_string());
    }));

    map.map_scripts.push(MapScript {
        when: MapScriptKind::OnTileChange,
        script_index: 1,
    });

    map.map_scripts.push(MapScript {
        when: MapScriptKind::OnMapEnter,
        script_index: 2,
    });

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
            let mut map = load_map(world, &connection.map, Some(reference_point), &mut progress_counter);

            if connection.map == "test_map2" {
                map.script_repository.push(GameScript::Native(|world| {
                    world
                        .write_resource::<EventQueue>()
                        .push(
                            WarpEvent::new("test_map3", MapCoordinates::new(5, 10))
                        );
                }));

                map.script_repository.push(GameScript::Native(|world| {
                    change_current_map(world, "test_map2".to_string());
                }));

                map.map_scripts.push(MapScript {
                    when: MapScriptKind::OnMapEnter,
                    script_index: map.script_repository.len() - 1,
                });
            }

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

    let tile_size = TILE_SIZE as i32;
    let (offset_x, offset_y) = get_direction_offset::<i32>(&first_direction);
    let external_tile_world_coordinates = WorldCoordinates::new(
        tile_world_coordinates.x() + tile_size * offset_x,
        tile_world_coordinates.y() + tile_size * offset_y,
    );
    let half_tile = HALF_TILE_SIZE as i32;

    WorldCoordinates::new(
        external_tile_world_coordinates.x() - half_tile - (external_tile.x() as i32) * tile_size,
        external_tile_world_coordinates.y() - half_tile - (external_tile.y() as i32) * tile_size,
    )
}

fn map_to_world_coordinates(
    tile: &MapCoordinates,
    reference_point: &WorldCoordinates,
) -> WorldCoordinates {
    let tile_size = TILE_SIZE as i32;
    let half_tile = HALF_TILE_SIZE as i32;

    WorldCoordinates::new(
        (tile.x() as i32) * tile_size + half_tile + reference_point.x(),
        (tile.y() as i32) * tile_size + half_tile + reference_point.y(),
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
    let map_data: SerializableMap = {
        let map_file = application_root_dir()
            .unwrap()
            .join("assets")
            .join("maps")
            .join(map_name)
            .join("map.ron");
        let file = File::open(map_file).expect("Failed opening map file");

        from_reader(file).expect("Failed deserializing map")
    };

    let SerializableMap {
        map_name,
        base_file_name,
        layer3_file_name,
        num_tiles_x,
        num_tiles_y,
        solids,
        actions,
        map_scripts,
        connections,
    } = map_data;

    let half_map = Vector2::new(
        (num_tiles_x as i32) * (HALF_TILE_SIZE as i32),
        (num_tiles_y as i32) * (HALF_TILE_SIZE as i32),
    );

    let (reference_point, map_center) = match reference_point {
        Some(reference_point) => {
            let center = WorldCoordinates(reference_point.0 + half_map);
            (reference_point, center)
        },
        None => (WorldCoordinates(-half_map), WorldCoordinates::new(0, 0)),
    };

    let map_size = (num_tiles_x * TILE_SIZE as u32, num_tiles_y * TILE_SIZE as u32);

    let terrain_entity = initialise_map_layer(
        world,
        MAP_TERRAIN_LAYER_Z,
        &base_file_name,
        &map_size,
        &map_center,
        progress_counter,
    );
    let decoration_entity = initialise_map_layer(
        world,
        MAP_DECORATION_LAYER_Z,
        &layer3_file_name,
        &map_size,
        &map_center,
        progress_counter,
    );

    Map {
        map_name,
        reference_point,
        terrain_entity,
        solids: solids
            .into_iter()
            .map(|tile_position| (MapCoordinates(tile_position), Tile))
            .collect(),
        decoration_entity,
        script_repository: Vec::new(),
        actions: actions
            .into_iter()
            .map(|(tile_position, action)| (MapCoordinates(tile_position), action))
            .collect(),
        map_scripts,
        connections: connections
            .into_iter()
            .map(|(tile_position, connection)| {
                (
                    MapCoordinates(tile_position),
                    MapConnection {
                        map: connection.map,
                        directions: connection.directions
                            .into_iter()
                            .map(|(direction, coordinates)| (direction, MapCoordinates(coordinates)))
                            .collect(),
                    },
                )
            })
            .collect(),
    }
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

    let mut transform = Transform::default();
    transform.set_translation_xyz(position.x() as f32, position.y() as f32, depth);

    world
        .create_entity()
        .with(transform)
        .with(sprite_render)
        .build()
}
