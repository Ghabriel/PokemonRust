use amethyst::{
    assets::ProgressCounter,
    core::{math::{Vector2, Vector3}, Transform},
    ecs::{Entity, Join, world::Builder, World, WorldExt},
    renderer::SpriteRender,
    utils::application_root_dir,
};

use crate::{
    common::{get_direction_offset, load_sprite_sheet},
    constants::{HALF_TILE_SIZE, MAP_DECORATION_LAYER_Z, MAP_TERRAIN_LAYER_Z, TILE_SIZE},
    entities::{
        event_queue::{EventQueue, GameEvent},
        player::Player,
    },
};

use ron::de::from_reader;

use std::{
    collections::HashMap,
    fs::File,
};

use super::{
    map::{
        GameScript,
        Map,
        MapConnection,
        MapScript,
        MapScriptKind,
        Tile,
    },
    MapHandler,
    serializable_map::SerializableMap,
};

pub fn initialise_map(world: &mut World, progress_counter: &mut ProgressCounter) {
    let mut map = load_map(world, "test_map", None, progress_counter);

    map.script_repository.push(GameScript::Native(|world| {
        world
            .write_resource::<EventQueue>()
            .push(
                GameEvent::TextEvent(
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                    .to_string()
                )
            );
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

fn get_player_position(world: &World) -> Vector3<f32> {
    let players = world.read_storage::<Player>();
    let transforms = world.read_storage::<Transform>();

    (&players, &transforms).join()
        .map(|(_, transform)| transform.translation())
        .next()
        .unwrap()
        .clone()
}

fn get_new_map_reference_point(
    tile: &Vector2<u32>,
    connection: &MapConnection,
    current_map_reference_point: &Vector3<i32>,
) -> Vector3<i32> {
    let tile_size = TILE_SIZE as i32;
    let half_tile = HALF_TILE_SIZE as i32;
    let tile_world_coordinates = Vector2::new(
        (tile.x as i32) * tile_size + half_tile + current_map_reference_point.x,
        (tile.y as i32) * tile_size + half_tile + current_map_reference_point.y,
    );

    // TODO: handle multi-connections (non-rectangular maps)
    let (first_direction, external_tile) = connection.directions.iter().next().unwrap();

    let (offset_x, offset_y) = get_direction_offset::<i32>(&first_direction);
    let external_tile_offset = tile_size * Vector2::new(offset_x, offset_y);
    let external_tile_world_coordinates = tile_world_coordinates + external_tile_offset;

    Vector3::new(
        external_tile_world_coordinates.x - half_tile - (external_tile.x as i32) * tile_size,
        external_tile_world_coordinates.y - half_tile - (external_tile.y as i32) * tile_size,
        0,
    )
}

fn change_current_map(world: &mut World, new_map: String) {
    println!("Changing to map {}", new_map);
    world
        .write_resource::<MapHandler>()
        .current_map = new_map;
}

pub fn load_map(
    world: &mut World,
    map_name: &str,
    reference_point: Option<Vector3<i32>>,
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
        spritesheet_file_name,
        num_tiles_x,
        num_tiles_y,
        solids,
        actions,
        map_scripts,
        connections,
    } = map_data;

    let half_map = Vector3::new(
        (num_tiles_x as i32) * (HALF_TILE_SIZE as i32),
        (num_tiles_y as i32) * (HALF_TILE_SIZE as i32),
        0,
    );

    let (reference_point, map_center) = match reference_point {
        Some(reference_point) => (reference_point, reference_point + half_map),
        None => (-half_map, Vector3::new(0, 0, 0)),
    };

    let terrain_entity = initialise_map_layer(
        world,
        MAP_TERRAIN_LAYER_Z,
        &base_file_name,
        &spritesheet_file_name,
        &map_center,
        progress_counter,
    );
    let decoration_entity = initialise_map_layer(
        world,
        MAP_DECORATION_LAYER_Z,
        &layer3_file_name,
        &spritesheet_file_name,
        &map_center,
        progress_counter,
    );

    Map {
        map_name,
        reference_point,
        terrain_entity,
        solids: solids
            .into_iter()
            .map(|tile_position| (tile_position, Tile))
            .collect(),
        decoration_entity,
        script_repository: Vec::new(),
        actions,
        map_scripts,
        connections,
    }
}

fn initialise_map_layer(
    world: &mut World,
    depth: f32,
    image_name: &str,
    ron_name: &str,
    position: &Vector3<i32>,
    progress_counter: &mut ProgressCounter,
) -> Entity {
    let sprite_render = SpriteRender {
        sprite_sheet: load_sprite_sheet(world, &image_name, &ron_name, progress_counter),
        sprite_number: 0,
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(position.x as f32, position.y as f32, depth);

    world
        .create_entity()
        .with(transform)
        .with(sprite_render)
        .build()
}
