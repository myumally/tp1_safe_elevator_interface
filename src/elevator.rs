use super::{elevator_error::ElevatorError, state::State};
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct ElevatorSnapshot {
    pub floor: i32,
    pub state: State,
    pub queue: VecDeque<i32>
}

#[derive(Debug, Clone)]
pub struct Elevator{
    floor: i32,
    state: State,
    queue: VecDeque<i32>
}

impl Elevator{
    pub fn default() -> Self {
        Elevator{
            state: State::Idle,
            floor: 0,
            queue: VecDeque::new()
        }
    }

    pub fn new(f : i32) -> Result<Self, ElevatorError> {
        if f < 0 || f > 5 {
            return Err(ElevatorError::InvalidFloor(f));
        }
        Ok(Elevator{
            state: State::Idle,
            floor: f,
            queue: VecDeque::new()
        })
    }

    pub fn state(&self) -> State {
        self.state.clone()
    }

    pub fn floor(&self) -> i32 {
        self.floor
    }

    pub fn queue(&self) -> VecDeque<i32> {
        self.queue.clone()
    }

    pub fn open_doors(&mut self) -> Result<(),ElevatorError> {
        if self.state == State::DoorsOpen { 
            return Err(ElevatorError::DoorsAlreadyOpen);
        }
        else if self.state == State::MovingDown || self.state == State::MovingUp {
            return Err(ElevatorError::CannotOpenWhileMoving)
        }
        else {
            self.state = State::DoorsOpen;
        }
        Ok(())
    }

    pub fn close_doors(&mut self) -> Result<(),ElevatorError> {
        if self.state != State::DoorsOpen { 
            return Err(ElevatorError::DoorsAlreadyClosed);
        }
        if let Some(&dest) = self.queue.front() {
            if self.floor < dest {
                self.state = State::MovingUp;
            } else if self.floor > dest {
                self.state = State::MovingDown;
            } 
        }
        Ok(())
    }

    pub fn call(&mut self, f: i32) -> Result<(),ElevatorError>{
        if f < 0 || f > 5 {
            return Err(ElevatorError::InvalidFloor(f));
        }
        if self.queue.contains(&f) || self.floor == f {
            return Ok(());
        }
        self.queue.push_back(f);
        if let Some(&dest) = self.queue.front() && self.state == State::Idle {
            if self.floor < dest{
                self.state = State::MovingUp;
            } 
            else if self.floor > dest{
                self.state = State::MovingDown;
            }
        }
        Ok(())
        
    }

    pub fn step(&mut self) -> Result<(),ElevatorError>{
        if self.state == State::DoorsOpen {
            return Err(ElevatorError::CannotMoveDoorsOpen)
        }
        if let Some(& dest) = self.queue.front(){
            if self.floor < dest{
                self.floor += 1;
                self.state = State::MovingUp;
            } 
            else if self.floor > dest{
                self.floor -= 1;
                self.state = State::MovingDown;
            }
            else if self.floor == dest{
                self.queue.pop_front();
            } 
        }
        else{
            self.state = State::Idle;
            return Err(ElevatorError::EmptyQueue);
        }
        Ok(())
    }

    pub fn status(&self) -> ElevatorSnapshot {
        ElevatorSnapshot {
            floor: self.floor(),
            state: self.state(),
            queue: self.queue()
        }
    }
}

