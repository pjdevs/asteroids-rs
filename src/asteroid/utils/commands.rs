use bevy::ecs::system::{EntityCommand, EntityCommands};
use bevy::prelude::DespawnRecursiveExt;

pub struct EnsureDespawned;

impl EntityCommand for EnsureDespawned {
    fn apply(self, id: bevy::prelude::Entity, world: &mut bevy::prelude::World) {
        if let Ok(entity) = world.get_entity_mut(id) {
            entity.despawn_recursive();
        }
    }
}

pub trait EnsureDespawnedEntityCommandsExt {
    fn ensure_despawned(&mut self) -> &mut Self;
}

impl<'a> EnsureDespawnedEntityCommandsExt for EntityCommands<'a> {
    fn ensure_despawned(&mut self) -> &mut Self {
        self.queue(EnsureDespawned)
    }
}
