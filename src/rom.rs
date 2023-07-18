use thiserror::Error;

pub type RomResult<T> = Result<T, RomError>;

#[derive(Debug, Clone, PartialEq)]
pub struct Rom<const N: usize> {
    buffer: [u8; N],
}

impl<const N: usize> Rom<N> {
    pub fn new(buffer: [u8; N]) -> Self {
        Rom {
            buffer,
        }
    }

    pub fn read(&self, index: usize) -> RomResult<u8> {
        if index >= N {
            Err(RomError::IndexOutOfBounds(index, N))
        } else {
            Ok(self.buffer[index])
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum RomError {
    #[error("index {0} out of bounds [0, {1})")]
    IndexOutOfBounds(usize, usize),
}
