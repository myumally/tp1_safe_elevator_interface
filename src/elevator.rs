use super::state::State;
use std::collections::VecDeque;

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

    pub fn new(f : i32) -> Self {
        Elevator{
            state: State::Idle,
            floor: f,
            queue: VecDeque::new()
        }
    }

    pub fn call(&mut self, f: i32){
        self.queue.push_back(f);
    }

    pub fn step(&mut self){
        if let Some(&mut dest) = self.queue.front_mut(){
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
                self.state = State::DoorsOpen;
            } 
        }
        else{
            self.state = State::Idle;
        }
    }
}

