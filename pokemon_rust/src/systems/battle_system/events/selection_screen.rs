use amethyst::{
    assets::Handle,
    ecs::Entity,
    renderer::{SpriteRender, SpriteSheet},
    ui::{Anchor, UiImage, UiTransform},
};

use super::super::BattleSystemData;

const SELECTION_SCREEN_ARROW_HEIGHT: f32 = 37.;
const SELECTION_SCREEN_BUTTON_SCREEN_MARGIN: f32 = 10.;
const SELECTION_SCREEN_BUTTON_HEIGHT: f32 = 47.;

pub struct SelectionScreen {
    selection_arrow_entity: Entity,
    button_entities: Vec<Entity>,
    focused_option: u8,
}

impl SelectionScreen {
    pub fn new(
        button_width: f32,
        sprite_sheets: Vec<Handle<SpriteSheet>>,
        system_data: &mut BattleSystemData,
    ) -> Self {
        let num_options = sprite_sheets.len();
        let selection_arrow_entity = Self::create_selection_arrow(
            num_options,
            button_width,
            system_data,
        );

        let button_entities = sprite_sheets
            .into_iter()
            .enumerate()
            .map(|(index, sprite_sheet)| {
                Self::create_button(num_options, index, sprite_sheet, button_width, system_data)
            })
            .collect();

        Self {
            selection_arrow_entity,
            button_entities,
            focused_option: 0,
        }
    }

    pub fn get_focused_option(&self) -> u8 {
        self.focused_option
    }

    pub fn move_selection(&mut self, offset: i8, system_data: &mut BattleSystemData) {
        let num_options = self.button_entities.len() as u8;
        let scaled_option = ((self.focused_option + num_options) as i8 + offset) as u8;

        self.focused_option = scaled_option % num_options;
        self.update_selection_arrow(system_data);
    }

    pub fn remove(&mut self, system_data: &mut BattleSystemData) {
        let entities = &system_data.entities;
        entities.delete(self.selection_arrow_entity).expect("Failed to delete selection arrow");

        for button in &self.button_entities {
            entities.delete(*button).expect("Failed to delete button");
        }
    }

    fn create_selection_arrow(
        num_options: usize,
        button_width: f32,
        system_data: &mut BattleSystemData,
    ) -> Entity {
        Self::create_entity(
            system_data.resources.selection_arrow.clone(),
            "Selection Arrow".to_string(),
            -SELECTION_SCREEN_BUTTON_SCREEN_MARGIN - button_width,
            Self::get_selection_arrow_y(num_options, 0),
            32.,
            SELECTION_SCREEN_ARROW_HEIGHT,
            system_data,
        )
    }

    fn create_button(
        num_options: usize,
        index: usize,
        sprite_sheet: Handle<SpriteSheet>,
        button_width: f32,
        system_data: &mut BattleSystemData,
    ) -> Entity {
        Self::create_entity(
            sprite_sheet,
            format!("Selection Screen Button {}", index),
            -SELECTION_SCREEN_BUTTON_SCREEN_MARGIN,
            Self::get_button_bottom_y(num_options, index as u8),
            button_width,
            SELECTION_SCREEN_BUTTON_HEIGHT,
            system_data,
        )
    }

    fn create_entity(
        sprite_sheet: Handle<SpriteSheet>,
        id: String,
        right_margin: f32,
        down_margin: f32,
        width: f32,
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
            id,
            Anchor::BottomRight,
            Anchor::BottomRight,
            right_margin,
            down_margin,
            2.,
            width,
            height,
        );

        entities
            .build_entity()
            .with(UiImage::Sprite(sprite_render), ui_images)
            .with(ui_transform, ui_transforms)
            .build()
    }

    fn update_selection_arrow(&self, system_data: &mut BattleSystemData) {
        system_data.ui_transforms
            .get_mut(self.selection_arrow_entity)
            .expect("Failed to retrieve UiTransform")
            .local_y = Self::get_selection_arrow_y(self.button_entities.len(), self.focused_option);
    }

    fn get_selection_arrow_y(num_options: usize, focused_option: u8) -> f32 {
        let height_difference = SELECTION_SCREEN_BUTTON_HEIGHT - SELECTION_SCREEN_ARROW_HEIGHT;

        Self::get_button_bottom_y(num_options, focused_option) + height_difference / 2.
    }

    fn get_button_bottom_y(num_options: usize, option_index: u8) -> f32 {
        let option_index: usize = option_index.into();
        let inverted_option = (num_options - 1 - option_index) as f32;

        (SELECTION_SCREEN_BUTTON_SCREEN_MARGIN + SELECTION_SCREEN_BUTTON_HEIGHT) * inverted_option
        + SELECTION_SCREEN_BUTTON_SCREEN_MARGIN
    }
}
