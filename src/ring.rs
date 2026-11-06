use thiserror::Error;
use std::fmt;

pub struct RingBuffer<'a> {
    buffer: &'a mut [u8], 
    size: usize, 
    push: u8,
    pop: u8
}

#[derive(Debug, Error)]
pub enum RingBufferError {
    #[error("Buffer over capacity")]
    BufferFull, 
    #[error("Buffer supplied not empty")]
    NonZeroBufferInitialized, 
    #[error("Write bytes over capacity")]
    WriteBiggerThanBuffer, 
}

impl<'a> TryFrom<&'a mut [u8]> for RingBuffer<'a> {
    type Error = RingBufferError;

    fn try_from(value: &'a mut [u8]) -> Result<Self, Self::Error> {
        if !value.iter().all(|&byte| byte == 0) {
            return Err(RingBufferError::NonZeroBufferInitialized)
        }
        let size = *&value.len(); 
        Ok(
            RingBuffer {
                buffer: value, 
                size: size, 
                push: 0, 
                pop: 0
            }
        )
    }
}

impl<'a> RingBuffer<'a> {
    pub fn write(self, value: &[u8]) -> Result<(), RingBufferError> {
        if value.len() > self.size && value.len() > (self.size - self.pop as usize + self.push as usize) {
            return Err(RingBufferError::WriteBiggerThanBuffer)
        } else {
            
        }
    }
}

#[test]
fn test_ring_buffer_try_from() {
    let test_slice : &mut [u8] = &mut [0, 0, 0, 0]; 

    let buffer = RingBuffer::try_from(test_slice).unwrap();

    assert_eq!(buffer.pop, 0); 
    assert_eq!(buffer.push, 0);
}

#[test]
fn test_ring_buffer_initializes_empty() {
    let test_slice : &mut [u8] = &mut [0, 0, 0, 1]; 

    let buffer = RingBuffer::try_from(test_slice);

    assert!(matches!(
        buffer, 
        Err(RingBufferError::NonZeroBufferInitialized)
    ));
}
