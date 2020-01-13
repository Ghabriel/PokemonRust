use amethyst::{
    core::{math::{Vector2, Vector3}, Transform},
    ecs::{Component, DenseVecStorage, Entity, Join, world::Builder, World, WorldExt},
    renderer::SpriteRender,
    utils::application_root_dir,
};

use crate::{
    common::{Direction, load_sprite_sheet},
    constants::TILE_SIZE,
    entities::player::Player,
};

use ron::de::from_reader;

use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    fmt::{Debug, Error, Formatter},
    fs::File,
};

#[derive(Clone, Debug)]
pub enum MapEvent {
    Interaction,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SerializableMap {
    map_name: String,
    base_file_name: String,
    layer3_file_name: String,
    spritesheet_file_name: String,
    num_tiles_x: u32,
    num_tiles_y: u32,
    solids: Vec<Vector2<u32>>,
    actions: HashMap<Vector2<u32>, GameAction>,
    map_scripts: Vec<MapScript>,
    connections: HashMap<Vector2<u32>, MapConnection>,
}

// TODO: find a better name
pub struct MapHandler {
    loaded_maps: HashMap<String, Map>,
    current_map: String,
}

impl MapHandler {
    pub fn get_forward_tile(
        &self,
        player: &Player,
        player_position: &Transform,
    ) -> TileData {
        let (offset_x, offset_y) = match player.facing_direction {
            Direction::Up => (0., 1.),
            Direction::Down => (0., -1.),
            Direction::Left => (-1., 0.),
            Direction::Right => (1., 0.),
        };

        let tile_size = TILE_SIZE as f32;

        let current_map = &self.loaded_maps[&self.current_map];
        let current_tile = current_map.world_to_tile_coordinates(&player_position.translation());
        let connection = current_map.connections.get(&current_tile);
        let target_map = if let Some(connection) = connection {
            if connection.directions.contains_key(&player.facing_direction) {
                connection.map.clone()
            } else {
                self.current_map.clone()
            }
        } else {
            self.current_map.clone()
        };

        let position = player_position.translation() + Vector3::new(
            offset_x * tile_size,
            offset_y * tile_size,
            0.,
        );

        TileData {
            position,
            map_id: MapId(target_map),
        }
    }

    pub fn is_tile_blocked(&self, tile_data: &TileData) -> bool {
        self.loaded_maps[&tile_data.map_id.0]
            .is_tile_blocked(&tile_data.position)
    }

    pub fn get_action_at(&self, tile_data: &TileData) -> Option<ValidatedGameAction> {
        let map = &self.loaded_maps[&tile_data.map_id.0];
        let tile_coordinates = map.world_to_tile_coordinates(&tile_data.position);

        map.actions
            .get(&tile_coordinates)
            .map(|game_action| {
                ValidatedGameAction {
                    when: game_action.when.clone(),
                    script_event: ScriptEvent(tile_data.map_id.clone(), game_action.script_index)
                }
            })
    }

    pub fn get_script_from_event(&self, script_event: &ScriptEvent) -> &GameScript {
        let map = &self.loaded_maps[&(script_event.0).0];

        &map.script_repository[script_event.1]
    }

    pub fn get_map_scripts<'a>(
        &'a self,
        tile_data: &'a TileData,
        kind: MapScriptKind,
    ) -> impl Iterator<Item = ScriptEvent> + 'a {
        self.loaded_maps[&tile_data.map_id.0]
            .map_scripts
            .iter()
            .filter(move |script| script.when == kind)
            .map(move |script| ScriptEvent(tile_data.map_id.clone(), script.script_index))
    }

    pub fn get_nearby_connections(
        &self,
        position: &Vector3<f32>,
    ) -> impl Iterator<Item = (&Vector2<u32>, &MapConnection)> {
        let map = &self.loaded_maps[&self.current_map];
        let position = map.world_to_tile_coordinates(&position);

        map.connections
            .iter()
            .filter(move |(tile, connection)| {
                let visible_tiles_x = 22;
                let visible_tiles_y = 16;
                let distance_x = (tile.x as i32) - (position.x as i32);
                let distance_y = (tile.y as i32) - (position.y as i32);
                let leniency = 12;

                connection
                    .directions
                    .iter()
                    .all(|(direction, _)| match direction {
                        Direction::Up | Direction::Down => {
                            distance_y.abs() <= visible_tiles_y / 2 + leniency
                        },
                        Direction::Left | Direction::Right => {
                            distance_x.abs() <= visible_tiles_x / 2 + leniency
                        },
                    })
            })
    }
}

pub struct TileData {
    pub position: Vector3<f32>,
    pub map_id: MapId,
}

#[derive(Clone, Debug)]
pub struct MapId(String);

pub struct Map {
    map_name: String,
    bottom_left_corner: Vector3<i32>,
    num_tiles_x: u32,
    num_tiles_y: u32,
    terrain_entity: Entity,
    solids: HashMap<Vector2<u32>, Tile>,
    decoration_entity: Entity,
    pub script_repository: Vec<GameScript>,
    pub actions: HashMap<Vector2<u32>, GameAction>,
    map_scripts: Vec<MapScript>,
    connections: HashMap<Vector2<u32>, MapConnection>,
}

impl Component for Map {
    type Storage = DenseVecStorage<Self>;
}

impl Map {
    pub fn world_to_tile_coordinates(&self, position: &Vector3<f32>) -> Vector2<u32> {
        let position = Vector3::new(
            position.x as i32,
            position.y as i32,
            position.z as i32,
        );
        let tile_size = TILE_SIZE as i32;
        let half_tile = tile_size / 2;
        let target_corner = position - Vector3::new(half_tile, half_tile + 12, 0);
        let normalized_position = (target_corner - self.bottom_left_corner) / tile_size;

        Vector2::new(
            normalized_position.x as u32,
            normalized_position.y as u32,
        )
    }

    pub fn is_tile_blocked(&self, position: &Vector3<f32>) -> bool {
        let tile = self.world_to_tile_coordinates(&position);
        self.solids.contains_key(&tile)
    }
}

#[derive(Clone, Default)]
pub struct Tile;

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

// TODO: find a better name
pub struct ValidatedGameAction {
    pub when: GameActionKind,
    pub script_event: ScriptEvent,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameAction {
    pub when: GameActionKind,
    pub script_index: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum GameActionKind {
    /**
     * Triggered when the player presses Z on a tile.
     */
    OnInteraction,
    /**
     * Triggered after the player steps on a tile.
     */
    OnStep,
    /**
     * Triggered when the player tries to step on a tile,
     * _before_ actually stepping on it (e.g doors, sign posts).
     */
    OnStepAttempt,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MapScript {
    pub when: MapScriptKind,
    pub script_index: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum MapScriptKind {
    /**
     * Triggered when the player steps on a new tile.
     */
    OnTileChange,
}

#[derive(Clone)]
pub enum GameScript {
    Native(fn(&mut World) -> ()),
}

impl Debug for GameScript {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        match self {
            GameScript::Native(_) => write!(formatter, "Native Script"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MapConnection {
    map: String,
    directions: HashMap<Direction, Vector2<u32>>,
}

#[derive(Clone, Debug)]
pub struct ScriptEvent(MapId, usize);

pub fn initialise_map(world: &mut World) {
    let mut map = load_map(world, "test_map", None);

    map.script_repository.push(GameScript::Native(|world| {
        use amethyst::shrev::EventChannel;
        use crate::entities::text::TextEvent;

        world
            .write_resource::<EventChannel<TextEvent>>()
            .single_write(TextEvent::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."));
    }));

    map.script_repository.push(GameScript::Native(|world| {
        let (nearby_connections, bottom_left_corner) = {
            let map = world.read_resource::<MapHandler>();
            let players = world.read_storage::<Player>();
            let transforms = world.read_storage::<Transform>();

            let position = (&players, &transforms).join()
                .map(|(_, transform)| transform.translation())
                .next()
                .unwrap();

            let nearby_connections = map
                .get_nearby_connections(&position)
                .filter(|(_, connection)| !map.loaded_maps.contains_key(&connection.map))
                .map(|(tile, connection)| (tile.clone(), connection.clone()))
                .collect::<Vec<_>>();

            let bottom_left_corner = map
                .loaded_maps[&map.current_map]
                .bottom_left_corner
                .clone();

            (nearby_connections, bottom_left_corner)
        };

        let loaded_maps: Vec<_> = nearby_connections
            .iter()
            .map(|(tile, connection)| {
                let tile_size = TILE_SIZE as i32;
                let half_tile = (TILE_SIZE / 2) as i32;
                let tile_world_coordinates = Vector2::new(
                    (tile.x as i32) * tile_size + half_tile + bottom_left_corner.x,
                    (tile.y as i32) * tile_size + half_tile + bottom_left_corner.y,
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

                let map = load_map(world, &connection.map, Some(external_left_corner));
                (connection.map.clone(), map)
            })
            .collect();

        world
            .write_resource::<MapHandler>()
            .loaded_maps
            .extend(loaded_maps);
    }));

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
        num_tiles_x,
        num_tiles_y,
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
