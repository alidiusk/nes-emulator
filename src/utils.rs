

pub enum TickResult {
    Executing,
    Finished
}

pub trait Tick {
    fn tick(&mut self) -> TickResult;
}
