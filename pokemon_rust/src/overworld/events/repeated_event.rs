//! Generic event. Repeats an event for a given number of times sequentially.

use amethyst::ecs::World;

use super::{BoxedGameEvent, ChainedEvents, ExecutionConditions, GameEvent};

use std::marker::PhantomData;

pub struct RepeatedEvent<T>
where
    T: 'static + GameEvent + Sync + Send,
{
    chain: ChainedEvents,
    phantom_data: PhantomData<T>,
}

impl<T> RepeatedEvent<T>
where
    T: 'static + GameEvent + Sync + Send + Clone,
{
    pub fn from_prototype(prototype: &T, repetitions: usize) -> RepeatedEvent<T> {
        let mut chain = ChainedEvents::default();
        for _ in 0..repetitions {
            chain.add_event(Box::new(prototype.clone()));
        }

        RepeatedEvent {
            chain,
            phantom_data: PhantomData,
        }
    }
}

impl<T> Clone for RepeatedEvent<T>
where
    T: 'static + GameEvent + Sync + Send,
{
    fn clone(&self) -> RepeatedEvent<T> {
        RepeatedEvent::<T> {
            chain: self.chain.clone(),
            phantom_data: PhantomData,
        }
    }
}

impl<T> GameEvent for RepeatedEvent<T>
where
    T: 'static + GameEvent + Sync + Send,
{
    fn boxed_clone(&self) -> BoxedGameEvent {
        Box::new(RepeatedEvent::<T> {
            chain: self.chain.clone(),
            phantom_data: PhantomData,
        })
    }

    fn get_execution_conditions(&self) -> ExecutionConditions {
        self.chain.get_execution_conditions()
    }

    fn start(&mut self, world: &mut World) {
        self.chain.start(world);
    }

    fn tick(&mut self, world: &mut World, disabled_inputs: bool) {
        self.chain.tick(world, disabled_inputs);
    }

    fn is_complete(&self, world: &mut World) -> bool {
        self.chain.is_complete(world)
    }
}
