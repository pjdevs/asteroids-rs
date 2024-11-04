use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Asset, TypePath)]
pub struct SizeAsset {
    pub sprite_size: Vec2,
    pub collider_size: Vec2,
}

#[derive(Deserialize, Asset, Reflect)]
pub struct SpawnerAsset {
    pub spawn_delay_ms: u64,
    pub max_entity_count: usize,
    pub min_max_speed: Vec2,
    pub min_max_angular_speed: Vec2,
    pub min_max_angle: Vec2,
    pub min_max_scale: Vec2,
}
