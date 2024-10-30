use bevy::prelude::*;

pub fn remove_resource<R: Resource>(mut commands: Commands) {
    commands.remove_resource::<R>();
}

// TODO Use an exclusive world to despawn all at once without commands
pub fn despawn_entities_with<C: Component>(mut commands: Commands, query: Query<Entity, With<C>>) {
    query.iter().for_each(|e| {
        commands.entity(e).despawn();
    });
}
