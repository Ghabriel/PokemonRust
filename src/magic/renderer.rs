use amethyst::{
    error::Error,
    ecs::{
        DispatcherBuilder,
        World,
    },
    renderer::{
        Backend,
        bundle::{RenderOrder, RenderPlan, Target},
        Factory,
        pass::{DrawFlat2DTransparentDesc},
        RenderGroupDesc,
        RenderPlugin,
    },
};

use super::{
    flat2d::PokeDrawFlat2DDesc,
    visibility::PokeVisibility,
};

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
