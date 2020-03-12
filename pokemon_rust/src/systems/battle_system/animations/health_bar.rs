use amethyst::{
    assets::Handle,
    ecs::Entity,
    renderer::{SpriteRender, SpriteSheet},
    ui::{Anchor, UiImage, UiText, UiTransform},
};

use crate::{
    battle::backend::Team,
    constants::{
        ALLY_HEALTH_BAR_HEIGHT,
        HEALTH_BAR_HORIZONTAL_PADDING,
        HEALTH_BAR_MARGIN,
        HEALTH_BAR_SMALLER_WIDTH,
        HEALTH_BAR_WIDTH,
        OPPONENT_HEALTH_BAR_HEIGHT,
        WINDOW_HEIGHT,
        WINDOW_WIDTH,
    },
    entities::pokemon::{get_all_pokemon_species, get_pokemon_display_name, Pokemon},
};

use super::super::BattleSystemData;

const P1_BAR_X: f32 = 0.;
const P2_BAR_X: f32 = WINDOW_WIDTH - HEALTH_BAR_WIDTH;

const P1_BAR_CONTENT_X: f32 = HEALTH_BAR_HORIZONTAL_PADDING;
const P2_BAR_CONTENT_X: f32 = WINDOW_WIDTH - HEALTH_BAR_SMALLER_WIDTH + HEALTH_BAR_HORIZONTAL_PADDING;

const P1_BAR_BOTTOM_Y: f32 = HEALTH_BAR_MARGIN;
const P2_BAR_BOTTOM_Y: f32 = WINDOW_HEIGHT - OPPONENT_HEALTH_BAR_HEIGHT - HEALTH_BAR_MARGIN;

const P1_BAR_TOP_Y: f32 = P1_BAR_BOTTOM_Y + ALLY_HEALTH_BAR_HEIGHT;
const P2_BAR_TOP_Y: f32 = P2_BAR_BOTTOM_Y + OPPONENT_HEALTH_BAR_HEIGHT;

pub struct HealthBarProperties {
    x: f32,
    content_x: f32,
    bottom_y: f32,
    top_y: f32,
    height: f32,
    width: f32,
}

pub struct HealthBar {
    container_entity: Entity,
    name_entity: Entity,
    // gender_entity: Entity,
    // level_entity: Entity,
    // health_bar_entity: Entity,
    // caught_indicator_entity: Option<Entity>,
    // health_values_entity: Option<Entity>,
    // experience_bar_entity: Option<Entity>,
}

impl HealthBar {
    pub fn new(pokemon: &Pokemon, team: Team, system_data: &mut BattleSystemData) -> Self {
        let properties = Self::get_properties(team);

        let container_entity = match team {
            Team::P1 => Self::create_left_container(&properties, system_data),
            Team::P2 => Self::create_right_container(&properties, system_data),
        };

        let name_entity = Self::create_name_entity(pokemon, &properties, system_data);
        // let gender_entity = Self::create_gender_entity(pokemon, system_data);
        // let level_entity = Self::create_level_entity(pokemon, system_data);
        // let health_bar_entity = Self::create_health_bar_entity(pokemon, system_data);
        // let health_values_entity = Self::create_health_values_entity(pokemon, team, system_data);
        // let experience_bar_entity = Self::create_experience_bar_entity(pokemon, team, system_data);

        Self {
            container_entity,
            name_entity,
            // gender_entity,
            // level_entity,
            // health_bar_entity,
            // health_values_entity,
            // experience_bar_entity,
        }
    }

    pub fn remove(&mut self, system_data: &mut BattleSystemData) {
        let entities = &system_data.entities;
        entities.delete(self.container_entity).expect("Failed to delete health container");
    }
}

impl HealthBar {
    fn get_properties(team: Team) -> HealthBarProperties {
        match team {
            Team::P1 => HealthBarProperties {
                x: P1_BAR_X,
                content_x: P1_BAR_CONTENT_X,
                bottom_y: P1_BAR_BOTTOM_Y,
                top_y: P1_BAR_TOP_Y,
                height: ALLY_HEALTH_BAR_HEIGHT,
                width: 220.,
            },
            Team::P2 => HealthBarProperties {
                x: P2_BAR_X,
                content_x: P2_BAR_CONTENT_X,
                bottom_y: P2_BAR_BOTTOM_Y,
                top_y: P2_BAR_TOP_Y,
                height: OPPONENT_HEALTH_BAR_HEIGHT,
                width: 220.,
            },
        }
    }

    fn create_left_container(
        properties: &HealthBarProperties,
        system_data: &mut BattleSystemData,
    ) -> Entity {
        Self::create_container(
            system_data.resources.hp_bar_left.clone(),
            &properties,
            system_data,
        )
    }

    fn create_right_container(
        properties: &HealthBarProperties,
        system_data: &mut BattleSystemData,
    ) -> Entity {
        Self::create_container(
            system_data.resources.hp_bar_right.clone(),
            &properties,
            system_data,
        )
    }

    fn create_container(
        sprite_sheet: Handle<SpriteSheet>,
        properties: &HealthBarProperties,
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
            Anchor::BottomLeft,
            Anchor::BottomLeft,
            properties.x,
            properties.bottom_y,
            2.,
            properties.width,
            properties.height,
        );

        entities
            .build_entity()
            .with(UiImage::Sprite(sprite_render), ui_images)
            .with(ui_transform, ui_transforms)
            .build()
    }
}

impl HealthBar {
    fn create_name_entity(
        pokemon: &Pokemon,
        properties: &HealthBarProperties,
        system_data: &mut BattleSystemData,
    ) -> Entity {
        let BattleSystemData {
            ui_texts,
            ui_transforms,
            entities,
            resources,
            ..
        } = system_data;

        let pokedex = get_all_pokemon_species();

        // TODO: extract to constant
        let font_size = 16.;

        let mut ui_text = UiText::new(
            resources.font.clone(),
            get_pokemon_display_name(&pokemon, pokedex).to_string(),
            [0., 0., 0., 1.],
            font_size,
        );
        ui_text.align = Anchor::TopLeft;

        let ui_transform = UiTransform::new(
            "Pokémon Display Name".to_string(),
            Anchor::BottomLeft,
            Anchor::BottomLeft,
            properties.content_x,
            properties.top_y - font_size,
            3.,
            200.,
            font_size,
        );

        entities
            .build_entity()
            .with(ui_text, ui_texts)
            .with(ui_transform, ui_transforms)
            .build()
    }
}
