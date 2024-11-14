use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Asset, TypePath)]
pub struct SizeAsset {
    pub sprite_size: Vec2,
    pub collider_size: Vec2,
}
