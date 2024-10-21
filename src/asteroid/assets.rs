use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Asset, TypePath)]
pub struct SizeAsset {
    pub size: Vec2,
}
