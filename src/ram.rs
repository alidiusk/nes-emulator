use thiserror::Error;

pub type WorkRam = Ram<2048>;
pub type VideoRam = Ram<2048>;

pub type RamResult<T> = Result<T, RamError>;

#[derive(Debug, Clone, PartialEq)]
pub struct Ram<const N: usize> {
    buffer: [u8; N],
}

impl<const N: usize> Ram<N> {
    pub fn new() -> Self {
        Ram {
            buffer: [0; N],
        }
    }

    pub fn read(&self, index: usize) -> RamResult<u8> {
        if index >= N {
            Err(RamError::IndexOutOfBounds(index, N))
        } else {
            Ok(self.buffer[index])
        }
    }

    pub fn write(&mut self, index: usize, value: u8) -> RamResult<()> {
        if index >= N {
            Err(RamError::IndexOutOfBounds(index, N))
        } else {
            self.buffer[index] = value;
            Ok(())
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum RamError {
    #[error("index {0} out of bounds [0, {1})")]
    IndexOutOfBounds(usize, usize),
}
