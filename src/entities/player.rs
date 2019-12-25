use amethyst::{
    assets::Handle,
    core::Transform,
    ecs::{
        Component,
        DenseVecStorage,
        Entity,
        world::Builder,
        World,
        WorldExt,
    },
    renderer::{SpriteRender, SpriteSheet},
};

pub struct Player {
    pub velocity: [f32; 2],
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialise_player(world: &mut World, sprite_sheet: Handle<SpriteSheet>) -> Entity {
    let player = Player {
        velocity: [0., 0.],
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(200., 300., 0.);

    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(player)
        .with(transform)
        .with(sprite_render)
        .build()
}
