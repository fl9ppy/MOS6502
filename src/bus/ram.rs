/// RAM memory for CPU
pub struct Ram {
    mem: Vec<u8>,
}

impl Ram {
    pub fn new(size: usize) -> Self {
        Self {
            mem: vec![0; size],
        }
    }

    /// Load data into RAM at specified address
    pub fn load(&mut self, start_addr: u16, data: &[u8]) {
        let start = start_addr as usize;
        let end = start + data.len();
        if end > self.mem.len() {
            panic!("Attempt to load past RAM size");
        }
        self.mem[start..end].copy_from_slice(data);
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.mem[addr as usize] = data;
    }
}
