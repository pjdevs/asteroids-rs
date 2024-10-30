pub mod macros;
pub mod systems;
pub mod window;

pub mod prelude {
    pub use super::systems::{despawn_entities_with, remove_resource};
}
