use amethyst::{
    core::{
        Hidden,
        HiddenPropagate,
        math::{Point3, Vector3},
        Transform,
    },
    error::Error,
    ecs::{
        DispatcherBuilder,
        Entities,
        Entity,
        prelude::BitSet,
        Join,
        Read,
        ReadStorage,
        System,
        Write,
        World,
    },
    renderer::{
        Backend,
        bundle::{RenderOrder, RenderPlan, Target},
        camera::{ActiveCamera, Camera},
        Factory,
        pass::{DrawFlat2DDesc, DrawFlat2DTransparentDesc},
        RenderGroupDesc,
        RenderPlugin,
        Sprite,
        sprite_visibility::{SpriteVisibility, SpriteVisibilitySortingSystem},
        transparent::Transparent,
    },
};

use crate::entities::map::Tile;

use std::cmp::Ordering;

#[derive(Default, Debug)]
pub struct PokeRenderer {
    target: Target,
}

impl PokeRenderer {
    pub fn with_target(mut self, target: Target) -> Self {
        self.target = target;
        self
    }
}

impl<B: Backend> RenderPlugin<B> for PokeRenderer {
    fn on_build<'a, 'b>(
        &mut self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            PokeVisibility::new(),
            "sprite_visibility_system",
            &[],
        );
        // builder.add(
        //     SpriteVisibilitySortingSystem::new(),
        //     "sprite_visibility_system",
        //     &[],
        // );
        Ok(())
    }

    fn on_plan(
        &mut self,
        plan: &mut RenderPlan<B>,
        _factory: &mut Factory<B>,
        _world: &World,
    ) -> Result<(), Error> {
        plan.extend_target(self.target, |ctx| {
            ctx.add(RenderOrder::Opaque, DrawFlat2DDesc::new().builder())?;
            ctx.add(
                RenderOrder::Transparent,
                DrawFlat2DTransparentDesc::new().builder(),
            )?;
            Ok(())
        });
        Ok(())
    }
}

// fn get_tile_centroid(
//     entities: &Entities,
//     transforms: &ReadStorage<Transform>,
//     tiles: &ReadStorage<Tile>,
//     origin: &Point3<f32>,
//     camera_centroid: &Point3<f32>,
// ) -> Internals {
//     for (entity, transform, _) in (entities, transforms, tiles).join() {
//         let centroid = transform.global_matrix().transform_point(&origin);

//         return Internals {
//             entity,
//             transparent: false,
//             centroid,
//             camera_distance: (centroid.z - camera_centroid.z).abs(),
//             from_camera: centroid - camera_centroid,
//         };
//     }

//     panic!("No tiles");
// }

#[derive(Default)]
struct PokeVisibility {
    centroids: Vec<Internals>,
    transparent: Vec<Internals>,
}

#[derive(Debug, Clone)]
struct Internals {
    entity: Entity,
    transparent: bool,
    centroid: Point3<f32>,
    camera_distance: f32,
    from_camera: Vector3<f32>,
}

impl PokeVisibility {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'a> System<'a> for PokeVisibility {
    type SystemData = (
        Entities<'a>,
        Write<'a, SpriteVisibility>,
        ReadStorage<'a, Hidden>,
        ReadStorage<'a, HiddenPropagate>,
        Read<'a, ActiveCamera>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Transparent>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Tile>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut visibility,
            hidden,
            hidden_prop,
            active,
            camera,
            transparent,
            transform,
            tiles,
        ): Self::SystemData,
    ) {
        #[cfg(feature = "profiler")]
        profile_scope!("poke_visibility");

        let origin = Point3::origin();

        // The camera position is used to determine culling, but the sprites are ordered based on
        // the Z coordinate
        let camera: Option<&Transform> = active
            .entity
            .and_then(|a| transform.get(a))
            .or_else(|| (&camera, &transform).join().map(|ct| ct.1).next());
        let camera_backward = camera
            .map(|c| c.global_matrix().column(2).xyz())
            .unwrap_or_else(Vector3::z);
        let camera_centroid = camera
            .map(|t| t.global_matrix().transform_point(&origin))
            .unwrap_or_else(|| origin);

        self.centroids.clear();
        self.centroids.extend(
            (&*entities, &transform, !&hidden, !&hidden_prop, !&tiles)
                .join()
                .map(|(e, t, _, _, _)| (e, t.global_matrix().transform_point(&origin)))
                // filter entities behind the camera
                .filter(|(_, c)| (c - camera_centroid).dot(&camera_backward) < 0.0)
                .map(|(entity, centroid)| Internals {
                    entity,
                    transparent: transparent.contains(entity),
                    centroid,
                    camera_distance: (centroid.z - camera_centroid.z).abs(),
                    from_camera: centroid - camera_centroid,
                }),
        );

        // let tile_centroid = get_tile_centroid(
        //     &entities,
        //     &transform,
        //     &tiles,
        //     &origin,
        //     &camera_centroid,
        // );

        visibility.visible_unordered.clear();
        visibility.visible_unordered.extend(
            (&entities, &tiles)
                .join()
                .map(|(entity, _)| entity.id()),
        );
        visibility.visible_unordered.extend(
            self.centroids
                .iter()
                .filter(|c| !c.transparent)
                .map(|c| c.entity.id()),
        );

        self.transparent.clear();
        self.transparent
            .extend(self.centroids.drain(..).filter(|c| c.transparent));

        // Note: Smaller Z values are placed first, so that semi-transparent sprite colors blend
        // correctly.
        self.transparent.sort_by(|a, b| {
            b.camera_distance
                .partial_cmp(&a.camera_distance)
                .unwrap_or(Ordering::Equal)
        });

        visibility.visible_ordered.clear();
        visibility
            .visible_ordered
            .extend(self.transparent.iter().map(|c| c.entity));
    }
}


// #[derive(Default)]
// struct PokeVisibility {
//     subsystem: SpriteVisibilitySortingSystem,
// }

// impl<'a> System<'a> for PokeVisibility {
//     type SystemData = (
//         ReadStorage<'a, Tile>,
//         <SpriteVisibilitySortingSystem as System<'a>>::SystemData,
//     );

//     fn run(&mut self, (
//         tiles,
//         (entities, mut visibility, hidden, hidden_prop, active, camera, transparent, transform),
//     ): Self::SystemData) {
//         // TODO
//     }
// }
