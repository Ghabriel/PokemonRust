use amethyst::{
    assets::Handle,
    ecs::Entity,
    renderer::{SpriteRender, SpriteSheet},
    ui::{Anchor, UiImage, UiTransform},
};

use crate::{
    battle::backend::Team,
    constants::{ALLY_HEALTH_BAR_HEIGHT, HEALTH_BAR_MARGIN, OPPONENT_HEALTH_BAR_HEIGHT},
};

use super::super::BattleSystemData;

pub struct HealthBar {
    container_entity: Entity,
}

impl HealthBar {
    pub fn new(team: Team, system_data: &mut BattleSystemData) -> Self {
        let container_entity = match team {
            Team::P1 => Self::create_left_container(system_data),
            Team::P2 => Self::create_right_container(system_data),
        };

        Self {
            container_entity
        }
    }

    pub fn remove(&mut self, system_data: &mut BattleSystemData) {
        let entities = &system_data.entities;
        entities.delete(self.container_entity).expect("Failed to delete health container");
    }

    fn create_left_container(system_data: &mut BattleSystemData) -> Entity {
        Self::create_container(
            system_data.resources.hp_bar_left.clone(),
            Anchor::BottomLeft,
            Anchor::BottomLeft,
            HEALTH_BAR_MARGIN,
            ALLY_HEALTH_BAR_HEIGHT,
            system_data,
        )
    }

    fn create_right_container(system_data: &mut BattleSystemData) -> Entity {
        Self::create_container(
            system_data.resources.hp_bar_right.clone(),
            Anchor::TopRight,
            Anchor::TopRight,
            -HEALTH_BAR_MARGIN,
            OPPONENT_HEALTH_BAR_HEIGHT,
            system_data,
        )
    }

    fn create_container(
        sprite_sheet: Handle<SpriteSheet>,
        anchor: Anchor,
        pivot: Anchor,
        y: f32,
        height: f32,
        system_data: &mut BattleSystemData,
    ) -> Entity {
        let BattleSystemData {
            ui_images,
            ui_transforms,
            entities,
            ..
        } = system_data;

        let sprite_render = SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        };

        let ui_transform = UiTransform::new(
            "Health bar".to_string(),
            anchor,
            pivot,
            0.,
            y,
            2.,
            220.,
            height,
        );

        entities
            .build_entity()
            .with(UiImage::Sprite(sprite_render), ui_images)
            .with(ui_transform, ui_transforms)
            .build()
    }
}
