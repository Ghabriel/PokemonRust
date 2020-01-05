use amethyst::{
    assets::AssetStorage,
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
        ReadExpect,
        ReadStorage,
        System,
        Write,
        World,
        WorldExt,
    },
    renderer::{
        Backend,
        batch::{GroupIterator, OneLevelBatch, OrderedOneLevelBatch},
        bundle::{RenderOrder, RenderPlan, Target},
        camera::{ActiveCamera, Camera},
        Factory,
        pass::{DrawFlat2DTransparentDesc},
        pipeline::{PipelineDescBuilder, PipelinesBuilder},
        pod::SpriteArgs,
        RenderGroupDesc,
        RenderPlugin,
        rendy::{
            command::{QueueId, RenderPassEncoder},
            graph::{
                render::{PrepareResult, RenderGroup},
                GraphContext, NodeBuffer, NodeImage,
            },
            hal::{
                image::Layout::ShaderReadOnlyOptimal,
                pass::Subpass,
            },
        },
        resources::Tint,
        Sprite,
        SpriteRender,
        SpriteSheet,
        sprite_visibility::{SpriteVisibility, SpriteVisibilitySortingSystem},
        submodules::{DynamicVertexBuffer, FlatEnvironmentSub, TextureId, TextureSub},
        Texture,
        transparent::Transparent,
    },
};

use crate::entities::map::Tile;

use std::cmp::Ordering;

#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

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
            ctx.add(RenderOrder::Opaque, PokeDrawFlat2DDesc::new().builder())?;
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

// ----------------------------------------------
// TODO: move this to another file
// ----------------------------------------------

/// Draw opaque sprites without lighting.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PokeDrawFlat2DDesc;

impl PokeDrawFlat2DDesc {
    /// Create instance of `DrawFlat2D` render group
    pub fn new() -> Self {
        Default::default()
    }
}

impl<B: Backend> RenderGroupDesc<B, World> for PokeDrawFlat2DDesc {
    fn build(
        self,
        _ctx: &GraphContext<B>,
        factory: &mut Factory<B>,
        _queue: QueueId,
        _aux: &World,
        framebuffer_width: u32,
        framebuffer_height: u32,
        subpass: Subpass<'_, B>,
        _buffers: Vec<NodeBuffer>,
        _images: Vec<NodeImage>,
    ) -> Result<Box<dyn RenderGroup<B, World>>, failure::Error> {
        #[cfg(feature = "profiler")]
        profile_scope!("build");

        let env = FlatEnvironmentSub::new(factory)?;
        let textures = TextureSub::new(factory)?;
        let vertex = DynamicVertexBuffer::new();

        let (pipeline, pipeline_layout) = build_sprite_pipeline(
            factory,
            subpass,
            framebuffer_width,
            framebuffer_height,
            false,
            vec![env.raw_layout(), textures.raw_layout()],
        )?;

        Ok(Box::new(DrawFlat2D::<B> {
            pipeline,
            pipeline_layout,
            env,
            textures,
            vertex,
            sprites: Default::default(),
        }))
    }
}

/// Draws opaque 2D sprites to the screen without lighting.
#[derive(Debug)]
pub struct DrawFlat2D<B: Backend> {
    pipeline: B::GraphicsPipeline,
    pipeline_layout: B::PipelineLayout,
    env: FlatEnvironmentSub<B>,
    textures: TextureSub<B>,
    vertex: DynamicVertexBuffer<B, SpriteArgs>,
    sprites: OneLevelBatch<TextureId, SpriteArgs>,
}

impl<B: Backend> RenderGroup<B, World> for DrawFlat2D<B> {
    fn prepare(
        &mut self,
        factory: &Factory<B>,
        _queue: QueueId,
        index: usize,
        _subpass: Subpass<'_, B>,
        world: &World,
    ) -> PrepareResult {
        #[cfg(feature = "profiler")]
        profile_scope!("prepare opaque");

        let (
            sprite_sheet_storage,
            tex_storage,
            visibility,
            hiddens,
            hidden_props,
            sprite_renders,
            transforms,
            tints,
        ) = <(
            Read<'_, AssetStorage<SpriteSheet>>,
            Read<'_, AssetStorage<Texture>>,
            ReadExpect<'_, SpriteVisibility>,
            ReadStorage<'_, Hidden>,
            ReadStorage<'_, HiddenPropagate>,
            ReadStorage<'_, SpriteRender>,
            ReadStorage<'_, Transform>,
            ReadStorage<'_, Tint>,
        )>::fetch(world);

        self.env.process(factory, index, world);

        let sprites_ref = &mut self.sprites;
        let textures_ref = &mut self.textures;

        sprites_ref.clear_inner();

        {
            #[cfg(feature = "profiler")]
            profile_scope!("gather_visibility");

            (
                &sprite_renders,
                &transforms,
                tints.maybe(),
                &visibility.visible_unordered,
            )
                .join()
                .filter_map(|(sprite_render, global, tint, _)| {
                    let (batch_data, texture) = SpriteArgs::from_data(
                        &tex_storage,
                        &sprite_sheet_storage,
                        &sprite_render,
                        &global,
                        tint,
                    )?;
                    let (tex_id, _) = textures_ref.insert(
                        factory,
                        world,
                        texture,
                        ShaderReadOnlyOptimal,
                    )?;
                    Some((tex_id, batch_data))
                })
                .for_each_group(|tex_id, batch_data| {
                    sprites_ref.insert(tex_id, batch_data.drain(..))
                });
        }

        self.textures.maintain(factory, world);

        {
            #[cfg(feature = "profiler")]
            profile_scope!("write");

            sprites_ref.prune();
            self.vertex.write(
                factory,
                index,
                self.sprites.count() as u64,
                self.sprites.data(),
            );
        }

        PrepareResult::DrawRecord
    }

    fn draw_inline(
        &mut self,
        mut encoder: RenderPassEncoder<'_, B>,
        index: usize,
        _subpass: Subpass<'_, B>,
        _world: &World,
    ) {
        #[cfg(feature = "profiler")]
        profile_scope!("draw opaque");

        let layout = &self.pipeline_layout;
        encoder.bind_graphics_pipeline(&self.pipeline);
        self.env.bind(index, layout, 0, &mut encoder);
        self.vertex.bind(index, 0, 0, &mut encoder);
        for (&tex, range) in self.sprites.iter() {
            if self.textures.loaded(tex) {
                self.textures.bind(layout, 1, tex, &mut encoder);
                unsafe {
                    encoder.draw(0..4, range);
                }
            }
        }
    }

    fn dispose(self: Box<Self>, factory: &mut Factory<B>, _world: &World) {
        unsafe {
            factory.device().destroy_graphics_pipeline(self.pipeline);
            factory
                .device()
                .destroy_pipeline_layout(self.pipeline_layout);
        }
    }
}

// fn build_sprite_pipeline<B: Backend>(
//     factory: &Factory<B>,
//     subpass: hal::pass::Subpass<'_, B>,
//     framebuffer_width: u32,
//     framebuffer_height: u32,
//     transparent: bool,
//     layouts: Vec<&B::DescriptorSetLayout>,
// ) -> Result<(B::GraphicsPipeline, B::PipelineLayout), failure::Error> {
//     let pipeline_layout = unsafe {
//         factory
//             .device()
//             .create_pipeline_layout(layouts, None as Option<(_, _)>)
//     }?;

//     let shader_vertex = unsafe { super::SPRITE_VERTEX.module(factory).unwrap() };
//     let shader_fragment = unsafe { super::SPRITE_FRAGMENT.module(factory).unwrap() };

//     let pipes = PipelinesBuilder::new()
//         .with_pipeline(
//             PipelineDescBuilder::new()
//                 .with_vertex_desc(&[(SpriteArgs::vertex(), pso::VertexInputRate::Instance(1))])
//                 .with_input_assembler(pso::InputAssemblerDesc::new(hal::Primitive::TriangleStrip))
//                 .with_shaders(util::simple_shader_set(
//                     &shader_vertex,
//                     Some(&shader_fragment),
//                 ))
//                 .with_layout(&pipeline_layout)
//                 .with_subpass(subpass)
//                 .with_framebuffer_size(framebuffer_width, framebuffer_height)
//                 .with_blend_targets(vec![pso::ColorBlendDesc {
//                     mask: pso::ColorMask::ALL,
//                     blend: if transparent {
//                         Some(pso::BlendState::PREMULTIPLIED_ALPHA)
//                     } else {
//                         None
//                     },
//                 }])
//                 .with_depth_test(pso::DepthTest {
//                     fun: pso::Comparison::Less,
//                     write: !transparent,
//                 }),
//         )
//         .build(factory, None);

//     unsafe {
//         factory.destroy_shader_module(shader_vertex);
//         factory.destroy_shader_module(shader_fragment);
//     }

//     match pipes {
//         Err(e) => {
//             unsafe {
//                 factory.device().destroy_pipeline_layout(pipeline_layout);
//             }
//             Err(e)
//         }
//         Ok(mut pipes) => Ok((pipes.remove(0), pipeline_layout)),
//     }
// }

// ----------------------------------------------
// TODO: move this to another file
// ----------------------------------------------

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

        let a = std::time::Instant::now();

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

        let b = std::time::Instant::now();
        println!("Delay: {}", (b - a).as_millis());
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
