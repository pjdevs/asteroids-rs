use super::physics::collision::LayerMask;

pub const PLAYER_MASK: LayerMask = LayerMask(1 << 0);
pub const ENEMY_MASK: LayerMask = LayerMask(1 << 1);
