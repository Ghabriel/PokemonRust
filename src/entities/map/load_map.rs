use amethyst::{
    core::{math::{Vector2, Vector3}, Transform},
    ecs::{Entity, Join, world::Builder, World, WorldExt},
    renderer::SpriteRender,
    utils::application_root_dir,
};

use crate::{
    common::{Direction, load_sprite_sheet},
    constants::TILE_SIZE,
    entities::player::Player,
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

pub fn initialise_map(world: &mut World) {
    let mut map = load_map(world, "test_map", None);

    map.script_repository.push(GameScript::Native(|world| {
        use amethyst::shrev::EventChannel;
        use crate::entities::text::TextEvent;

        world
            .write_resource::<EventChannel<TextEvent>>()
            .single_write(TextEvent::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."));
    }));

    map.script_repository.push(GameScript::Native(load_nearby_connections));

    map.map_scripts.push(MapScript {
        when: MapScriptKind::OnTileChange,
        script_index: 1,
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
    let (nearby_connections, bottom_left_corner) = {
        let map = world.read_resource::<MapHandler>();
        let player_position = get_player_position(world);

        let nearby_connections: Vec<_> = map
            .get_nearby_connections(&player_position)
            .filter(|(_, connection)| !map.loaded_maps.contains_key(&connection.map))
            .map(|(tile, connection)| (tile.clone(), connection.clone()))
            .collect();

        let bottom_left_corner = map
            .loaded_maps[&map.current_map]
            .bottom_left_corner
            .clone();

        (nearby_connections, bottom_left_corner)
    };

    let loaded_maps: Vec<_> = nearby_connections
        .iter()
        .map(|(tile, connection)| {
            let bottom_left_corner = get_new_map_bottom_left_corner(
                &tile,
                &connection,
                &bottom_left_corner,
            );
            let map = load_map(world, &connection.map, Some(bottom_left_corner));
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

fn get_new_map_bottom_left_corner(
    tile: &Vector2<u32>,
    connection: &MapConnection,
    current_map_bottom_left_corner: &Vector3<i32>,
) -> Vector3<i32> {
    let tile_size = TILE_SIZE as i32;
    let half_tile = (TILE_SIZE / 2) as i32;
    let tile_world_coordinates = Vector2::new(
        (tile.x as i32) * tile_size + half_tile + current_map_bottom_left_corner.x,
        (tile.y as i32) * tile_size + half_tile + current_map_bottom_left_corner.y,
    );

    // TODO: handle multi-connections (non-rectangular maps)
    let (first_direction, external_tile) = connection.directions.iter().next().unwrap();

    let external_tile_offset = match first_direction {
        Direction::Up => Vector2::new(0, tile_size),
        Direction::Down => Vector2::new(0, -tile_size),
        Direction::Left => Vector2::new(-tile_size, 0),
        Direction::Right => Vector2::new(tile_size, 0),
    };

    let external_tile_world_coordinates = tile_world_coordinates + external_tile_offset;
    let external_left_corner = Vector3::new(
        external_tile_world_coordinates.x - half_tile - (external_tile.x as i32) * tile_size,
        external_tile_world_coordinates.y - half_tile - (external_tile.y as i32) * tile_size,
        0,
    );

    println!("Loading map {}...", connection.map);
    println!("Connection tile (map coordinates): {:?}", tile);
    println!("Connection tile (world coordinates): {:?}", tile_world_coordinates);
    println!("External tile (world coordinates): {:?}", external_tile_world_coordinates);
    println!("External left corner: {:?}", external_left_corner);

    external_left_corner
}

pub fn load_map(world: &mut World, map_name: &str, bottom_left_corner: Option<Vector3<i32>>) -> Map {
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

    let bottom_left_corner = bottom_left_corner.unwrap_or(
        Vector3::new(
            -(num_tiles_x as i32) * ((TILE_SIZE / 2) as i32),
            -(num_tiles_y as i32) * ((TILE_SIZE / 2) as i32),
            0,
        )
    );

    let map_center = bottom_left_corner + Vector3::new(
        (num_tiles_x as i32) * ((TILE_SIZE / 2) as i32),
        (num_tiles_y as i32) * ((TILE_SIZE / 2) as i32),
        0,
    );

    let terrain_entity = initialise_map_layer(
        world,
        -1.,
        &base_file_name,
        &spritesheet_file_name,
        &map_center,
    );
    let decoration_entity = initialise_map_layer(
        world,
        0.5,
        &layer3_file_name,
        &spritesheet_file_name,
        &map_center,
    );

    Map {
        map_name,
        bottom_left_corner,
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
) -> Entity {
    let sprite_render = SpriteRender {
        sprite_sheet: load_sprite_sheet(world, &image_name, &ron_name),
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
