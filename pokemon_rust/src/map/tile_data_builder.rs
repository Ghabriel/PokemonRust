use amethyst::{
    core::Transform,
    ecs::{Entity, World, WorldExt},
};

use crate::entities::character::{CharacterId, PlayerEntity};

use super::{MapHandler, PlayerCoordinates, TileData};

#[derive(Default)]
struct KnownData {
    entity: Option<Entity>,
    character_id: Option<CharacterId>,
    player_coordinates: Option<PlayerCoordinates>,
}

#[derive(Default)]
pub struct TileDataBuilder(KnownData);

impl TileDataBuilder {
    pub fn with_entity(mut self, entity: Entity) -> PreparedTileDataBuilder {
        self.0.entity = Some(entity);
        PreparedTileDataBuilder(self.0)
    }

    pub fn with_character_id(mut self, character_id: CharacterId) -> PreparedTileDataBuilder {
        self.0.character_id = Some(character_id);
        PreparedTileDataBuilder(self.0)
    }

    #[allow(unused)]
    pub fn with_player_coordinates(mut self, player_coordinates: PlayerCoordinates) -> TileDataBuilder {
        self.0.player_coordinates = Some(player_coordinates);
        self
    }
}

pub struct PreparedTileDataBuilder(KnownData);

impl PreparedTileDataBuilder {
    #[allow(unused)]
    pub fn with_entity(mut self, entity: Entity) -> PreparedTileDataBuilder {
        self.0.entity = Some(entity);
        PreparedTileDataBuilder(self.0)
    }

    pub fn with_character_id(mut self, character_id: CharacterId) -> PreparedTileDataBuilder {
        self.0.character_id = Some(character_id);
        PreparedTileDataBuilder(self.0)
    }

    pub fn with_player_coordinates(mut self, player_coordinates: PlayerCoordinates) -> PreparedTileDataBuilder {
        self.0.player_coordinates = Some(player_coordinates);
        PreparedTileDataBuilder(self.0)
    }

    pub fn build(mut self, world: &mut World) -> TileData {
        self.set_player_coordinates(world);
        let map = world.read_resource::<MapHandler>();
        self.set_character_id(&map);

        let position = self.0.player_coordinates.unwrap();

        let map_id = {
            let character_id = self.0.character_id.unwrap();
            map.get_character_current_map(character_id).clone()
        };

        TileData { position, map_id }
    }

    fn set_entity(&mut self, world: &mut World) {
        if self.0.entity.is_none() {
            let entity = world.read_resource::<PlayerEntity>().0;
            self.0.entity = Some(entity);
        }
    }

    fn set_character_id(&mut self, map: &MapHandler) {
        if self.0.character_id.is_none() {
            // self.0.entity is always present here due to the type transitions
            let character_id = map.get_character_id_by_entity(self.0.entity.unwrap());
            self.0.character_id = Some(character_id);
        }
    }

    fn set_player_coordinates(&mut self, world: &mut World) {
        if self.0.player_coordinates.is_none() {
            self.set_entity(world);
            let entity = self.0.entity.unwrap();

            let player_coordinates = world
                .read_storage::<Transform>()
                .get(entity)
                .map(PlayerCoordinates::from_transform)
                .expect("Failed to retrieve Transform");

            self.0.player_coordinates = Some(player_coordinates);
        }
    }
}
