use bevy::prelude::*;

pub fn remove_resource<R: Resource>(mut commands: Commands) {
    commands.remove_resource::<R>();
}

pub fn despawn_entities_with<C: Component>(mut commands: Commands, query: Query<Entity, With<C>>) {
    query.iter().for_each(|e| {
        commands.entity(e).despawn();
    });
}
