use crate::asteroid::core::prelude::*;
use crate::asteroid::physics::prelude::*;
use crate::{get, get_mut};
use bevy::prelude::*;

pub struct AsteroidDamagePlugin;

impl Plugin for AsteroidDamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                gameplay_collision_damage_system.run_if(on_event::<CollisionEvent>()),
                gameplay_death_system.after(gameplay_collision_damage_system),
                gameplay_collision_despawn_system
                    .run_if(on_event::<CollisionEvent>())
                    .run_if(any_with_component::<KillCollision>),
            )
                .after(AsteroidPhysicsSystem::FixedUpdateCollisionDetection)
                .in_set(AsteroidDamageSystem::FixedUpdateDamageSystem)
                .run_if(in_state(AsteroidGameState::Game)),
        )
        .add_systems(
            FixedPostUpdate,
            gameplay_despawn_dead_system
                .run_if(any_with_component::<Dead>)
                .in_set(AsteroidDamageSystem::FixedPostUpdateDeathSystem),
        );
    }
}

// Components

#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct DespawnIfDead;

#[derive(Component, Default)]
pub struct KillCollision;

pub trait Damager {
    fn get_damage(&self, health: &Health) -> i32;
}

#[derive(Component)]
pub struct CollisionDamager {
    damage_amount: i32,
}

impl Default for CollisionDamager {
    fn default() -> Self {
        Self { damage_amount: 10 }
    }
}

impl CollisionDamager {
    pub fn new(damage_amount: i32) -> Self {
        Self { damage_amount }
    }
}

impl Damager for CollisionDamager {
    fn get_damage(&self, _: &Health) -> i32 {
        self.damage_amount
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Dead;

#[derive(Component)]
pub struct Health {
    max_health: i32,
    current_health: i32,
}

impl Default for Health {
    fn default() -> Self {
        Self::new(100)
    }
}

impl Health {
    pub fn new(max_health: i32) -> Self {
        Self {
            max_health,
            current_health: max_health,
        }
    }

    #[inline(always)]
    pub fn get_max_health(&self) -> i32 {
        self.max_health
    }

    #[inline(always)]
    pub fn set_max_health(&mut self, new_max: i32) {
        self.max_health = new_max;
        self.current_health = self.max_health;
    }

    #[inline(always)]
    pub fn damage(&mut self, amount: i32) {
        self.current_health = (self.current_health - amount).clamp(0, self.max_health);
    }

    #[inline(always)]
    pub fn is_dead(&self) -> bool {
        self.current_health <= 0
    }
}

// Systems

#[derive(SystemSet, Hash, Eq, PartialEq, Clone, Debug)]
pub enum AsteroidDamageSystem {
    FixedUpdateDamageSystem,
    FixedPostUpdateDeathSystem,
}

fn gameplay_collision_despawn_system(
    mut commands: Commands,
    mut collision_event: EventReader<CollisionEvent>,
    query: Query<(), With<KillCollision>>,
) {
    for collision in collision_event.read() {
        handle_instant_kill(&mut commands, collision.first, &query);
        handle_instant_kill(&mut commands, collision.second, &query);
    }
}

fn handle_instant_kill(
    commands: &mut Commands,
    entity: Entity,
    query: &Query<(), With<KillCollision>>,
) {
    get!(_despawn, query, entity, return);
    commands.entity(entity).insert(Dead);
}

fn gameplay_collision_damage_system(
    mut collision_event: EventReader<CollisionEvent>,
    damager_query: Query<&CollisionDamager>,
    mut health_query: Query<&mut Health>,
) {
    for collision in collision_event.read() {
        handle_damage(
            collision.first,
            collision.second,
            &damager_query,
            &mut health_query,
        );
        handle_damage(
            collision.second,
            collision.first,
            &damager_query,
            &mut health_query,
        );
    }
}

fn handle_damage(
    first: Entity,
    second: Entity,
    damager_query: &Query<&CollisionDamager>,
    health_query: &mut Query<&mut Health>,
) {
    get!(damager, damager_query, first, return);
    get_mut!(mut health, health_query, second, return);

    let damage = damager.get_damage(&health);
    health.damage(damage);
}

fn gameplay_death_system(
    mut commands: Commands,
    health_query: Query<(Entity, &Health), Without<Dead>>,
) {
    for (entity, health) in &health_query {
        if health.is_dead() {
            commands.entity(entity).insert(Dead);
        }
    }
}

fn gameplay_despawn_dead_system(
    mut commands: Commands,
    dead_query: Query<Entity, (With<Dead>, With<DespawnIfDead>)>,
) {
    for entity in &dead_query {
        commands.entity(entity).despawn_recursive();
    }
}
