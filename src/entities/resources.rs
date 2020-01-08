use amethyst::{
    assets::Loader,
    ecs::{World, WorldExt},
    ui::{FontHandle, TtfFormat},
};

pub struct Resources {
    pub font: FontHandle,
}

pub fn initialise_resources(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "fonts/arial.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    world.insert(Resources { font });
}
