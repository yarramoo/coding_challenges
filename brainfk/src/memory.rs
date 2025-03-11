pub const MEM_SIZE: usize = 30_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Memory {
    pub buf: Box<[usize; MEM_SIZE]>,
    pub cursor: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MemoryError {
    OutOfBoundsWrite(Memory),
    OutOfBoundsRead(Memory),
    OutOfBoundsCursorMove(Memory),
}

impl Memory {
    pub fn new() -> Self {
        Self {
            buf: Box::new([0; MEM_SIZE]),
            cursor: 0,
        }
    }

    pub fn write(&mut self, byte: usize) -> Result<usize, MemoryError> {
        if !self.in_bounds() {
            return Err(MemoryError::OutOfBoundsWrite(self.clone()));
        } 
        let old_byte = self.buf[self.cursor];
        self.buf[self.cursor] = byte;
        Ok(old_byte)
    }

    pub fn read(&self) -> Result<usize, MemoryError> {
        if !self.in_bounds() {
            return Err(MemoryError::OutOfBoundsRead(self.clone()));
        }
        Ok(self.buf[self.cursor])
    }

    pub fn cursor_right(&mut self) -> Result<(), MemoryError> {
        if self.cursor == self.buf.len() - 1 {
            return Err(MemoryError::OutOfBoundsCursorMove(self.clone()));
        }
        self.cursor += 1;
        Ok(())
    }

    pub fn cursor_left(&mut self) -> Result<(), MemoryError> {
        if self.cursor == 0 {
            return Err(MemoryError::OutOfBoundsCursorMove(self.clone()));
        }
        self.cursor -= 1;
        Ok(())
    }

    fn in_bounds(&self) -> bool {
        self.cursor < self.buf.len() 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_write_and_read() {
        let mut memory = Memory::new();
        assert_eq!(memory.write(42), Ok(0)); // Previous value should be 0
        assert_eq!(memory.read(), Ok(42));
    }

    #[test]
    fn test_cursor_movement() {
        let mut memory = Memory::new();
        assert!(memory.cursor_right().is_ok());
        assert_eq!(memory.cursor, 1);
        assert!(memory.cursor_left().is_ok());
        assert_eq!(memory.cursor, 0);
    }

    #[test]
    fn test_cursor_out_of_bounds_right() {
        let mut memory = Memory::new();
        memory.cursor = MEM_SIZE - 1;
        assert!(matches!(memory.cursor_right(), Err(MemoryError::OutOfBoundsCursorMove(_))));
    }

    #[test]
    fn test_cursor_out_of_bounds_left() {
        let mut memory = Memory::new();
        assert!(matches!(memory.cursor_left(), Err(MemoryError::OutOfBoundsCursorMove(_))));
    }

    #[test]
    fn test_write_out_of_bounds() {
        let mut memory = Memory::new();
        memory.cursor = MEM_SIZE; // Set cursor out of bounds
        assert!(matches!(memory.write(42), Err(MemoryError::OutOfBoundsWrite(_))));
    }

    #[test]
    fn test_read_out_of_bounds() {
        let mut memory = Memory::new();
        memory.cursor = MEM_SIZE; // Set cursor out of bounds
        assert!(matches!(memory.read(), Err(MemoryError::OutOfBoundsRead(_))));
    }
}