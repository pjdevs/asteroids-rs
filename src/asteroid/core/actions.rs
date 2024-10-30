use crate::asteroid::input::ActionLike;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
pub enum AsteroidAction {
    #[default]
    TurnLeft,
    TurnRight,
    Forward,
    Backward,
    Shoot,
}

impl ActionLike for AsteroidAction {}
