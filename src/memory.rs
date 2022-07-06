const MEM_BYTES: usize = 4096;

pub struct Memory {
  buffer: Vec<u8>
}

pub fn new_memory() -> Memory {
  Memory { buffer: (0..MEM_BYTES).map(|_| b'0').collect() }
}

impl Memory {
  pub fn write(&mut self, index: u16, val: u8) {
    if let Some(cell) = self.buffer.get_mut(index as usize) {
      *cell = val;
    }
  }

  pub fn read(&self, index: u16) {
    self.buffer.get(index as usize).unwrap_or(&b'0');
  }
}