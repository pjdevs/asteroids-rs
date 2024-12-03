use crate::asteroid::core::prelude::*;
use crate::asteroid::physics::prelude::*;
use crate::asteroid::utils::prelude::*;
use crate::{get, get_mut};
use bevy::prelude::*;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                gameplay_collision_damage_system,
                gameplay_collision_despawn_system.run_if(any_with_component::<DespawnOnCollision>),
            )
                .run_if(on_event::<CollisionEvent>)
                .run_if(in_state(GameState::Game))
                .in_set(DamageSystem::FixedUpdateDamageSystem),
        )
        .add_systems(
            FixedPostUpdate,
            gameplay_despawn_dead_system
                .run_if(any_with_component::<Dead>)
                .in_set(DamageSystem::FixedPostUpdateDeathSystem),
        );
    }
}

// Components

#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct DespawnOnDead;

#[derive(Component, Default)]
pub struct DespawnOnCollision;

pub enum Damager {
    Constant(i32),
    Kill,
}

impl Default for Damager {
    fn default() -> Self {
        Self::Constant(10)
    }
}

impl Damager {
    fn get_damage(&self, health: &Health) -> i32 {
        match self {
            Damager::Constant(damage) => *damage,
            Damager::Kill => health.max_health,
        }
    }
}

// We could use dynamic dispatch and heap allocation with a Box<dyn> to be fully flexible
// but there will not be much cases so prefer more idiomatic ECS ways

#[derive(Component, Default)]
pub struct CollisionDamager {
    damager: Damager,
}

impl From<Damager> for CollisionDamager {
    fn from(damager: Damager) -> Self {
        Self { damager }
    }
}

// TODO Think about this :
// - Make a DamagedModifier and implement Invincibility, Shield etc with it ?
// - Or keep Invincibilty as a component to be able to directly tell if invincible
//   and react to start/end of Invincibility
// Advantage of individual components for game changing statuses is that
// that we can react to add/remove easily for sound, vfx, etc
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Invincibility;

// Try an enum with this one for better perfs with static typing/type safety
// while allowing extensibility and flexibilty
#[derive(Component)]
#[component(storage = "SparseSet")]
pub enum DamageModifier {
    Multiply(f32),
    Add(f32),
    ReduceToZero,
}

impl DamageModifier {
    pub fn apply(&self, damage: i32) -> i32 {
        let damage = damage as f32;
        let modified_damage = match self {
            DamageModifier::Multiply(factor) => damage * factor,
            DamageModifier::Add(amount) => damage + amount,
            DamageModifier::ReduceToZero => 0.0,
        };

        modified_damage as i32
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
pub enum DamageSystem {
    FixedUpdateDamageSystem,
    FixedPostUpdateDeathSystem,
}

fn gameplay_collision_despawn_system(
    mut commands: Commands,
    mut collision_event: EventReader<CollisionEvent>,
    query: Query<(), With<DespawnOnCollision>>,
) {
    for collision in collision_event.read() {
        handle_collision_despawn(&mut commands, collision.first, &query);
        handle_collision_despawn(&mut commands, collision.second, &query);
    }
}

fn handle_collision_despawn(
    commands: &mut Commands,
    entity: Entity,
    query: &Query<(), With<DespawnOnCollision>>,
) {
    get!(_despawn, query, entity, return);
    commands.entity(entity).ensure_despawned();
}

fn gameplay_collision_damage_system(
    mut commands: Commands,
    mut collision_event: EventReader<CollisionEvent>,
    damager_query: Query<(&CollisionDamager, Option<&DamageModifier>)>,
    mut health_query: Query<(&mut Health, Option<&Invincibility>)>,
) {
    for collision in collision_event.read() {
        handle_collision_damage(
            &mut commands,
            collision.first,
            collision.second,
            &damager_query,
            &mut health_query,
        );
        handle_collision_damage(
            &mut commands,
            collision.second,
            collision.first,
            &damager_query,
            &mut health_query,
        );
    }
}

fn handle_collision_damage(
    commands: &mut Commands,
    damager: Entity,
    damaged: Entity,
    damager_query: &Query<(&CollisionDamager, Option<&DamageModifier>)>,
    damaged_query: &mut Query<(&mut Health, Option<&Invincibility>)>,
) {
    get!(
        (collision_damager, damage_modifier),
        damager_query,
        damager,
        return
    );
    get_mut!((mut health, invincibility), damaged_query, damaged, return);

    do_damage(
        commands,
        &collision_damager.damager,
        &damage_modifier,
        damaged,
        &mut health,
        &invincibility,
    );
}

/// Aplly damages to health and return the final damage amount applied.
fn do_damage(
    commands: &mut Commands,
    damager: &Damager,
    damage_modifier: &Option<&DamageModifier>,
    damaged_entity: Entity,
    damaged_health: &mut Health,
    invincibility: &Option<&Invincibility>,
) -> i32 {
    if invincibility.is_some() {
        return 0;
    }

    let base_damage = damager.get_damage(&damaged_health);
    let modified_damage = if let Some(modifier) = damage_modifier {
        modifier.apply(base_damage) as i32
    } else {
        base_damage
    };

    damaged_health.damage(modified_damage);

    if damaged_health.is_dead() {
        commands.entity(damaged_entity).insert(Dead);
    }

    modified_damage
}

fn gameplay_despawn_dead_system(
    mut commands: Commands,
    dead_query: Query<Entity, (With<Dead>, With<DespawnOnDead>)>,
) {
    for entity in &dead_query {
        commands.entity(entity).despawn_recursive();
    }
}
