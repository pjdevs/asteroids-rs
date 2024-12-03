use crate::asteroid::core::states::GameState;
use bevy::ecs::system::{EntityCommand, EntityCommands};
use bevy::prelude::*;
use std::marker::PhantomData;

pub struct InsertTimedComponent<T: Component> {
    component: T,
    duration: f32,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
struct TimedComponent<T: Component> {
    timer: Timer,
    _phantom: PhantomData<T>,
}

impl<T: Component> TimedComponent<T> {
    pub fn new(timer: Timer) -> Self {
        Self {
            timer,
            _phantom: PhantomData,
        }
    }
}

impl<T: Component> EntityCommand for InsertTimedComponent<T> {
    fn apply(self, id: Entity, world: &mut World) {
        world
            .entity_mut(id)
            .insert(self.component)
            .insert::<TimedComponent<T>>(TimedComponent::new(Timer::from_seconds(
                self.duration,
                TimerMode::Once,
            )));
    }
}

fn update_timed_components<T: Component>(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TimedComponent<T>)>,
) {
    for (entity, mut timed) in &mut query {
        timed.timer.tick(time.delta());

        if timed.timer.just_finished() {
            commands
                .entity(entity)
                .remove::<TimedComponent<T>>()
                .remove::<T>();
        }
    }
}

pub trait TimedEntityCommandsExt {
    fn insert_timed<T: Component>(&mut self, component: T, duration: f32) -> &mut Self;
}

impl<'a> TimedEntityCommandsExt for EntityCommands<'a> {
    fn insert_timed<T: Component>(&mut self, component: T, duration: f32) -> &mut Self {
        self.queue(InsertTimedComponent {
            component,
            duration,
        })
    }
}

pub trait TimedAppExt {
    fn register_timed_component<T: Component>(&mut self) -> &mut Self;
}

impl TimedAppExt for App {
    fn register_timed_component<T: Component>(&mut self) -> &mut Self {
        self.add_systems(
            Update,
            update_timed_components::<T>
                .run_if(any_with_component::<TimedComponent<T>>)
                .run_if(in_state(GameState::Game)),
        )
    }
}
