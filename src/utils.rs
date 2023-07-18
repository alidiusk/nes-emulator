use crate::interrupt::Interrupt;


pub enum TickResult {
    Executing,
    Finished,
    Interrupt(Interrupt)
}

pub trait Tick {
    fn tick(&mut self) -> TickResult;
}
