use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Asset, TypePath)]
pub struct SizeAsset {
    pub sprite_size: Vec2,
    pub collider_size: Vec2,
}

#[derive(Deserialize, Asset, TypePath)]
pub struct SpawnerAsset {
    pub min_max_speed: Vec2,
    pub min_max_angular_speed: Vec2,
    pub min_max_angle: Vec2,
}
