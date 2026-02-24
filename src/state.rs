#[derive(Debug, Clone, PartialEq)]
pub enum State{
    Idle,
    MovingUp,
    MovingDown,
    DoorsOpen
}