use super::input::ActionLike;

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
