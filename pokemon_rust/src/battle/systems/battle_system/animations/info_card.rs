use amethyst::{
    assets::Handle,
    ecs::Entity,
    renderer::{palette::Srgba, resources::Tint, SpriteRender, SpriteSheet},
    ui::{Anchor, UiImage, UiText, UiTransform},
};

use crate::{
    battle::backend::Team,
    constants::{
        ALLY_HEALTH_BAR_HEIGHT,
        BAR_HEIGHT,
        BAR_SPACING,
        BAR_WIDTH,
        HEALTH_BAR_HORIZONTAL_PADDING,
        HEALTH_BAR_MARGIN,
        HEALTH_BAR_POKEMON_HEALTH_TEXT_FONT_SIZE,
        HEALTH_BAR_POKEMON_LEVEL_FONT_SIZE,
        HEALTH_BAR_POKEMON_NAME_FONT_SIZE,
        HEALTH_BAR_SMALLER_WIDTH,
        HEALTH_BAR_WIDTH,
        OPPONENT_HEALTH_BAR_HEIGHT,
        WINDOW_HEIGHT,
        WINDOW_WIDTH,
    },
    pokemon::{get_all_pokemon_species, get_pokemon_display_name, Pokemon},
};

use super::super::BattleSystemData;

const P1_BAR_X: f32 = 0.;
const P2_BAR_X: f32 = WINDOW_WIDTH - HEALTH_BAR_WIDTH;

const P1_BAR_CONTENT_X: f32 = HEALTH_BAR_HORIZONTAL_PADDING;
const P2_BAR_CONTENT_X: f32 = WINDOW_WIDTH - HEALTH_BAR_SMALLER_WIDTH + HEALTH_BAR_HORIZONTAL_PADDING;

const P1_BAR_CONTENT_END_X: f32 = P1_BAR_CONTENT_X + BAR_WIDTH;
const P2_BAR_CONTENT_END_X: f32 = P2_BAR_CONTENT_X + BAR_WIDTH;

const P1_BAR_BOTTOM_Y: f32 = HEALTH_BAR_MARGIN;
const P2_BAR_BOTTOM_Y: f32 = WINDOW_HEIGHT - OPPONENT_HEALTH_BAR_HEIGHT - HEALTH_BAR_MARGIN;

const P1_BAR_TOP_Y: f32 = P1_BAR_BOTTOM_Y + ALLY_HEALTH_BAR_HEIGHT;
const P2_BAR_TOP_Y: f32 = P2_BAR_BOTTOM_Y + OPPONENT_HEALTH_BAR_HEIGHT;

pub struct HealthBarProperties {
    x: f32,
    content_x: f32,
    content_end_x: f32,
    bottom_y: f32,
    top_y: f32,
    height: f32,
    width: f32,
}

pub struct InfoCard {
    container_entity: Entity,
    name_entity: Entity,
    // gender_entity: Entity,
    level_entity: Entity,
    health_bar_support_entity: Entity,
    health_bar_entity: Entity,
    // caught_indicator_entity: Option<Entity>,
    health_values_entity: Option<Entity>,
    // experience_bar_entity: Option<Entity>,
}

impl InfoCard {
    pub fn new(pokemon: &Pokemon, team: Team, system_data: &mut BattleSystemData) -> Self {
        let properties = Self::get_properties(team);

        let container_entity = match team {
            Team::P1 => Self::create_left_container(&properties, system_data),
            Team::P2 => Self::create_right_container(&properties, system_data),
        };

        let name_entity = Self::create_name_entity(&pokemon, &properties, system_data);
        // let gender_entity = Self::create_gender_entity(pokemon, system_data);
        let level_entity = Self::create_level_entity(&pokemon, &properties, system_data);
        let (health_bar_support_entity, health_bar_entity) = Self::create_health_bar_entities(
            &properties,
            system_data,
        );
        let health_values_entity = Self::create_health_values_entity(
            &pokemon,
            team,
            &properties,
            system_data,
        );
        // let experience_bar_entity = Self::create_experience_bar_entity(pokemon, team, system_data);

        Self {
            container_entity,
            name_entity,
            // gender_entity,
            level_entity,
            health_bar_support_entity,
            health_bar_entity,
            health_values_entity,
            // experience_bar_entity,
        }
    }

    pub fn remove(&mut self, system_data: &mut BattleSystemData) {
        let entities = &system_data.entities;
        entities.delete(self.container_entity).expect("Failed to delete info card container");
        entities.delete(self.name_entity).expect("Failed to delete name container");
        entities.delete(self.level_entity).expect("Failed to delete level container");
        entities.delete(self.health_bar_entity).expect("Failed to delete health bar container");

        if let Some(health_values_entity) = self.health_values_entity {
            entities.delete(health_values_entity).expect("Failed to delete health values container");
        }
    }

    pub fn damage(
        &mut self,
        _amount: usize,
        pokemon: &Pokemon,
        system_data: &mut BattleSystemData,
    ) {
        let BattleSystemData {
            ui_texts,
            ui_transforms,
            ..
        } = system_data;

        let health_rate = (pokemon.current_hp as f32) / (pokemon.stats[0] as f32);

        ui_transforms
            .get_mut(self.health_bar_entity)
            .expect("Failed to retrieve UiTransform")
            .width = (BAR_WIDTH - 2.) * health_rate;

        if let Some(health_values_entity) = self.health_values_entity {
            let content = format!("{} / {}", pokemon.current_hp, pokemon.stats[0]);

            ui_texts
                .get_mut(health_values_entity)
                .expect("Failed to retrieve UiText")
                .text = content;
        }
    }
}

impl InfoCard {
    fn get_properties(team: Team) -> HealthBarProperties {
        match team {
            Team::P1 => HealthBarProperties {
                x: P1_BAR_X,
                content_x: P1_BAR_CONTENT_X,
                content_end_x: P1_BAR_CONTENT_END_X,
                bottom_y: P1_BAR_BOTTOM_Y,
                top_y: P1_BAR_TOP_Y,
                height: ALLY_HEALTH_BAR_HEIGHT,
                width: HEALTH_BAR_WIDTH,
            },
            Team::P2 => HealthBarProperties {
                x: P2_BAR_X,
                content_x: P2_BAR_CONTENT_X,
                content_end_x: P2_BAR_CONTENT_END_X,
                bottom_y: P2_BAR_BOTTOM_Y,
                top_y: P2_BAR_TOP_Y,
                height: OPPONENT_HEALTH_BAR_HEIGHT,
                width: HEALTH_BAR_WIDTH,
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
            "Health bar container".to_string(),
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

impl InfoCard {
    fn create_name_entity(
        pokemon: &Pokemon,
        properties: &HealthBarProperties,
        system_data: &mut BattleSystemData,
    ) -> Entity {
        let pokedex = get_all_pokemon_species();

        Self::create_ui_text(
            get_pokemon_display_name(&pokemon, pokedex).to_string(),
            HEALTH_BAR_POKEMON_NAME_FONT_SIZE,
            properties.content_x,
            properties.top_y - HEALTH_BAR_POKEMON_NAME_FONT_SIZE,
            system_data,
        )
    }

    fn create_level_entity(
        pokemon: &Pokemon,
        properties: &HealthBarProperties,
        system_data: &mut BattleSystemData,
    ) -> Entity {
        let content = format!("Lv. {}", pokemon.level);
        let font_size = HEALTH_BAR_POKEMON_LEVEL_FONT_SIZE;
        let content_width = Self::estimate_text_width(&content, font_size);

        let x = properties.content_end_x - content_width;

        Self::create_ui_text(
            content,
            font_size,
            x,
            properties.top_y - font_size,
            system_data,
        )
    }

    fn create_health_bar_entities(
        properties: &HealthBarProperties,
        system_data: &mut BattleSystemData,
    ) -> (Entity, Entity) {
        let support_bar = Self::create_generic_health_bar_entity(
            0.,
            0.,
            0.,
            0.,
            0.,
            Tint(Srgba::new(0.2, 0.2, 0.2, 1.0)),
            properties,
            system_data,
        );

        let live_bar = Self::create_generic_health_bar_entity(
            1.,
            1.,
            1.,
            -2.,
            -2.,
            Tint(Srgba::new(0.0, 1.0, 0.0, 1.0)),
            properties,
            system_data,
        );

        (support_bar, live_bar)
    }

    fn create_generic_health_bar_entity(
        delta_x: f32,
        delta_y: f32,
        delta_z: f32,
        delta_width: f32,
        delta_height: f32,
        tint: Tint,
        properties: &HealthBarProperties,
        system_data: &mut BattleSystemData,
    ) -> Entity {
        let BattleSystemData {
            tints,
            ui_images,
            ui_transforms,
            entities,
            resources,
            ..
        } = system_data;

        let sprite_render = SpriteRender {
            sprite_sheet: resources.white.clone(),
            sprite_number: 0,
        };

        let ui_transform = UiTransform::new(
            "Health bar".to_string(),
            Anchor::BottomLeft,
            Anchor::BottomLeft,
            properties.content_x + delta_x,
            properties.top_y - HEALTH_BAR_POKEMON_LEVEL_FONT_SIZE - BAR_HEIGHT - BAR_SPACING + delta_y,
            3. + delta_z,
            BAR_WIDTH + delta_width,
            BAR_HEIGHT + delta_height,
        );

        entities
            .build_entity()
            .with(UiImage::Sprite(sprite_render), ui_images)
            .with(ui_transform, ui_transforms)
            .with(tint, tints)
            .build()
    }

    fn create_health_values_entity(
        pokemon: &Pokemon,
        team: Team,
        properties: &HealthBarProperties,
        system_data: &mut BattleSystemData,
    ) -> Option<Entity> {
        if team == Team::P2 {
            return None;
        }

        let content = format!("{} / {}", pokemon.current_hp, pokemon.stats[0]);

        Some(Self::create_ui_text(
            content,
            HEALTH_BAR_POKEMON_HEALTH_TEXT_FONT_SIZE,
            properties.content_x,
            properties.bottom_y,
            system_data,
        ))
    }

    fn create_ui_text(
        content: String,
        font_size: f32,
        x: f32,
        y: f32,
        system_data: &mut BattleSystemData
    ) -> Entity {
        let BattleSystemData {
            ui_texts,
            ui_transforms,
            entities,
            resources,
            ..
        } = system_data;

        let content_width = Self::estimate_text_width(&content, font_size);

        let mut ui_text = UiText::new(
            resources.font.clone(),
            content,
            [0., 0., 0., 1.],
            font_size,
        );
        ui_text.align = Anchor::TopLeft;

        let ui_transform = UiTransform::new(
            "Text".to_string(),
            Anchor::BottomLeft,
            Anchor::BottomLeft,
            x,
            y,
            3.,
            content_width,
            font_size,
        );

        entities
            .build_entity()
            .with(ui_text, ui_texts)
            .with(ui_transform, ui_transforms)
            .build()
    }

    fn estimate_text_width(text: &str, font_size: f32) -> f32 {
        // TODO: improve this estimate
        text.len() as f32 * font_size / 2.
    }
}
