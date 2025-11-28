/// Read-only memory (ROM) for CPU
pub struct Rom {
    mem: Vec<u8>,
    base_addr: u16, // address where ROM is mapped
}

impl Rom {
    pub fn new(data: &[u8]) -> Self {
        Self {
            mem: data.to_vec(),
            base_addr: 0x8000, // default start address
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        let offset = addr.wrapping_sub(self.base_addr) as usize;
        if offset >= self.mem.len() {
            eprintln!("Warning: read beyond ROM at {:#06X}", addr);
            return 0xFF; // Open bus behavior
        }
        self.mem[offset]
    }
}
