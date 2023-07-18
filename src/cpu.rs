

pub type CpuError = Result<(), ()>;

pub struct Cpu {

}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {

        }
    }

    pub fn startup(&mut self) -> CpuError {
        Ok(())
    }

    pub fn shutdown(&mut self) -> CpuError {
        Ok(())
    }

    pub fn tick(&mut self) -> CpuError {
        Ok(())
    }

}
