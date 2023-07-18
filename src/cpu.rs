

pub type CpuResult = Result<(), ()>;

#[derive(Debug, Clone, PartialEq, Default)]
struct ByteRegister(u8);

#[derive(Debug, Clone, PartialEq, Default)]
struct HalfWordRegister(u16);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Registers {
    pc: HalfWordRegister,
    sp: ByteRegister,
    p: ByteRegister,
    a: ByteRegister,
    x: ByteRegister,
    y: ByteRegister,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::default(),
        }
    }

    pub fn startup(&mut self) -> CpuResult {
        Ok(())
    }

    pub fn shutdown(&mut self) -> CpuResult {
        Ok(())
    }

    pub fn tick(&mut self) -> CpuResult {
        Ok(())
    }

}
