#[derive(Debug, PartialEq)]
pub enum ElevatorError{
    InvalidFloor(i32),
    DoorsAlreadyOpen,
    DoorsAlreadyClosed,
    CannotOpenWhileMoving,
    CannotMoveDoorsOpen,
    EmptyQueue
}