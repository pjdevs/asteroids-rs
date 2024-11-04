use crate::asset;
use crate::asteroid::core::prelude::*;
use crate::asteroid::physics::prelude::*;
use crate::asteroid::utils::prelude::*;
use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;
use bevy_asset_loader::prelude::*;
use rand::Rng;
use std::marker::PhantomData;
use std::time::Duration;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct AsteroidSpawner<M: Component> {
    pub enabled: bool,
    #[reflect(ignore)]
    pub spawner_asset: Handle<SpawnerAsset>,
    #[reflect(ignore)]
    pub _spawned_type: PhantomData<M>,
}

impl<M: Component> AsteroidSpawner<M> {
    pub fn from_asset(spawner_asset: Handle<SpawnerAsset>) -> Self {
        Self {
            enabled: true,
            spawner_asset,
            _spawned_type: PhantomData,
        }
    }
}

pub trait SpawnerAppExt {
    fn add_spawner<M: Component>(
        &mut self,
        loading_state: impl FreelyMutableState,
        spawning_state: impl FreelyMutableState,
        make_spawnable_system: impl IntoSystem<(), Entity, ()>,
        set: impl SystemSet,
    ) -> &mut Self
    where
        AsteroidSpawner<M>: FromWorld;
}

impl SpawnerAppExt for App {
    fn add_spawner<M: Component>(
        &mut self,
        loading_state: impl FreelyMutableState,
        spawning_state: impl FreelyMutableState,
        make_spawnable_system: impl IntoSystem<(), Entity, ()>,
        set: impl SystemSet,
    ) -> &mut Self
    where
        AsteroidSpawner<M>: FromWorld,
    {
        self.configure_loading_state(
            LoadingStateConfig::new(loading_state).init_resource::<AsteroidSpawner<M>>(),
        )
        .add_systems(
            Update,
            make_spawnable_system.pipe(spawner_system::<M>).run_if(
                in_state(spawning_state.clone())
                    .and_then(spawner_enabled::<M>)
                    .and_then(on_spawn_timer::<M>()),
            ).in_set(set),
        )
        .add_systems(
            OnExit(spawning_state),
            remove_resource::<AsteroidSpawner<M>>,
        )
    }
}

fn spawner_enabled<M: Component>(spawner: Res<AsteroidSpawner<M>>) -> bool {
    spawner.enabled
}

pub fn spawner_system<M: Component>(
    In(entity): In<Entity>,
    mut commands: Commands,
    spawner: Res<AsteroidSpawner<M>>,
    spawner_assets: Res<Assets<SpawnerAsset>>,
    camera_query: Query<&Camera>,
) {
    let spawner_asset = asset!(spawner_assets, &spawner.spawner_asset);
    let camera = camera_query.single();
    let mut random = rand::thread_rng();

    let min_max_angle = spawner_asset.min_max_angle * std::f32::consts::PI;
    let random_angle = random.gen_range(min_max_angle.x..=min_max_angle.y);
    let random_speed =
        random.gen_range(spawner_asset.min_max_speed.x..=spawner_asset.min_max_speed.y);
    let random_velocity = Vec2::new(random_angle.cos(), random_angle.sin()) * random_speed;
    let random_angular_velocity = random
        .gen_range(spawner_asset.min_max_angular_speed.x..=spawner_asset.min_max_angular_speed.y);
    let screen_size = camera.physical_target_size().unwrap();
    let half_screen_size = Vec2::new(screen_size.x as f32 / 2.0, screen_size.y as f32 / 2.0);
    let random_position =
        2.0 * half_screen_size * Vec2::from(random.gen::<(f32, f32)>()).round() - half_screen_size;
    let random_scale =
        random.gen_range(spawner_asset.min_max_scale.x..=spawner_asset.min_max_scale.y);

    commands.entity(entity).add(move |mut e: EntityWorldMut| {
        if let Some(mut movement) = e.get_mut::<Movement>() {
            movement.position = random_position;
            movement.velocity = random_velocity;
            movement.angular_velocity = random_angular_velocity;
        }

        // TODO Make a Size/Scale component or include it into movement?
        if let Some(sprite) = e.get_mut::<Sprite>() {
            if let Some(mut size) = sprite.custom_size {
                size *= random_scale;
            }
        };

        if let Some(mut collider) = e.get_mut::<Collider>() {
            collider.shape = collider.shape.scaled(random_scale);
        };
    });
}

fn on_spawn_timer<M: Component>(
) -> impl FnMut(Res<Time>, Res<AsteroidSpawner<M>>, Res<Assets<SpawnerAsset>>) -> bool + Clone {
    let mut timer = Timer::new(Duration::ZERO, TimerMode::Repeating);

    move |time: Res<Time>,
          spawner: Res<AsteroidSpawner<M>>,
          spawner_assets: Res<Assets<SpawnerAsset>>| {
        if spawner.is_changed() {
            let spawner_asset = asset!(spawner_assets, &spawner.spawner_asset);
            timer.set_duration(Duration::from_millis(spawner_asset.spawn_delay_ms));
        }

        timer.tick(time.delta());
        timer.just_finished()
    }
}
