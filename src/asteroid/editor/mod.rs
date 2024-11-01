use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[cfg(feature = "dev")]
pub struct AsteroidEditorPlugin;

#[cfg(feature = "dev")]
impl Plugin for AsteroidEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new());
    }
}